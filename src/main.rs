use std::io::stdin;

fn main() {
    let user_options: [&str; 1] = ["Print 1st 10 fibonacci numbers"];

    // Print user_options as a numbered list
    println!("Enter number below to choose option:");
    for i in 0..user_options.len() {
        println!("{}. {}", i + 1, user_options[i]);
    }

    // Get user input
    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).unwrap();

    if user_input.trim() == "1" {
        let fibonacci_text: String = fibonacci();
        println!("{}", fibonacci_text);
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
