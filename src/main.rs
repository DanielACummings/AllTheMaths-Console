use std::io::stdin;

fn main() {
    // Todo: turn into HashMap
    let user_options: [&str; 2] = [
        "Print 1st 10 fibonacci numbers",
        "Convert number to binary",
    ];

    // Print user_options as a numbered list
    println!("Enter number below to choose option:");
    for i in 0..user_options.len() {
        println!("{}. {}", i + 1, user_options[i]);
    }

    // Get user input
    // Todo: Use Ok() or Err() for input validation
    let mut user_option: String = String::new();
    stdin().read_line(&mut user_option).unwrap();

    // Todo: reference HashMap in future
    if user_option.trim() == "1" {
        let fibonacci_text: String = fibonacci();
        println!("{}", fibonacci_text);
    } else if user_option.trim() == "2" {
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
        number_to_binary(user_num);
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

fn number_to_binary(user_num: i32) {
    println!("{}", user_num.to_string());
}
