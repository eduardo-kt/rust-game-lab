//! Binário do exemplo c.2 p.20-
use wolverson2021::treehouse;
fn main() {
    let name = treehouse::what_is_your_name();

    let visitor_list = [
        treehouse::Visitor::new("eduardo", "Hello eduardo"),
        treehouse::Visitor::new("Omar", "Nice to see you again, Omar!"),
        treehouse::Visitor::new("Carl", "Good day, Carl."),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}