
extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;
use std::ops::DerefMut;

trait Node {
    fn pull(&mut self) -> Vec<String>;
    /*fn setInput(&self, &Node) -> ();*/
}

struct StandardIn {}

impl Node for StandardIn {
    fn pull(&mut self) -> Vec<String> {
        return vec!["abcdef".to_string(),
                    "ghijk".to_string(),
                    "asdfabcasdfasd".to_string()];
    }

    /*fn setInput(&self, node: &Node) {}*/
}



struct StandardOut {
    input: Option<Box<Node>>,
}

impl Node for StandardOut {
    fn pull(&mut self) -> Vec<String> {
        match self.input {
            None => return vec![],
            Some(ref mut input) => {
                let content = input.pull();
                println!("{:?}", content);
                return vec![];
            }
        }
    }

    /*
    fn setInput(&self, node: &Node) {
        self.input = Some(node);
    }
    */
}

struct StringMatch {
    input: Option<Box<Node>>,
    value: String,
}

impl Node for StringMatch {
    fn pull(&mut self) -> Vec<String> {
        match self.input {
            None => return vec![],
            Some(ref mut input) => {
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

    /*
    fn setInput(&self, node: &Node) {
        self.input = Some(node);
    }
    */
}

fn pull<T: Node>(node: &mut T) -> Vec<String> {
    node.pull()
}

fn main() {

    let mut file = File::open("example.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    let input = Box::new(StandardIn {});
    let string_match = Box::new(StringMatch {
                                    input: Some(input),
                                    value: "abc".to_string(),
                                });
    let mut output = Box::new(StandardOut { input: Some(string_match) });

    pull(output.deref_mut());

    // string_match.setInput(input);
    // // connect(stringMatch, output);
    // pull(output.borrow());
}
