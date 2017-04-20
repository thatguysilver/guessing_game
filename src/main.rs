use std::io; //imports io from the standard library

fn main() { //
    println!("Gimme a number.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
