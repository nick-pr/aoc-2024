pub fn get_input(file_name: &str) -> String {
    let input_path = format!("input/{}", file_name);
    let input = match std::fs::read_to_string(input_path) {
        Ok(input) => input,
        Err(error) => {
            panic!("Failed to read input for file {}: {}", file_name, error)
        }
    };

    return input;
}
