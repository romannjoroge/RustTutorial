pub fn print_title(length_title_block: u32, title: &String) {
    // Print a long line
    print_line(length_title_block);

    // Print a pipe, title text then pipe at end
    print_title_text(length_title_block, title);

    // Print a long line
    print_line(length_title_block);
}

fn print_line(length_line: u32) {
    let mut i: u32 = 0;

    while i < length_line {
        print!("-");
        i += 1;
    }

    println!("");
}

fn print_title_text(length_block: u32, title: &String) {
    let mut i: u32 = 0;
    
    while i <= length_block {
        // Print a pipe first
        if i == 0 {
            print!("|");
        } else if i == 1 {
            // Print a space next
            print!(" ");
        } else if i == length_block{
            // Print pipe at end
            println!("|");
        } else if i <= title.len() as u32 + 1 {
            print!("{} ", title);
            i += title.len() as u32 + 1;
        } else {
            // Print spaces
            print!(" ");
        }

        i += 1;
    }
}