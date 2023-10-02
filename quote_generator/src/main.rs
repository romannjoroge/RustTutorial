pub mod typing;

use typing::title;
use typing::get_input;

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

    // Get user input
    let input: String = get_input::get_input();
    println!("{}", input);

    // Do action depending
}