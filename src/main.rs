use std::char::from_digit;
use std::io::stdin;
use std::convert::TryInto;

fn main() {
    let user_options: [&str; 3] = [
        "Print 1st 10 fibonacci numbers",
        "Convert number to binary",
        "Exit",
    ];
    let mut display_text: String = String::new();

    // Display options & execute user choise until they exit 
    loop {
        // Print user_options as a numbered list
        println!("Enter number below to choose option:");
        for i in 0..user_options.len() {
            println!("{}. {}", i + 1, user_options[i]);
        }
    
        // Get user input
        // Todo: Use Ok() or Err() for input validation
        let mut user_choice: String = String::new();
        stdin().read_line(&mut user_choice).unwrap();
        user_choice = user_choice.trim().to_string();

        if user_choice == "1" {
            display_text = fibonacci();
        } else if user_choice == "2" {
            display_text = number_to_binary()
        } else if user_choice == "3" {
            break;
        }

        println!("{}\n", display_text);
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

fn number_to_binary() -> String {
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
    let mut binary_str = String::new();
    
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
    
    let ret_text: String = format!("\n{} in binary: {}\n", user_num_res.trim(), binary_str);

    ret_text    
}
