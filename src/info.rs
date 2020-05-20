use crate::color::ColoredString;
use crate::error::Result;
use crate::opts::Opts;
use crate::skimmer::preview_width;
use ansi_term::Color;
use ansi_term::Style;
use rusoto_core::Region;

const OK_BG: Color = Color::Fixed(64);
const NG_BG: Color = Color::Fixed(160);
const CACHE_BG: Color = Color::Fixed(208);
const RESOURCE_BG: Color = Color::Fixed(255);
const RESOURCE_FG: Color = Color::Fixed(0);

#[derive(Debug, Clone)]
pub enum BarInfo {
    Fetch(FetchInfo),
    Cache(CacheInfo),
    Resource(ResourceInfo),
}

impl BarInfo {
    pub fn display(&self) -> String {
        match self {
            BarInfo::Resource(info) => info.display(),
            BarInfo::Fetch(info) => info.display(),
            BarInfo::Cache(info) => info.display(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FetchInfo {
    pub request_count: usize,
    pub resource_count: usize,
    pub fetch_all: bool,
    pub region: Region,
    pub opt_str: crate::color::ColoredString,
}

impl FetchInfo {
    pub fn new(opts: &Opts) -> Result<Self> {
        Ok(FetchInfo {
            region: opts.region()?,
            opt_str: opts.colored_string(),
            ..Default::default()
        })
    }

    fn display(&self) -> String {
        let mut status = if self.fetch_all {
            ColoredString::new(" Fetched All ", None, Some(OK_BG))
        } else {
            ColoredString::new(" Not Fetched All ", None, Some(NG_BG))
        };

        status.push_str(ColoredString::no_style(&format!(
            " (requests:{} / fetched:{} @{})",
            self.request_count,
            self.resource_count,
            self.region.name(),
        )));

        let mid = crate::show::rep(
            " ",
            preview_width() - status.len() as isize - self.opt_str.len(),
        );
        format!("{}{}{}", status, mid, self.opt_str)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CacheInfo {
    pub opt_str: crate::color::ColoredString,
}

impl CacheInfo {
    fn display(&self) -> String {
        let left = ColoredString::new(" Using Cache ", None, Some(CACHE_BG));
        let mid = crate::show::rep(" ", preview_width() - left.len() - self.opt_str.len());
        format!("{}{}{}", left, mid, self.opt_str)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResourceInfo {
    pub service_name: String,
    pub command_name: String,
}

impl ResourceInfo {
    fn display(&self) -> String {
        let text = format!("{} {}", self.service_name, self.command_name);
        Style::new()
            .on(RESOURCE_BG)
            .fg(RESOURCE_FG)
            .paint(format!(
                " {:<max$}\n",
                text,
                max = crate::skimmer::preview_width() as usize
            ))
            .to_string()
    }
}
