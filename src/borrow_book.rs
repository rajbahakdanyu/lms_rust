use crate::display_booklist::read_booklist;
use crate::utils::read_input;

pub fn borrow_book() {
    println!("");
    read_booklist();
    let mut borrower_name = String::new();

    println!("Enter borrower name: ");
    read_input(&mut borrower_name);
}
