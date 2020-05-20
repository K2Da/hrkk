use crate::show;
use crate::skimmer::preview_width as width;
use ansi_term::Color;
use std::cmp::max;
use yaml_rust::Yaml;

const BORDER_COLOUR: Color = Color::Fixed(244);
const HEADER_COLOUR: Color = Color::Yellow;
const NAME_COLOUR: Color = Color::Cyan;
const VALUE_COLOUR: Color = Color::White;
const BAD_COLOUR: Color = Color::Red;

#[derive(Debug, Clone)]
pub struct Section {
    yaml: Yaml,
    name: Option<Name>,
    children: Vec<Child>,
}

impl Section {
    pub fn new(yaml: &Yaml) -> Self {
        Self {
            yaml: yaml.clone(),
            name: None,
            children: vec![],
        }
    }

    pub fn new_without_yaml() -> Self {
        Self {
            yaml: Yaml::BadValue,
            name: None,
            children: vec![],
        }
    }

    pub fn tag_name(&mut self, tag: &str, key: &str) -> &mut Self {
        self.name = Some(Name::Yaml(
            crate::service::tag_value(&self.yaml[tag], key).clone(),
        ));
        self
    }

    pub fn yaml_name(&mut self, key: &str) -> &mut Self {
        self.name = Some(Name::Yaml(self.yaml[key].clone()));
        self
    }

    pub fn string_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(Name::String(name.to_string()));
        self
    }

    pub fn span(&mut self, name: &str, span: (&str, &str)) -> &mut Self {
        self.child(name, &show::span(&self.yaml[span.0], &self.yaml[span.1]))
    }

    pub fn duration(&mut self, name: &str, duration: (&str, &str)) -> &mut Self {
        self.child(
            name,
            &show::duration(&self.yaml[duration.0], &self.yaml[duration.1]),
        )
    }

    pub fn raw(&mut self, name: &str, key: &str) -> &mut Self {
        self.child(name, &show::raw(&self.yaml[key]))
    }

    pub fn raw2(&mut self, name: &str, key: (&str, &str)) -> &mut Self {
        self.child(name, &show::raw(&self.yaml[key.0][key.1]))
    }

    pub fn str(&mut self, name: &str, val: &str) -> &mut Self {
        self.child(name, val)
    }

    pub fn time(&mut self, name: &str, key: &str) -> &mut Self {
        self.child(name, &show::time(&self.yaml[key]))
    }

    pub fn byte(&mut self, name: &str, key: &str) -> &mut Self {
        self.child(name, &show::byte(&self.yaml[key]))
    }

    fn child(&mut self, name: &str, val: &str) -> &mut Self {
        self.children.push(Child::Attribute(Attribute {
            name: Name::String(name.to_string()),
            value: val.to_owned(),
        }));
        self
    }

    pub fn yaml_pairs(&mut self, root: &str, key_value: (&str, &str)) -> &mut Self {
        match &self.yaml[root] {
            Yaml::Array(array) => {
                for y in array {
                    let (name, value) = (y[key_value.0].clone(), y[key_value.1].clone());
                    self.children.push(Child::Attribute(Attribute {
                        name: Name::Yaml(name.clone()),
                        value: raw(&value.clone()),
                    }));
                }
            }
            _ => (),
        }
        self
    }

    pub fn raw_array(&mut self, root: &str) -> &mut Self {
        match &self.yaml[root] {
            Yaml::Array(array) => {
                for y in array {
                    self.children.push(Child::Attribute(Attribute {
                        name: Name::String(" ".to_string()),
                        value: show::raw(&y),
                    }));
                }
            }
            _ => (),
        }
        self
    }

    pub fn string_attributes(&mut self, attrs: Vec<(String, String)>) -> &mut Self {
        for (name, value) in attrs {
            self.children.push(Child::Attribute(Attribute {
                name: Name::String(name.to_owned()),
                value: value.to_owned(),
            }))
        }
        self
    }

    pub fn section(&mut self, section: &Section) -> &mut Self {
        self.children.push(Child::Section(section.clone()));
        self
    }

    pub fn print_all(&self) -> String {
        let mut last_lv = 0;
        let mut last_border = 0;
        self.print(0, &mut last_lv, &mut last_border)
    }

    pub fn print(&self, lv: isize, last_lv: &mut isize, last_border: &mut isize) -> String {
        let mut sb = String::new();

        let span = if lv > 0 {
            rep("│ ", lv)
        } else {
            "".to_string()
        };

        sb.push_str(&format!(
            "{}\n",
            BORDER_COLOUR.paint(
                span.clone()
                    + &lo(lv != 0 && lv <= *last_lv)
                    + &Self::header_border(lv, last_lv, last_border)
                    + &ro(lv != 0)
            )
        ));

        sb.push_str(&match &self.name {
            Some(name) => format!(
                "{span}{border} {title}{back_span}{right}\n",
                span = BORDER_COLOUR.paint(span.clone()),
                border = BORDER_COLOUR.paint("║"),
                title = HEADER_COLOUR.bold().paint(name.to_string()),
                back_span = rep(" ", width() - lv * 2 - name.to_string().len() as isize - 3),
                right = BORDER_COLOUR.paint("│"),
            ),
            None => format!("{}", BAD_COLOUR.paint("None")),
        });

        let mut header_width = self
            .children
            .iter()
            .map(|c| match c {
                Child::Section(_) => 0,
                Child::Attribute(attr) => attr.name.to_string().len(),
            })
            .max()
            .unwrap_or(0) as isize;
        header_width = max(10, header_width);

        let border = "╠".to_string()
            + &rep("─", header_width + 2)
            + "┬"
            + &rep("─", width() - lv * 2 - header_width - 5)
            + "╣";
        sb.push_str(&format!(
            "{span}{title}\n",
            span = BORDER_COLOUR.paint(span.clone()),
            title = BORDER_COLOUR.paint(&border)
        ));

        *last_lv = lv;

        for child in &self.children {
            sb.push_str(&child.print(&span, header_width, lv, last_lv, last_border));
        }

        if lv == 0 {
            sb.push_str(&format!(
                "{}",
                BORDER_COLOUR
                    .paint("└".to_string() + &Self::header_border(lv, last_lv, last_border) + "┘")
            ));
        }
        sb
    }

    fn header_border(lv: isize, last_lv: &mut isize, last_border: &mut isize) -> String {
        let mut tops = vec![];
        if *last_lv > lv {
            for i in 0..(*last_lv - lv) {
                tops.push(i * 2 + 1);
            }
        }

        if *last_border - 1 > lv * 2 {
            tops.push(*last_border - 1 - lv * 2);
        }
        let mut bar = String::new();
        for i in 0..(max(crate::skimmer::preview_width() - lv * 2 - 2, 0) as isize) {
            bar.push_str(&m(tops.contains(&i)));
        }
        bar
    }
}

#[derive(Debug, Clone)]
pub enum Child {
    Section(Section),
    Attribute(Attribute),
}

impl Child {
    pub fn print(
        &self,
        span: &str,
        separator: isize,
        lv: isize,
        last_lv: &mut isize,
        last_separator: &mut isize,
    ) -> String {
        match self {
            Child::Section(section) => section.print(lv + 1, last_lv, last_separator),
            Child::Attribute(attribute) => attribute.print(&span, separator, lv, last_separator),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: Name,
    value: String,
}

impl Attribute {
    pub fn print(
        &self,
        span: &str,
        separator: isize,
        lv: isize,
        last_separator: &mut isize,
    ) -> String {
        let name = self.name.to_string();
        let space = " ".to_owned();
        let left_space = if separator >= name.len() as isize {
            rep(&space, separator - name.len() as isize)
        } else {
            space.clone()
        };

        let mut sb = String::new();
        let width = width() - (separator + (lv * 2) + 7) as isize;

        for (i, val) in self
            .value
            .chars()
            .collect::<Vec<char>>()
            .chunks(width as usize)
            .enumerate()
        {
            let value = val.iter().collect::<String>();
            let right_space = rep(&space, width - value.len() as isize);
            let separator = if i == 0 { "├" } else { "│" };

            sb.push_str(&format!(
                "{span}{border} {left_space}{name} {separator} {value} {right_space}{border}\n",
                span = BORDER_COLOUR.paint(span),
                border = BORDER_COLOUR.paint("│"),
                left_space = left_space,
                name = NAME_COLOUR.paint(if i == 0 {
                    name.clone()
                } else {
                    " ".repeat(name.len())
                }),
                separator = BORDER_COLOUR.paint(separator),
                value = VALUE_COLOUR.paint(value),
                right_space = BORDER_COLOUR.paint(right_space),
            ));
        }
        *last_separator = lv * 2 + separator as isize + 3;
        sb
    }
}

pub fn raw(yaml: &Yaml) -> String {
    match yaml {
        Yaml::String(string) => string.clone(),
        Yaml::Integer(int) => format!("{}", int),
        _ => "None".to_string(),
    }
}

pub fn byte(yaml: &Yaml) -> String {
    match yaml {
        Yaml::Integer(byte) => {
            if *byte < 1024 {
                format!("{} bytes", byte)
            } else if *byte < 1024_i64.pow(2) {
                format!("{:.2} KiB", *byte as f64 / 1024_f64)
            } else if *byte < 1024_i64.pow(3) {
                format!("{:.2} MiB", *byte as f64 / 1024_f64.powf(2.0))
            } else {
                format!("{:.2} GiB", *byte as f64 / 1024_f64.powf(3.0))
            }
        }
        _ => "?".to_string(),
    }
}

pub fn span(from: &Yaml, to: &Yaml) -> String {
    let time_from = to_datetime(from);
    let time_to = to_datetime(to);

    let str_to = match time_to {
        Some(time_to) => match time_from {
            Some(time_from) => trimmed_time(time_to, time_from),
            None => time(to),
        },
        None => "-".to_string(),
    };

    let duration = match get_duration(from, to) {
        Some(duration) => format!(" ({})", duration),
        None => "".to_string(),
    };
    format!("{} to {}{}", time(from), str_to, duration)
}

pub fn duration(from: &Yaml, to: &Yaml) -> String {
    get_duration(from, to).unwrap_or("-".to_string())
}

fn get_duration(from: &Yaml, to: &Yaml) -> Option<String> {
    use humantime::format_duration;

    let time_from = to_datetime(from);
    let time_to = to_datetime(to);

    if let Some(time_to) = time_to {
        if let Some(time_from) = time_from {
            return Some(
                format_duration(std::time::Duration::new(
                    time_to.signed_duration_since(time_from).num_seconds() as u64,
                    0,
                ))
                .to_string(),
            );
        }
    }
    None
}

pub fn time(yaml: &Yaml) -> String {
    match to_datetime(yaml) {
        Some(dt) => trimmed_time(dt, Local::now()),
        None => "-".to_string(),
    }
}

fn trimmed_time(target: DateTime<Local>, base: DateTime<Local>) -> String {
    if target.date() == base.date() {
        target.format("%X").to_string()
    } else if target.year() == base.year() {
        target.format("%a, %d %b %X").to_string()
    } else {
        target.format("%a, %d %b %Y %X").to_string()
    }
}

use chrono::prelude::*;
fn to_datetime(yaml: &Yaml) -> Option<DateTime<Local>> {
    match yaml {
        Yaml::Integer(time) => Some(Local.timestamp(*time / 1_000, 0)),
        Yaml::Real(time) => {
            let t: f64 = time.parse().unwrap();
            Some(Local.timestamp(t as i64, 0))
        }
        Yaml::String(time) => {
            if let Ok(time) = DateTime::parse_from_rfc2822(time) {
                Some(time.into())
            } else if let Ok(time) = DateTime::parse_from_rfc3339(time) {
                Some(time.into())
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    Yaml(Yaml),
    String(String),
}

use std::fmt;

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Yaml(yaml) => match yaml {
                Yaml::String(string) => write!(f, "{}", string)?,
                _ => (),
            },
            Name::String(string) => write!(f, "{}", string)?,
        }
        Ok(())
    }
}

fn lo(line: bool) -> String {
    if line { "╠" } else { "╔" }.to_owned()
}

fn ro(line: bool) -> String {
    if line { "╣" } else { "╗" }.to_owned()
}

fn m(line: bool) -> String {
    if line { "┴" } else { "─" }.to_owned()
}

pub fn rep(s: &str, n: isize) -> String {
    s.repeat(max(n, 0) as usize)
}
