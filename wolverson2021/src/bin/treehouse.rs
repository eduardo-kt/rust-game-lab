//! Binário do exemplo c.2 p.20-
use wolverson2021::treehouse;
fn main() {
    let name = treehouse::what_is_your_name();
    

    let visitor_list = [
        treehouse::Visitor::new("eduardo", "Hello eduardo"),
        treehouse::Visitor::new("Omar", "Nice to see you again, Omar!"),
        treehouse::Visitor::new("Carl", "Good day, Carl."),
    ];
    
    let mut allow_them_in = false;

    for visitor in &visitor_list {
        if visitor.name == name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome to the Treehouse, {}.", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}
