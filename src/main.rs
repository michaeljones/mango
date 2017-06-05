
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
    input: &'a Node,
}

impl<'a> Node for StandardOut<'a> {
    fn pull(&self) -> Vec<String> {
        let content = self.input.pull();
        println!("{:?}", content);
        return vec![];
    }
}

struct StringMatch<'a> {
    input: &'a Node,
    value: String,
}

impl<'a> Node for StringMatch<'a> {
    fn pull(&self) -> Vec<String> {
        let content = self.input.pull();
        let mut output = vec![];
        for i in &content {
            if i.contains(self.value.as_str()) {
                output.push(i.to_string());
            }
        }
        return output;
    }
}

fn pull<T: Node>(node: &T) -> Vec<String> {
    node.pull()
}

fn main() {
    let input = StandardIn {};
    let stringMatch = StringMatch {
        input: &input,
        value: "abc".to_string(),
    };
    let output = StandardOut { input: &stringMatch };

    // connect(input, stringMatch);
    // connect(stringMatch, output);
    pull(&output);
}
