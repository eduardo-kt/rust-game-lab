//! Binário do exemplo c.2 p.20-
use wolverson2021::treehouse::{self, Visitor, VisitorAction};
fn main() {
    let mut visitor_list = vec![
        treehouse::Visitor::new("eduardo", VisitorAction::Accept, 45),
        treehouse::Visitor::new(
            "Omar",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose issues"),
            },
            15,
        ),
        treehouse::Visitor::new("Carl", VisitorAction::Refuse, 30),
    ];
    loop {
        let name = treehouse::what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }
    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
}
