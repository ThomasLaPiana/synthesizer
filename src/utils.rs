/// Load a file into a String
pub fn load_file(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(result) => result,
        Err(result) => {
            println!("> Failed to load file with error: {:?}", result);
            std::process::exit(2)
        }
    }
}
