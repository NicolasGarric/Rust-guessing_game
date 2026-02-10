use std::io; // bring the io input/output library into scope.
// The io library comes from the standard library, known as std
// --> ability to accept user input


use rand::Rng; // Phase 2 code adding rand library linked to dependency files
use std::cmp::Ordering; // Phase 2 code to compare random number with elements

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

    // rand::thread_rng --> function triggering random number
    // gen_range method --> linked to the thread_rng() random generator
        // taking start..=end range typo for range
        // 1..=100 to request a number between 1 and 100

    // IMPORTANT : cargo doc --open --> opens dependentie's documentation

    // Debug to show the secret number
    // println!("The secret number is: {secret_number}");

    loop {
        // Adding loop

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // First guess variable shadowed by the second
        // guess.trim().parse()
            // guess is the older guess variable before shadowing (String)
            // .trim() eliminates whitespace at the begining and the end (must do)
            // parse method on strings converts a string to another type

        // let guess: u32
            // : annotates the variable type
            // u32 = 32-bit integer

        // moving from expect to match expression to not crash but deal with error
            // parse return a Result type witch is an enum with Ok and Err

        // If parse correctly works it will return a number --> num
            // guess: u32 = match
            // Ok(num) => num, --> num will be return and guess will take the value

        // If parse doesn't work --> don't find num
            // Err(_) the "_" catch all the value

        println!("You guessed: {guess}");

        // Adding elements to comapre guess var and secret number random integer
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                // break --> exit the loop when the player win
            }

            // Ordering have three variants: Less / Equal / Greater
            // cmp method comapre two values
                // guess is compared (cmp) to &secret_number
            // match method is used: match guess.cmp(&secret_number)
                // match use arms = pattern to match against
        }
    }
}
