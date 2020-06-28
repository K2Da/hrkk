use crate::error::Error::UnableToWriteFileError;
use crate::error::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use yaml_rust::Yaml;

pub(crate) fn path_str(path: &std::path::PathBuf) -> String {
    path.as_path().display().to_string()
}

const CACHE_DIR: &str = "hrkk";

fn store_file_path(file_name: &str) -> PathBuf {
    let dir = dirs::cache_dir().unwrap().join(CACHE_DIR);
    if !dir.exists() && fs::create_dir(dirs::cache_dir().unwrap().join(CACHE_DIR)).is_err() {
        eprintln!("unable to create cache dir.");
    }
    dirs::cache_dir()
        .unwrap()
        .join(format!("{}/{}", CACHE_DIR, file_name))
}

pub(crate) fn store_response(body: &[u8]) -> Result<()> {
    let path = store_file_path("response_body.txt");
    let mut writer = fs::File::create(&path).or(Err(UnableToWriteFileError(path_str(&path))))?;
    writer
        .write_all(body)
        .or(Err(UnableToWriteFileError(path_str(&path))))?;
    Ok(())
}

pub(crate) fn store_yaml(yaml: &Yaml, file_name: &str) -> Result<()> {
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
