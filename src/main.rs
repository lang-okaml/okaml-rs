#![allow(unused)]
use log::{debug, error, info, trace, warn};
use std::fs;
use regex::Regex;

#[derive(Debug)]
enum OkmlType {
    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    SubList(Vec<Okml>),
    Null,
}

#[derive(Debug)]
struct Okml {
    key: String,
    value:  OkmlType,    
}

fn load_from_file(file_path: &str) {
    let extension = &file_path[file_path.len()-4..file_path.len()];
    
    if extension != "okml" {
	error!("File is not in .okml {extension}");
	std::process::exit(-1);
    }
    info!("File: {}", file_path);

    let content = fs::read_to_string(file_path)
	.expect("Should've read the file");

    let re = Regex::new(r"[ \t\n]+").unwrap();
    let tokens: Vec<&str> = re.split(&content).filter(|s| !s.is_empty()).collect();
    let token_len = tokens.len();

    let mut parent: Vec<Okml> = Default::default();
    let count_braces: u64 = 0;
    
    for (mut index) in 0..token_len-1 {
	let current = tokens.get(index).unwrap();
	let mut value = tokens.get(index+1).unwrap();
	// parse (sub_list_key { )
	if value == &"{" {
	    info!("sublist found at {{{current}}}");
	    // parse_sublist(tokens)
	} 

	
	// parse (key: value) 
	if let Some(':') = current.chars().last() {
	    if value == &"```" {
		let mut _current: &str;
		let mut result: String = Default::default();
		while index < token_len{
		    index = index+1;
		    _current = tokens.get(index+1).unwrap();
		    if _current == "```" {
			break;
		    }
		    result.push_str(_current);
		    result.push_str(" ");
		} 

		parent.push(Okml{
		    key: current.to_string(),
		    value: OkmlType::String(result.clone())
		});
		info!("{} => {}", current, result);
	    } else {

		let mut return_value: OkmlType;
		match value {
		    &"true" | &"yes" => return_value = OkmlType::Boolean(true),
		    &"false" | &"no" => return_value = OkmlType::Boolean(false),
		    &"null" => return_value = OkmlType::Null,
		    &&_ => {
			if let Ok(parsed) = value.parse::<i64>() {
			    return_value = OkmlType::Integer(parsed);
			} else if let Ok(parsed) = value.parse::<f64>() {
			    return_value = OkmlType::Float(parsed);
			} else {
			    return_value = OkmlType::String(value.to_string());
			}
		    }			
		}
		// match value
		parent.push(Okml{
		    key: current.to_string(),
		    value: (return_value)
		});
		info!("{} => {}", current, value);
	    }
	} else {
	    // info!("{}", current);
	}
    }

    // println!("{:#?}", parent);
    
} 

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    load_from_file(file_path);
}
