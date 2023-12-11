use std::fs::read_to_string;

fn read_lines_to_vector(file_name: &str) -> Vec<String> {
    
    read_to_string(file_name)
    .unwrap() 
    .lines() 
    .map(String::from) 
    .collect() 
}