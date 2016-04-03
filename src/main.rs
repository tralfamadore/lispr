use std::env;
use std::fs::File;
use std::io::Read;
use std::str::SplitWhitespace;

struct Node<'a> {
    value: &'a str,
    children: Option<Vec<Node<'a>>>,
}

fn main() {
    let path: String = get_path_to_src();

    let mut file_content = read_file_to_string(& path);
    file_content = file_content.replace("(", " ( ");
    file_content = file_content.replace(")", " ) ");

    let mut tokens = file_content.split_whitespace();

    let root = parse(&mut tokens);
}

fn parse<'a>(tokens: &mut SplitWhitespace<'a>) -> Option<Node<'a>> {
    let mut current = tokens.next().unwrap();
    let mut has_children = false;

    if current == ")" {
        return None;
    } else if current == "(" {
        has_children = true;
        current = tokens.next().unwrap();
    }

    if has_children {
        let mut items = vec![];
        let mut child = parse(tokens);
        while child.is_some() {
            let node = child.unwrap();
            items.push(node);
            child = parse(tokens);
        }
        return Some(Node { value: current, children: Some(items) } );
    }

    Some(Node { value: current, children: None } )
}

fn read_file_to_string(path: & str) -> String {
    let mut file = match File::open(& path) {
        Err(_) => panic!("couldn't open {}", path),
        Ok(file) => file,
    };

    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Err(_) => panic!("couldn't read {}", path),
        Ok(_) => println!("\"{}\" contains:\n{}", path, result),
    };

    result
}

fn get_path_to_src() -> String {
    match env::args().nth(1) {
        Some(arg) => arg,
        None => String::from("../test"),
    }
}
