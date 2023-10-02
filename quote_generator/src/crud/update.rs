use crate::title;

pub fn update_quote() {
    // Prints Heading
    let title = String::from("Update A Quote");
    let title_ref = &title;

    title::print_title(20, title_ref);
}