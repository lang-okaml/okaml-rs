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

fn parse_list(tokens: Vec<&str>, mut index: usize) -> (Vec<Okml>, usize) {
    let mut parent: Vec<Okml> = Default::default();
    index = index +2;
    let mut current = tokens.get(index).unwrap();
    if current  == &"}" {
	return (parent, index)
    }
    let mut value = tokens.get(index+1).unwrap();
    while current != &"}" {
	current = tokens.get(index).unwrap();
	value = tokens.get(index+1).unwrap();
	if current == &"}" || value == &"}" {
	    break;
	}

	if value == &"{" {
	    let mut new_index: usize = 0;
	    let mut child: Vec<Okml> = Default::default();
	    (child, new_index) = parse_list(tokens.clone(), index);
	    parent.push(Okml {
		key: current.to_string(),
		value: OkmlType::SubList(child)
	    });
	    index = new_index-1;
	} else {
	    // println!("{}  => {} ", current, value);
	    if value == &"```" {
		let mut _current: &str;
		let mut result: String = Default::default();
		while index < tokens.len(){
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
		// info!("{} => {}", current, result);
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
	    }
	}
	index = index+2;
    } 

    
    (parent, index)
}

fn parse(tokens: Vec<&str>, index: usize) -> (Vec<Okml>, usize) {
    let token_len = tokens.len();

    let mut parent: Vec<Okml> = Default::default();
    let count_braces: u64 = 0;
    let mut index = 0;
    while index < token_len-1 {
	let current = tokens.get(index).unwrap();
	let mut value = tokens.get(index+1).unwrap();
	// parse (sub_list_key { )
	if value == &"{" {
	    let mut child: Vec<Okml> = Default::default();
	    let mut new_index = 0;
	    (child, new_index) = parse_list(tokens.clone(), index);
	    let mut child_to_push: Okml = Okml {
		key: current.to_string(),
		value: OkmlType::SubList(child)
	    };
	    parent.push(child_to_push);
	    index = new_index-1;
	    continue;
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
	    info!("{}", current);
	}
	index +=1
    }

    return (parent, index)
}

fn pre_process(mut content: String) -> String{
   let mut chars: Vec<char> = content.chars().collect();
   let mut result = Vec::with_capacity(chars.len());
   let mut next_char: Option<&char>;
    for (index, &current_char) in chars.iter().enumerate() {
	next_char = chars.get(index+1);
	if current_char == ' ' && next_char ==  Some(&':') {
	    continue;
	}
	
	if current_char == ':'
            && next_char != Some(&' ')
            && chars.get(index.wrapping_sub(1)) != Some(&'\\')
	{
	    result.push(current_char);
            result.push(' ');
	} else if current_char == '{' && chars.get(index-1) != Some(&' '){
	    result.push(' ');
	    result.push(current_char);
	} else if current_char == '`' && (next_char != Some(&'`')) {
		result.push(current_char);
		result.push('\n');
	} else if current_char == '`' && (chars.get(index-1) != Some(&'\n')) {
	    // result.push('\n');
	    result.push(current_char);
	}
	else {
	    result.push(current_char);
	}
    }
    
    chars = result;


    // for c in chars {
    // 	print!("{c}", );
    // }

    let mut rc: String = chars.into_iter().collect();
    return rc;
}

fn load_from_file(file_path: &str) {
    let extension = &file_path[file_path.len()-4..file_path.len()];
    
    if extension != "okml" {
	error!("File is not in .okml {extension}");
	std::process::exit(-1);
    }
    info!("File: {}", file_path);

    let mut content = fs::read_to_string(file_path)
	.expect("Should've read the file");

    let mut processed_content = pre_process(content.clone());

    let re = Regex::new(r"[ \t\n]+").unwrap();
    let tokens: Vec<&str> = re.split(&processed_content).filter(|s| !s.is_empty()).collect();
    let (parsed_ast, _) = parse(tokens, 0);
    println!("{:#?}", parsed_ast);
} 

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    load_from_file(file_path);
}
