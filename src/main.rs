use std::io; // bring the io input/output library into scope.
// The io library comes from the standard library, known as std
// --> ability to accept user input


use rand::Rng; // Phase 2 code adding rand library linked to dependency files

fn main() {
    // CODE PHASE 1

    // println!("Guess the number!"); //  macro that prints a string to the screen

    // println!("Please input your guess.");

    // let mut guess = String::new();
            // let --> create a variable
            //  + mut mutable variable
            // guess --> function name

            // = is used to bind the variable guess to the instance of a String

            // String type
            //  :: --> associated fonction
            //  new associated function of the String type


            // ------
            // let mut guess = String::new();
            // has created a mutable variable
            // that is currently bound to a new, empty instance of a String
            // ------

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

            // call the stdin function from the io module

            // .read_line() --> take user input
            // .read_line(&mut guess) --> take user input and bind it
                // to mut var guess --> String type

            // .expect linked to Result Enum element to handle error
            // Result’s variants are Ok and Err
                // If this instance of Result is an Err value,
                // expect will cause the program to crash
                // and display the message that you passed as an argument to expect

            // io::stdin().read_line(&mut guess).expect("Failed to read line");
            // --> refacto

    // println!("You guessed: {guess}");

            // {} --> placeholder for the variable

            // println!("You guessed: {}", guess); --> also work


    // CODE PHASE 2

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
