use crate::error::Result;
use crate::service::all_resources;
use crate::service::file::{cache_file_info, delete_cache_file};
use prettytable::*;

pub fn list() -> Result<()> {
    let mut table = Table::new();
    table.set_titles(row!["name", "modified_at"]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for resource in all_resources() {
        let created_at = match cache_file_info(&*resource) {
            Some(time) => time.to_rfc2822(),
            None => "-".to_owned(),
        };

        table.add_row(row![resource.name(), created_at]);
    }

    table.printstd();
    Ok(())
}

pub fn clear() -> Result<()> {
    let mut deleted = 0;
    for resource in all_resources() {
        deleted += delete_cache_file(&*resource)?;
    }
    println!("{} cache file(s) deleted.", deleted);
    Ok(())
}
