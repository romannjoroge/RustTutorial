pub mod typing;
pub mod crud;

use typing::title;
use typing::get_input;
use crud::create;
use crud::read;
use crud::update;
use crud::delete;

fn main() {
    // Print title of app
    let title = String::from("Quote Generator");
    let title_ref = &title;

    title::print_title(20, title_ref);

    // Show instructions to user
    println!("This command line application allows you to manage your quotes, please select one of the options:");
    println!("");
    println!("1) Create A Quote");
    println!("2) Update A Quote");
    println!("3) Delete A Quote");
    println!("4) Read A Quote");
    println!("");

    let mut correct_option = false;
    // Repeat until correct option is chosen
    while correct_option == false {
        // Get user input
        let input: String = get_input::get_input();

        // Trim input and make it lower case
        let trim_input = input.trim().to_ascii_lowercase();

        // Do action depending on input
        if trim_input == "1" {
            create::create_quote();
            correct_option = true;
        } else if trim_input == "2" {
            update::update_quote();
            correct_option = true;
        } else if trim_input == "3" {
            delete::delete_quote();
            correct_option = true;
        } else if trim_input == "4" {
            read::read_quote();
            correct_option = true;
        } else {
            println!("Invalid Option! Try Again");
            correct_option = false;
        }
    }
}