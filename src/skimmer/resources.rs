use crate::error::Result;
use crate::info::BarInfo;
use crate::service::AwsResource;
use skim::prelude::*;
use yaml_rust::Yaml;

pub fn skim(
    resource: &dyn AwsResource,
    yaml_list: &Vec<Yaml>,
    info: &Vec<BarInfo>,
) -> Result<Vec<Arc<dyn SkimItem>>> {
    let (sender, receiver): (SkimItemSender, SkimItemReceiver) = unbounded();

    print_list_items(resource, &yaml_list, info, &sender);

    drop(sender);

    super::skim_run(true, receiver)
}

fn print_list_items(
    resource: &dyn AwsResource,
    yaml_list: &Vec<Yaml>,
    info: &Vec<BarInfo>,
    sender: &Sender<Arc<dyn SkimItem>>,
) {
    let mut resource_list = vec![];
    for yaml in yaml_list {
        resource_list.push((yaml.clone(), resource.line(yaml)));
    }

    let max_list = super::max_lengths(
        &resource_list
            .iter()
            .map(|(_, c)| c.iter().map(|s| s.clone()).collect())
            .collect(),
    );

    for (yaml, cols) in resource_list {
        let (line, color) = super::align_table(&max_list, &cols.iter().map(|s| &s[..]).collect());
        sender
            .send(Arc::new(SkItem {
                color,
                line,
                detail: resource.detail(&yaml),
                output: resource.resource_name(&yaml),
                bar_info: info.clone(),
            }))
            .unwrap();
    }
}

pub struct SkItem {
    color: String,
    line: String,
    detail: String,
    output: String,
    bar_info: Vec<BarInfo>,
}

impl SkimItem for SkItem {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(AnsiString::parse(&self.color.clone()))
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.line)
    }

    fn preview(&self) -> ItemPreview {
        let mut sb = String::new();
        for info in &self.bar_info {
            sb.push_str(&info.display())
        }
        ItemPreview::AnsiText(format!("\n{}\n{}", sb, self.detail))
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.output)
    }
}
