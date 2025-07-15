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
    pub action: VisitorAction,
    pub age: i8,
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    pub fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome {}", self.name);
                println!("{note}");
                if self.age < 21 {
                    println!("{} not allowed to drink alcohol", self.name);
                }
            }
            VisitorAction::Probation => println!("{} in probation", self.name),
            VisitorAction::Refuse => println!("{} not allowed to enter", self.name),
        }
    }
}

#[derive(Debug)]
pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
