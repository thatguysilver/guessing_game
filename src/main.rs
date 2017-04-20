extern crate rand; //imports rand

use std::io; //imports io from the standard library
use std::cmp::Ordering; //for using match
use rand::Rng; //uses the rng method from the rand crate

fn main() { //defines main function
    println!("Gimme a number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //rand is the module
    //:: is the secret magical bullshit
    //thread is related to threading; not totally sure.!
    //rng() is a method from rand

    println!("It's {}", secret_number); //for testing purposes only

    let mut guess = String::new();
        //let starts defining a variable
        //mut says it's mutable (by default, Rust var is immutable)
        //guess is our fn name
        //String says it's a UTF-8 String
        //:: is esoteric shit that py has not prepared me for
        //new() is a common type of identifier that says guess is a new var

    io::stdin().read_line(&mut guess)
    //io::stdin() is more esoteric Rust shit. Dammit.
    //read_line() is a method of io (???) that reads lines.
    //& is not explained well by official doc. I guess it precedes any argument being passed to a function.

        .expect("Failed to read line");
    //sidenote: this all could have been one line.
    //anyway, .expect() is similar to py's try/except syntax, except cleaner.

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    //adds to the guess var by defining it as a u32 number, an
    //unsigned thirty-two bit number. Rust defalts to i32, a regular
    //32-bit number, so we had to convert it when we called it.

    println!("You guessed: {}", guess);
    //this is straight out of py, but cleaner. For testing purposes only.

    match guess.cmp(&secret_number) { //cmp() compares using the less, greater, or equal operators
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win!"),
    }

}
