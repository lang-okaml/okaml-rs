#![allow(unused)]
use std::fs;
use regex::Regex;

#[derive(Debug)]
pub enum OkmlType {
    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    SubList(Vec<Okml>),
    Null,
}

impl OkmlType {
    pub fn unwrap_string(&self) -> &String {
        match self {
            OkmlType::String(s) => s,
            _ => panic!("called `unwrap_string` on a non-String value"),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            OkmlType::Boolean(b) => *b,
            _ => panic!("called `unwrap_bool` on a non-Boolean value"),
        }
    }

    pub fn unwrap_integer(&self) -> i64 {
        match self {
            OkmlType::Integer(i) => *i,
            _ => panic!("called `unwrap_integer` on a non-Integer value"),
        }
    }

    pub fn unwrap_float(&self) -> f64 {
        match self {
            OkmlType::Float(f) => *f,
            _ => panic!("called `unwrap_float` on a non-Float value"),
        }
    }

    pub fn unwrap_sublist(&self) -> &Vec<Okml> {
        match self {
            OkmlType::SubList(v) => v,
            _ => panic!("called `unwrap_sublist` on a non-SubList value"),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, OkmlType::Null)
    }
}

#[derive(Debug)]
pub struct Okml {
    key: String,
    value:  OkmlType,    
}

impl Okml {
    pub fn read_from_file(file_path: &str) -> Vec<Okml>{
	let extension = &file_path[file_path.len()-4..file_path.len()];
	
	if extension != "okml" {
	    println!("File is not in .okml {extension}");
	    std::process::exit(-1);
	}
	println!("Okml File Loaded: {}", file_path);
	
	let mut content = fs::read_to_string(file_path)
	.expect("Should've read the file");
	
	let mut processed_content = pre_process(content.clone());
	
	let re = Regex::new(r"[ \t\n]+").unwrap();
	let tokens: Vec<&str> = re.split(&processed_content).filter(|s| !s.is_empty()).collect();
	let (parsed_ast, _) = parse(tokens, 0);
	parsed_ast
    } 

    pub fn read_from_string(content: &str) -> Vec<Okml> {
	let mut processed_content = pre_process(content.to_string());	
	let re = Regex::new(r"[ \t\n]+").unwrap();
	let tokens: Vec<&str> = re.split(&processed_content).filter(|s| !s.is_empty()).collect();
	let (parsed_ast, _) = parse(tokens, 0);
	parsed_ast	
    }

    pub fn key(&self) -> &str{
	let key = &self.key;
	return &key[..key.len()-1]
    }


    pub fn value(&self) -> &OkmlType {
	return &self.value
    }
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

    let mut rc: String = chars.into_iter().collect();
    return rc;
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
	index +=1
    }

    return (parent, index)
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
