fn main() {
    happy_birthday(10);

    println!("You did an exam! Let's see your average:");
    println!("Average score: {}", calculate_mean_grade([100, 100, 100, 100, 100]));

    println!("You are now an adult and must pay tax!");

    pay_tax(1500.0);
}

fn calculate_mean_grade(grades: [u32; 5]) -> u32 {
    let mut summed_grades: u32 = 0;
    // Goes through each grade and adds it
    for grade in grades {
        summed_grades += grade;
    }

    // Return mean
    return summed_grades / 5;
}

fn happy_birthday(age: u32) {
    let mut i = 1;

    // Looping with the loop keyword
    loop {
        if i < age {
            println!("Are you {}", i);
            i = i + 1; // Remember to increment control variable
        } else if i == age { // break condition
            println!("Are you {}", i);
            println!("Hooray!");
            break
        } else {
            break
        }
    }
}

fn pay_tax(income: f64) {
    let mut remaining_income = income;
    // Pays tax as long as there is income left
    while remaining_income > 500.0 {
        remaining_income -= 500.0;
        println!("kshs 500 has been removed for tax, balance is {}", remaining_income);
    }
}
