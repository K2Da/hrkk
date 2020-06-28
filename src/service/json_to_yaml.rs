use crate::error::Result;
use inflector::Inflector;
use linked_hash_map::LinkedHashMap;
use serde_json::{from_reader, Value};
use yaml_rust::Yaml;

pub(crate) fn convert(source: &[u8]) -> Result<Yaml> {
    Ok(to_yaml(from_reader(source)?))
}

pub(crate) fn to_yaml(json: Value) -> Yaml {
    match json {
        Value::String(string) => Yaml::String(string),
        Value::Bool(bool) => Yaml::Boolean(bool),
        Value::Null => Yaml::Null,
        Value::Number(number) => {
            if number.is_i64() {
                Yaml::Integer(number.as_i64().unwrap_or(0))
            } else {
                Yaml::Real(number.to_string())
            }
        }
        Value::Array(array) => Yaml::Array(array.iter().map(|v| to_yaml(v.clone())).collect()),
        Value::Object(map) => {
            let mut lhm = LinkedHashMap::new();
            for (key, value) in map {
                lhm.insert(Yaml::String(key.to_snake_case()), to_yaml(value));
            }
            Yaml::Hash(lhm)
        }
    }
}
