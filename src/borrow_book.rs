use crate::display_booklist::read_booklist;

pub fn borrow_book() {
    read_booklist();
    println!("This is the borrow file\n")
}
