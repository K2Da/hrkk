use std::cmp::max;
use tui::style::Style;
use yaml_rust::Yaml;

pub enum Txt {
    Raw(String),
    Styled(String, Style),
}

impl Txt {
    pub fn raw(data: &str) -> Self {
        Txt::Raw(data.to_string())
    }

    pub fn styled(data: &str, style: Style) -> Txt {
        Txt::Styled(data.to_string(), style)
    }
}

pub struct Texts(pub Vec<Txt>);

impl Texts {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn append(&mut self, other: &mut Texts) {
        self.0.append(&mut other.0);
    }

    pub fn append_colored_pairs(&mut self, pairs: Vec<(&str, Option<Color>)>) {
        for pair in &pairs {
            match pair {
                (data, Some(color)) => self.styled(data, Style::new().fg(*color)),
                (data, None) => self.raw(data),
            }
        }
    }

    pub fn raw(&mut self, data: &str) {
        self.0.push(Txt::raw(data));
    }

    pub fn styled(&mut self, data: &str, style: Style) {
        self.0.push(Txt::styled(data, style));
    }
}

use tui::style::Color;
const BORDER_COLOR: Color = Color::White;
const HEADER_COLOR: Color = Color::Yellow;
const NAME_COLOR: Color = Color::Cyan;
const VALUE_COLOR: Color = Color::White;

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

    pub fn tag_name(mut self, tag: &str, key: &str) -> Self {
        self.name = Some(Name::Yaml(
            crate::service::tag_value(&self.yaml[tag], key).clone(),
        ));
        self
    }

    pub fn yaml_name(mut self, key: &str) -> Self {
        self.name = Some(Name::Yaml(self.yaml[key].clone()));
        self
    }

    pub fn string_name(mut self, name: &str) -> Self {
        self.name = Some(Name::String(name.to_string()));
        self
    }

    pub fn span(self, name: &str, span: (&str, &str)) -> Self {
        let span = self::span(&self.yaml[span.0], &self.yaml[span.1]);
        self.child(name, &span)
    }

    pub fn duration(self, name: &str, duration: (&str, &str)) -> Self {
        let duration = self::duration(&self.yaml[duration.0], &self.yaml[duration.1]);
        self.child(name, &duration)
    }

    pub fn raw(self, name: &str, key: &str) -> Self {
        let raw = self::raw(&self.yaml[key]);
        self.child(name, &raw)
    }

    pub fn raw2(self, name: &str, key: (&str, &str)) -> Self {
        let raw2 = self::raw(&self.yaml[key.0][key.1]);
        self.child(name, &raw2)
    }

    pub fn str(self, name: &str, val: &str) -> Self {
        self.child(name, val)
    }

    pub fn time(self, name: &str, key: &str) -> Self {
        let time = self::time(&self.yaml[key]);
        self.child(name, &time)
    }

    pub fn byte(self, name: &str, key: &str) -> Self {
        let byte = self::byte(&self.yaml[key]);
        self.child(name, &byte)
    }

    fn child(mut self, name: &str, val: &str) -> Self {
        self.children.push(Child::Attribute(Attribute {
            name: Name::String(name.to_string()),
            value: val.to_owned(),
        }));
        self
    }

    pub fn yaml_pairs(mut self, root: &str, key_value: (&str, &str)) -> Self {
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

    pub fn raw_array(mut self, root: &str) -> Self {
        match &self.yaml[root] {
            Yaml::Array(array) => {
                for y in array {
                    self.children.push(Child::Attribute(Attribute {
                        name: Name::String(" ".to_string()),
                        value: self::raw(&y),
                    }));
                }
            }
            _ => (),
        }
        self
    }

    pub fn string_attributes(mut self, attrs: Vec<(String, String)>) -> Self {
        for (name, value) in attrs {
            self.children.push(Child::Attribute(Attribute {
                name: Name::String(name.to_owned()),
                value: value.to_owned(),
            }))
        }
        self
    }

    pub fn section(mut self, section: Section) -> Self {
        self.children.push(Child::Section(section.clone()));
        self
    }

    pub fn print_all(&self, width: isize) -> Texts {
        let mut last_lv = 0;
        let mut last_border = 0;
        self.print(0, &mut last_lv, &mut last_border, width)
    }

    pub fn print(
        &self,
        lv: isize,
        last_lv: &mut isize,
        last_border: &mut isize,
        width: isize,
    ) -> Texts {
        let mut texts = Texts::new();

        let span = if lv > 0 {
            rep("│ ", lv)
        } else {
            "".to_string()
        };

        texts.styled(
            &(span.clone()
                + &lo(lv != 0 && lv <= *last_lv)
                + &Self::header_border(lv, last_lv, last_border, width)
                + &ro(lv != 0)
                + "\n"),
            Style::new().fg(BORDER_COLOR),
        );

        let name = match &self.name {
            Some(name) => name.to_string(),
            None => "".to_string(),
        };

        texts.append_colored_pairs(vec![
            (&span.clone(), Some(BORDER_COLOR)),
            ("║ ", Some(BORDER_COLOR)),
            (&name, Some(HEADER_COLOR)),
            (&rep(" ", width - lv * 2 - name.len() as isize - 3), None),
            ("│\n", Some(BORDER_COLOR)),
        ]);

        let header_width = max(
            self.children
                .iter()
                .map(|c| match c {
                    Child::Section(_) => 0,
                    Child::Attribute(attr) => attr.name.to_string().len(),
                })
                .max()
                .unwrap_or(0) as isize,
            10,
        );

        texts.styled(
            &(span.clone()
                + "╠"
                + &rep("─", header_width + 2)
                + if self.children.len() > 0 {
                    "┬"
                } else {
                    "─"
                }
                + &rep("─", width - lv * 2 - header_width - 5)
                + "╣\n"),
            Style::new().fg(BORDER_COLOR),
        );

        *last_lv = lv;

        for child in &self.children {
            texts.append(&mut child.print(&span, header_width, lv, last_lv, last_border, width));
        }

        if lv == 0 {
            texts.append_colored_pairs(vec![(
                &("└".to_string() + &Self::header_border(lv, last_lv, last_border, width) + "┘"),
                Some(BORDER_COLOR),
            )]);
        }
        texts
    }

    fn header_border(
        lv: isize,
        last_lv: &mut isize,
        last_border: &mut isize,
        width: isize,
    ) -> String {
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
        for i in 0..(max(width - lv * 2 - 2, 0) as isize) {
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
        width: isize,
    ) -> Texts {
        match self {
            Child::Section(section) => section.print(lv + 1, last_lv, last_separator, width),
            Child::Attribute(attribute) => {
                attribute.print(&span, separator, lv, last_separator, width)
            }
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
        width: isize,
    ) -> Texts {
        let name = self.name.to_string();
        let space = " ".to_owned();
        let left_space = if separator >= name.len() as isize {
            rep(&space, separator - name.len() as isize)
        } else {
            space.clone()
        };

        let mut texts = Texts::new();

        let w = width - (separator + (lv * 2) + 7) as isize;

        for (i, val) in self
            .value
            .chars()
            .collect::<Vec<char>>()
            .chunks(w as usize)
            .enumerate()
        {
            let value = val.iter().collect::<String>();
            let right_space = rep(&space, w - value.len() as isize);
            let separator = if i == 0 { "├" } else { "│" };

            texts.append_colored_pairs(vec![
                (span, Some(BORDER_COLOR)),
                ("│ ", Some(BORDER_COLOR)),
                (&left_space, None),
                (
                    &(if i == 0 {
                        name.clone()
                    } else {
                        " ".repeat(name.len())
                    }),
                    Some(NAME_COLOR),
                ),
                (&format!(" {} ", separator), Some(BORDER_COLOR)),
                (&value, Some(VALUE_COLOR)),
                (&right_space, Some(BORDER_COLOR)),
                (" │\n", Some(BORDER_COLOR)),
            ]);
        }
        *last_separator = lv * 2 + separator as isize + 3;
        texts
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
