use std::io::stdin;

fn main() {
    let mut prev: i32 = 0;
    let mut current: i32 = 1;
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
        println!("\n1st 10 fibonacci numbers:");

        for counter in 0..10 {
            print!("{}:", (counter + 1).to_string());
            println!("{}", prev);
            let next: i32 = prev + current;
            prev = current;
            current = next;
        }
    }
}
