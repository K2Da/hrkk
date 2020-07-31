use yaml_rust::Yaml;

pub(crate) fn apply_path<'a>(yaml: &'a Yaml, path: &[&str]) -> &'a Yaml {
    if path.len() == 0 {
        yaml
    } else {
        apply_path(&yaml[path[0]], &path[1..])
    }
}
