
extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;

trait Node {
    fn pull(&self) -> Vec<String>;
}

struct StandardIn {}

impl Node for StandardIn {
    fn pull(&self) -> Vec<String> {
        return vec!["abcdef".to_string(),
                    "ghijk".to_string(),
                    "asdfabcasdfasd".to_string()];
    }
}


struct StandardOut<'a> {
    input: Option<&'a Node>,
}

impl<'a> Node for StandardOut<'a> {
    fn pull(&self) -> Vec<String> {
        match self.input {
            None => return vec![],
            Some(input) => {
                let content = input.pull();
                println!("{:?}", content);
                return vec![];
            }
        }
    }
}

struct StringMatch<'a> {
    input: Option<&'a Node>,
    value: String,
}

impl<'a> Node for StringMatch<'a> {
    fn pull(&self) -> Vec<String> {
        match self.input {
            None => return vec![],
            Some(input) => {
                let content = input.pull();
                let mut output = vec![];
                for i in &content {
                    if i.contains(self.value.as_str()) {
                        output.push(i.to_string());
                    }
                }
                return output;
            }
        }
    }
}

fn pull<T: Node>(node: &T) -> Vec<String> {
    node.pull()
}

fn main() {

    let mut file = File::open("example.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    let input = StandardIn {};
    let string_match = StringMatch {
        input: Some(&input),
        value: "abc".to_string(),
    };
    let output = StandardOut { input: Some(&string_match) };

    // connect(input, stringMatch);
    // connect(stringMatch, output);
    pull(&output);
}
