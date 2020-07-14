use crate::error::Error::UnableToWriteFileError;
use crate::error::Result;
use std::fs;
use std::io::Write;
use yaml_rust::Yaml;

pub(crate) fn store_response(body: &[u8]) -> Result<()> {
    let file_name = "response_body.txt".to_string();
    let mut writer =
        fs::File::create(file_name.clone()).or(Err(UnableToWriteFileError(file_name.clone())))?;
    writer
        .write_all(body)
        .or(Err(UnableToWriteFileError(file_name.clone())))?;
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
