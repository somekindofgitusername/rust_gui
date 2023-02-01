use regex::Regex;
use std::collections::HashMap;



// a function that reads a directory and returns a vector of strings with the file names
pub fn read_dir(dir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        files.push(file_name);
    }
    files
}

// a function that takes a vector of strings and returns only those strings that match at least one regex from a vector of regexes
pub fn filter_files(files: Vec<String>, regexes: Vec<Regex>) -> Vec<String> {
    let mut filtered_files: Vec<String> = Vec::new();
    for file in files {
        for regex in &regexes {
            if regex.is_match(&file) {
                filtered_files.push(file);
                break;
            }
        }
    }
    filtered_files
}



// This function takes a vector of strings and groups them based on a common prefix, ignoring any numbers in the strings.
// The function returns a HashMap where the keys are the common prefixes and the values are vectors of strings that belong to that group.
pub fn group_strings(strings: Vec<String>) -> HashMap<String, Vec<String>> {
    // Initialize an empty HashMap to store the groups.
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over each string in the input vector.
    for string in strings {
        // Extract the base prefix of the string by taking the characters up until the first numeric character.
        let base = string.chars().take_while(|c| !c.is_numeric()).collect::<String>();

        // Insert the string into the HashMap, creating a new vector for the group if it doesn't already exist.
        groups.entry(base).or_default().push(string);
    }

    // Return the final HashMap of grouped strings.
    groups
}
