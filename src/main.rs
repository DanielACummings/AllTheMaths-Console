fn main() {
    let mut n1 = 0;
    let mut n2 = 1;

    println!("1st 10 fibonacci numbers:");
    // Print 1st 2 numbers
    println!("{}\n{}", n1, n2);

    // Calculate & print remaining numbers
    for _ in 0..10 {
        println!("{}", (n1 + n2));

        let temp = n2;
        n2 = n1 + n2;
        n1 = temp;
    }
}
