use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a text:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let returned_text = takes_ownership(input);

    println!("Returned text is: {}", returned_text);
}

fn takes_ownership(s: String) -> String{
    println!("The string is: {}", s);
    s
}
