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


// This function takes a vector of strings and groups them based on a common prefix,
// ignoring any numbers in the strings.
// The function returns a HashMap where the keys are the common prefixes 
//and the values are vectors of strings that belong to that group.
pub fn group_strings(strings: Vec<String>) -> HashMap<String, Vec<String>> {
    // Initialize an empty HashMap to store the groups.
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over each string in the input vector.
    for string in strings {

        // regex that matches evrything up until the last block of numbers in the string while ignoring a digit only string
        let re = Regex::new(r"(?P<base>.+?)(?:_\d+)?\.\w+$").unwrap();


        let base = if let Some(capture) = re.captures(&string).and_then(|c| c.get(1)) {
            capture.as_str().to_string()
        } else {
            string.clone()
        };

        // remove if base is only numbers
        if base.chars().all(|c| c.is_numeric()) {
                continue;
        }


            // Insert the string into the HashMap, creating a new vector for the group if it doesn't already exist.
            groups.entry(base).or_default().push(string);
    }



    //sort the group in groups by the numeric value part of the string
    /* 
    for (_, group) in groups.iter_mut() {
        group.sort_by(|a, b| {
            let a = a.chars().skip_while(|c| !c.is_numeric()).collect::<String>();
            let b = b.chars().skip_while(|c| !c.is_numeric()).collect::<String>();
            a.cmp(&b)
        });
    }
    */
    for (_, group) in groups.iter_mut() {
        group.sort_by(|a, b| {
            let a_num = a.split('_').last().unwrap().split('.').next().unwrap().parse::<i32>().unwrap();
            let b_num = b.split('_').last().unwrap().split('.').next().unwrap().parse::<i32>().unwrap();
            a_num.cmp(&b_num)
        });
    }



    /* 
    // add an element to the beginning of the vector indicating only the minimum and maximum numeric values of the numeric part of the string
    for (_, group) in groups.iter_mut() {
        let min = group.first().unwrap().chars().skip_while(|c| !c.is_numeric()).collect::<String>();
        let max = group.last().unwrap().chars().skip_while(|c| !c.is_numeric()).collect::<String>();
        // remove non-numeric characters from the min and max values
        let min = min.chars().filter(|c| c.is_numeric()).collect::<String>();
        let max = max.chars().filter(|c| c.is_numeric()).collect::<String>();
        // create a string that lists all the numbers in the group in a sorted order
        let all_numbers = group.iter().map(|s| s.chars().skip_while(|c| !c.is_numeric()).collect::<String>()).collect::<Vec<String>>().join(", ");
        
        //let all_numbers = group.iter().map(|s| s.chars().skip_while(|c| !c.is_numeric()).collect::<String>()).collect::<Vec<String>>().join(", ");

        // sort the numbers in the all_numbers string by their numeric value
        //sort_string(&all_numbers);
  
        // remove non-numeric characters from the all_numbers string
        let all_numbers = all_numbers.chars().filter(|c| c.is_numeric() || c == &',').collect::<String>();

        group.insert(0, format!("range: {:04}-{:04}", min, max));
        //group.insert(0, format!("{}", all_numbers));
    }
    */


    // Return the final HashMap of grouped strings.
    groups
}

