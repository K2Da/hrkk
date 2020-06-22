use crate::error::Error::UnableToWriteFileError;
use crate::error::Result;
use crate::service::AwsResource;
use chrono::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use yaml_rust::Yaml;

pub fn path_str(path: &std::path::PathBuf) -> String {
    path.as_path().display().to_string()
}

const CACHE_DIR: &str = "hrkk";

pub fn store_resource_file_path(resource: &dyn AwsResource) -> PathBuf {
    store_file_path(&format!("{}.yaml", &resource.name()))
}

fn store_file_path(file_name: &str) -> PathBuf {
    let dir = dirs::cache_dir().unwrap().join(CACHE_DIR);
    if !dir.exists() && fs::create_dir(dirs::cache_dir().unwrap().join(CACHE_DIR)).is_err() {
        eprintln!("unable to create cache dir.");
    }
    dirs::cache_dir()
        .unwrap()
        .join(format!("{}/{}", CACHE_DIR, file_name))
}

pub fn store_response(body: &[u8]) -> Result<()> {
    let path = store_file_path("response_body.txt");
    let mut writer = fs::File::create(&path).or(Err(UnableToWriteFileError(path_str(&path))))?;
    writer
        .write_all(body)
        .or(Err(UnableToWriteFileError(path_str(&path))))?;
    Ok(())
}

#[allow(dead_code)]
pub fn store_yaml_list(yaml: &Yaml, resource: &dyn AwsResource) -> Result<()> {
    let path = store_resource_file_path(resource);
    let mut writer = fs::File::create(&path).or(Err(UnableToWriteFileError(path_str(&path))))?;
    let mut out_str = String::new();
    {
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap();
    }
    writer
        .write_all(out_str.as_bytes())
        .or(Err(UnableToWriteFileError(path_str(&path))))?;
    Ok(())
}

#[allow(dead_code)]
pub fn store_yaml(yaml: &Yaml, file_name: &str) -> Result<()> {
    let mut file = fs::File::create(format!("{}.yaml", file_name))
        .or(Err(UnableToWriteFileError(file_name.to_string())))?;
    let mut out_str = String::new();
    {
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap();
    }
    file.write_all(out_str.as_bytes())
        .or(Err(UnableToWriteFileError(file_name.to_string())))?;
    Ok(())
}

#[allow(dead_code)]
pub fn restore_yaml(resource: &dyn AwsResource) -> Option<Vec<Yaml>> {
    let yaml = fs::read_to_string(store_resource_file_path(resource));
    if let Ok(yaml) = yaml {
        if let Ok(docs) = yaml_rust::YamlLoader::load_from_str(&yaml) {
            return match &docs[0] {
                Yaml::Array(arr) => Some(arr.to_vec()),
                _ => None,
            };
        }
    }
    None
}

pub fn cache_file_info(resource: &dyn AwsResource) -> Option<DateTime<Local>> {
    match fs::metadata(store_resource_file_path(resource)) {
        Ok(metadata) => match metadata.modified() {
            Ok(time) => Some(DateTime::from(time)),
            _ => None,
        },
        _ => None,
    }
}

pub fn delete_cache_file(resource: &dyn AwsResource) -> Result<usize> {
    let path = store_resource_file_path(resource);
    if path.exists() {
        fs::remove_file(path).unwrap_or(());
        Ok(1)
    } else {
        Ok(0)
    }
}
