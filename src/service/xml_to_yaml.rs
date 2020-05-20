use crate::error::Error;
use crate::error::Result;
use inflector::Inflector;
use linked_hash_map::LinkedHashMap;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;
use yaml_rust::Yaml;

pub fn convert(source: &[u8], iteration_tag: &Vec<&str>) -> Result<Yaml> {
    let reader = EventReader::new_with_config(source, ParserConfig::new().trim_whitespace(false));
    let elements = to_elements(reader)?;
    let (yaml, _) = convert_element(&elements, 0, iteration_tag)?;

    Ok(yaml)
}

fn convert_element(
    elements: &Vec<Element>,
    index: usize,
    iteration_tag: &Vec<&str>,
) -> Result<(Yaml, usize)> {
    match elements.get(index + 1) {
        Some(Element::StartTag(name)) => {
            // grep "ojb.push" in generated.rs
            if iteration_tag.contains(&&name[..]) {
                // if name == "item" || name == "member" || name == "DBInstance" || name == "Subnet" {
                convert_array(elements, index, iteration_tag)
            } else {
                convert_hash(elements, index, iteration_tag)
            }
        }
        Some(Element::Value(string)) => Ok((Yaml::String(string.to_owned()), 3)),
        Some(Element::EndTag(_)) => Ok((Yaml::String("".to_string()), 2)),
        _ => Err(Error::XmlError),
    }
}

fn convert_array(
    elements: &Vec<Element>,
    index: usize,
    iteration_tag: &Vec<&str>,
) -> Result<(Yaml, usize)> {
    let mut consumed = 1;
    let mut arr = vec![];

    loop {
        match elements.get(index + consumed) {
            Some(Element::StartTag(_)) => {
                let (yaml, forward) = convert_element(elements, index + consumed, iteration_tag)?;
                consumed += forward;
                arr.push(yaml);
            }
            Some(Element::EndTag(_)) => {
                consumed += 1;
                break;
            }
            _ => return Err(Error::XmlError),
        }
    }

    Ok((Yaml::Array(arr), consumed))
}

fn convert_hash(
    elements: &Vec<Element>,
    index: usize,
    iteration_tag: &Vec<&str>,
) -> Result<(Yaml, usize)> {
    let mut consumed = 1;
    let mut map = LinkedHashMap::new();

    loop {
        match elements.get(index + consumed) {
            Some(Element::StartTag(name)) => {
                let (yaml, forward) = convert_element(elements, index + consumed, iteration_tag)?;
                consumed += forward;
                let key = Yaml::String(name.to_snake_case());
                if map.contains_key(&key) {
                    return Err(Error::DuplicatedXmlTag(name.to_owned()));
                }
                map.insert(key, yaml);
            }
            Some(Element::EndTag(_)) => {
                consumed += 1;
                break;
            }
            _ => return Err(Error::XmlError),
        }
    }

    Ok((Yaml::Hash(map), consumed))
}

#[derive(Debug)]
enum Element {
    StartTag(String),
    EndTag(String),
    Value(String),
}

fn to_elements(reader: EventReader<&[u8]>) -> Result<Vec<Element>> {
    let mut e = vec![];
    for event in reader {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => e.push(Element::StartTag(name.local_name)),
            Ok(XmlEvent::Characters(string)) => e.push(Element::Value(string)),
            Ok(XmlEvent::EndElement { name }) => e.push(Element::EndTag(name.local_name)),
            Ok(XmlEvent::EndDocument {}) => break,
            Ok(_) => (),
            Err(_) => Err(Error::XmlError)?,
        }
    }
    Ok(e)
}
