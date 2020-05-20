use crate::error::Result;
use crate::opts::Opts;
use crate::service::file::{path_str, store_resource_file_path};
use crate::service::AwsResource;
use crate::show;
use crate::skimmer::preview_width;
use skim::prelude::*;

pub fn skim(resources: Vec<Box<dyn AwsResource>>, opts: &Opts) -> Result<Vec<Arc<dyn SkimItem>>> {
    let (sender, receiver): (SkimItemSender, SkimItemReceiver) = unbounded();

    print_list_items(resources, opts, &sender);

    drop(sender);

    super::skim_run(false, receiver)
}

fn print_list_items(
    resources: Vec<Box<dyn AwsResource>>,
    opts: &Opts,
    sender: &Sender<Arc<dyn SkimItem>>,
) {
    let mut list: Vec<Vec<String>> = vec![];

    for resource in &resources {
        list.push(vec![resource.service_name(), resource.resource_type_name()]);
    }

    let max_list = super::max_lengths(&list);

    for resource in resources {
        let (line, color) = super::align_table(
            &max_list,
            &vec![&resource.service_name(), &resource.command_name()],
        );

        sender
            .send(Arc::new(SkItem {
                color,
                line,
                output: resource.name().clone(),
                resource,
                opt_str: opts.colored_string(),
            }))
            .unwrap();
    }
}

pub struct SkItem {
    color: String,
    line: String,
    output: String,
    resource: Box<dyn AwsResource>,
    opt_str: crate::color::ColoredString,
}

impl SkimItem for SkItem {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(AnsiString::parse(&self.color.clone()))
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.line)
    }

    fn preview(&self) -> ItemPreview {
        let ansi_text = show::Section::new_without_yaml()
            .string_name(&format!(
                "{} {}",
                self.resource.service_name(),
                self.resource.command_name()
            ))
            .str("document", &self.resource.info().document_url)
            .str(
                "cache path",
                &path_str(&store_resource_file_path(&*self.resource)),
            )
            .str(
                "result limit",
                &format!("{}", self.resource.info().max_limit),
            )
            .print_all();

        let left = " Select command";
        let mid = show::rep(
            " ",
            preview_width() - left.len() as isize - self.opt_str.len() as isize,
        );
        ItemPreview::AnsiText(format!("\n{}{}{}\n{}", left, mid, self.opt_str, ansi_text))
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.output)
    }
}
