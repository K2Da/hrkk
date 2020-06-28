use super::Txt;
use crate::color;
use linked_hash_map::LinkedHashMap;
use tui::style::Color;
use unicode_width::UnicodeWidthStr;
use yaml_rust::Yaml;

type Array = Vec<Yaml>;
type Hash = LinkedHashMap<Yaml, Yaml>;

enum ValueType {
    Real(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Alias(usize),
    Null,
    BadValue,
}

enum YamlType {
    Value(ValueType),
    Hash(Hash),
    Array(Array),
}

fn t(yaml: &Yaml) -> YamlType {
    match yaml {
        Yaml::BadValue => YamlType::Value(ValueType::BadValue),
        Yaml::String(s) => YamlType::Value(ValueType::String(s.to_owned())),
        Yaml::Real(r) => YamlType::Value(ValueType::Real(r.to_owned())),
        Yaml::Integer(i) => YamlType::Value(ValueType::Integer(*i)),
        Yaml::Null => YamlType::Value(ValueType::Null),
        Yaml::Boolean(b) => YamlType::Value(ValueType::Boolean(*b)),
        Yaml::Alias(a) => YamlType::Value(ValueType::Alias(*a)),
        Yaml::Hash(h) => YamlType::Hash(h.clone()),
        Yaml::Array(a) => YamlType::Array(a.clone()),
    }
}

pub(super) struct YamlTexts {
    pub texts: Vec<Txt>,
    pub width: isize,
    pub current_width: isize,
    pub indent: isize,
}

impl YamlTexts {
    pub fn new(width: isize) -> Self {
        YamlTexts {
            texts: vec![],
            width,
            current_width: 0,
            indent: 0,
        }
    }

    pub fn colon(&mut self) {
        self.indent = self.current_width;
        self.colored(": ", color::YAML_OP);
    }

    pub fn hyphen(&mut self) {
        self.indent = self.current_width;
        self.colored("- ", color::YAML_OP);
    }

    pub fn wrap(&mut self, span: &str, str: &str) {
        let header_width = self.current_width;
        for (i, val) in str
            .chars()
            .collect::<Vec<char>>()
            .chunks((self.width - self.current_width - 1) as usize)
            .enumerate()
        {
            if i != 0 {
                self.cr();
                self.raw(span);
                self.raw(&" ".repeat((header_width - span.width_cjk() as isize) as usize));
            }
            self.raw(&val.iter().collect::<String>());
        }
    }

    pub fn raw(&mut self, str: &str) {
        self.current_width += str.width_cjk() as isize;
        self.texts.push(Txt::raw(str))
    }

    pub fn colored(&mut self, str: &str, color: Color) {
        self.current_width += str.width_cjk() as isize;
        self.texts.push(Txt::colored(str, color))
    }

    pub fn cr(&mut self) {
        let remain = self.width - self.current_width;
        if remain > 0 {
            self.raw(&" ".repeat(remain as usize))
        }

        self.raw("│\n");
        self.current_width = 0;
    }

    fn border(&self) -> String {
        "─".repeat((self.width - 2) as usize)
    }
}

pub(super) fn print_with_border(texts: &mut YamlTexts, yaml: &Yaml, span: &str) {
    let border = texts.border();
    texts.raw(&format!("╔{}╗", border));
    print(texts, yaml, span);
    texts.raw(&format!("└{}┘", border));
}

fn print(texts: &mut YamlTexts, yaml: &Yaml, span: &str) {
    match t(yaml) {
        YamlType::Value(vt) => {
            texts.wrap(span, &value(&vt));
            texts.cr();
        }
        YamlType::Hash(h) => print_hash(texts, &h, span),
        YamlType::Array(a) => print_array(texts, &a, span),
    }
}

fn value(vt: &ValueType) -> String {
    match vt {
        ValueType::BadValue | ValueType::Null => "".to_string(),
        ValueType::String(str) => {
            if str.len() == 0 {
                "\"\"".to_string()
            } else {
                str.to_owned()
            }
        }
        ValueType::Real(str) => str.to_owned(),
        ValueType::Integer(i) => format!("{}", i),
        ValueType::Boolean(bool) => format!("{}", bool),
        ValueType::Alias(u) => format!("{}", u),
    }
}

fn print_hash(texts: &mut YamlTexts, hash: &LinkedHashMap<Yaml, Yaml>, span: &str) {
    if hash.is_empty() {
        texts.raw("{}");
        texts.cr();
        return;
    }
    texts.cr();
    for (k, v) in hash {
        texts.raw(span);
        match t(k) {
            YamlType::Value(vt) => texts.colored(&value(&vt), color::YAML_KEY),
            _ => (),
        }

        texts.colon();

        print(texts, v, &format!("{}  ", span));
    }
}

fn print_array(texts: &mut YamlTexts, arr: &Vec<Yaml>, span: &str) {
    if arr.is_empty() {
        texts.raw("[]");
        texts.cr();
        return;
    }
    texts.cr();
    for v in arr {
        texts.raw(span);
        texts.hyphen();
        print(texts, v, &format!("{}  ", span));
    }
}
