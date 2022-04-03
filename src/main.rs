use std::io;

fn main() {
    println!("The Fibonacci Sequence!");
    println!("Enter the amount of sequences to generate");

    let mut num_sequence = String::new();

    io::stdin()
        .read_line(&mut num_sequence)
        .expect("Failed to read line");

    let num_sequence: u32 = match num_sequence.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    generate_fibonacci(num_sequence);
}

fn generate_fibonacci(iterations: u32) {
    let mut first: u128 = 0;
    let mut second: u128 = 1;

    for num in 0..(iterations + 1) {
        print!("{first}");
        if num != iterations {
            print!(", ")
        }
        let total = first + second;
        first = second;
        second = total;
    }
    println!("\nFinished")
}
