use crate::title;

pub fn read_quote() {
    // Prints Heading
    let title = String::from("Read A Quote");
    let title_ref = &title;

    title::print_title(20, title_ref);
}