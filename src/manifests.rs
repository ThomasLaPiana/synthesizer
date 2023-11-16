use crate::models;

pub fn parse_manifest_file(contents: String) -> models::Pipelines {
    let roxfile_result = serde_yaml::from_str(&contents);

    match roxfile_result {
        Ok(result) => result,
        Err(_) => {
            println!("Failed to parse the file into YAML!");
            std::process::exit(2)
        }
    }
}
