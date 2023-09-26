use chrono::Utc;
use std::char::from_digit;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let current_datetime = Utc::now();
    let formatted_datetime = current_datetime.format("%Y-%m-%d-%H%M%S");
    let file_name = format!(
        "AllTheMaths_Log_UTC{}.txt", formatted_datetime);
    let mut file = File::create(file_name)?;

    let mut display_text: String = String::new();
    let mut user_options: Vec<(String, Box<dyn Fn() -> String>)> = Vec::new();
    user_options.push((
        String::from("Print 1st 10 fibonacci numbers"),
        Box::new(fibonacci)
    ));
    user_options.push((
        String::from("Convert number to binary"),
        Box::new(number_to_binary)
    ));
    
    loop {
    // Display options & execute user choise until they exit 
        // Print user_options as a numbered list
        println!("\nEnter number below to choose option:");
        let mut i = 0;
        for (option, _) in &user_options {
            i += 1;
            println!("{}. {}", i, option);
        }
        // Print final number option for exiting
        println!("{}. Exit", user_options.len() + 1);
    
        // Get user input
        let mut user_choice: String = String::new();
        stdin().read_line(&mut user_choice).unwrap();
        
        // Convert input to a number
        let option_index: usize = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter number included in menu");
                continue;}
        };

        // Call function or exit
        if option_index == user_options.len() + 1 {
            break;
        }
        else if let Some(func) =
            user_options.get(option_index - 1).map(|(_, f)| f.as_ref()) {
            display_text = func();
        } else {
            println!("Invalid option");
        }

        // Display function output & write it to session log
        println!("\n{}", display_text);
        file.write_all(format!("\n{}\n", display_text).as_bytes())?;
    }

    Ok(())
}

fn fibonacci() -> String {
    let mut current: i32 = 1;
    let mut prev: i32 = 0;
    let mut ret_text: String = String::new();

    ret_text.push_str("1st 10 fibonacci numbers:");

    for counter in 1..=10 {
        let fib_text: String = format!("\n{}: {}", (counter).to_string(), prev);
        let next: i32 = prev + current;
        ret_text.push_str(&fib_text);
        prev = current;
        current = next;
    }

    ret_text
}

fn number_to_binary() -> String {
    // TODO: Handle large numbers
    println!("\nEnter number to convert to binary: ");

    let mut user_num_res: String = String::new();
    stdin().read_line(&mut user_num_res).unwrap();
    let user_num: i32 = match user_num_res.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return String::new();
        }
    };
    let mut u_user_num: u32 = user_num.try_into().unwrap();
    let mut binary_str: String = String::new();
    
    // Build binary string
    if u_user_num == 0 {
        binary_str.push('0');
    }
    while u_user_num > 0 {
        let remainder: u32 = u_user_num % 2;
        binary_str.insert(0, from_digit(remainder, 10)
            .unwrap());
        u_user_num /= 2;
    }
    
    let ret_text: String = format!("{} in binary: {}", user_num_res.trim(), binary_str);

    ret_text    
}
