use std::char::from_digit;
use std::convert::TryInto;
use std::io::stdin;

fn main() {
    // Todo: Turn user_options into HashMap
    let user_options: [&str; 3] = [
        "Print 1st 10 fibonacci numbers",
        "Convert number to binary",
        "Exit",
    ];

    // Loop: display options & execute user choise until they exit 
    loop {
        // Print user_options as a numbered list
        println!("Enter number below to choose option:");
        for i in 0..user_options.len() {
            println!("{}. {}", i + 1, user_options[i]);
        }
    
        // Get user input
        // Todo: Use Ok() or Err() for input validation
        let mut user_option: String = String::new();
        stdin().read_line(&mut user_option).unwrap();
        user_option = user_option.trim().to_string();

        // Todo: reference HashMap in future
        if user_option == "1" {
            let fibonacci_text: String = fibonacci();
            println!("{}", fibonacci_text);
        } else if user_option == "2" {
            println!("\nEnter number to convert to binary: ");
            let mut user_num_res: String = String::new();
            stdin().read_line(&mut user_num_res).unwrap();
            let user_num: i32 = match user_num_res.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    return;
                }
            };
            
            let u_user_num: u32 = user_num.try_into().unwrap();
            println!(
                "{} in binary: {}",
                user_num,
                number_to_binary(u_user_num)
            );
        } else if user_option == "3" {
            break;
        }

        println!();
    }
}

fn fibonacci() -> String {
    let mut current: i32 = 1;
    let mut prev: i32 = 0;
    let mut ret_text: String = String::new();

    ret_text.push_str("\n1st 10 fibonacci numbers:");

    for counter in 1..11 {
        let next: i32 = prev + current;
        let fib_text: String = format!("\n{}: {}", (counter).to_string(), prev);
        ret_text.push_str(&fib_text);
        prev = current;
        current = next;
    }

    ret_text
}

fn number_to_binary(u_user_num: u32) -> String{
    let mut binary_str = String::new();
    let mut u_int: u32 = u_user_num;

    if u_int == 0 {
        binary_str.push('0');
    }

    while u_int > 0 {
        let remainder: u32 = u_int % 2;
        binary_str.insert(0, from_digit(remainder, 10).unwrap());
        u_int /= 2;
    }

    binary_str
}
