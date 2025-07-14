//! biblioteca do exemplo c.2 p.20-
use std::io::stdin;

pub fn what_is_your_name() -> String {
    println!("Hello, what's your name? ");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}

#[derive(Debug)]
pub struct Visitor {
    pub name: String,
    pub greeting: String,
}

impl Visitor {
    pub fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    pub fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

#[derive(Debug)]
pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}