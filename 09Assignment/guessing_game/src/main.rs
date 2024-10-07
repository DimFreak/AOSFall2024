fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    let secret_number: i32 = 42; 
    let mut guess: i32; // Mutable variable for the guess
    let mut attempts = 0; // number of attempts

    loop {
        // Simulating user input with a fixed guess
        // You can change this number to test different scenarios
        guess = 30 + attempts; // Change this for different guesses

        // Increment the number of attempts
        attempts += 1;

        
        match check_guess(guess, secret_number) {
            0 => {
                println!("Congratulations! You've guessed the correct number: {}", secret_number);
                break; // Exitif the guess is correct
            }
            1 => println!("Your guess of {} is too high.", guess),
            -1 => println!("Your guess of {} is too low.", guess),
            _ => unreachable!(), // This case should never occur
        }
    }

    // Print the number of attempts taken
    println!("It took you {} guesses.", attempts);
}
