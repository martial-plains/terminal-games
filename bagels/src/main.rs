const NUM_DIGITS: usize = 3;
const MAX_GUESSES: i32 = 10;

fn main() -> Result<(), std::io::Error> {
    println!(
        r#"
Bagels, a deductive logic game.
    
I am thinking of a {NUM_DIGITS}-digit number with no repeated digits.
Try to guess what it is. Here are some clues:
When I say:     That means:
Pico            One digit is correct but in the wrong position.
Fermi           One digit is correct and in the right position.
Bagels          No digit is correct.

For example, if the secret number was 248 and your guess was 843, the
clues would be Fermi Pico.
    "#
    );

    // Main game loop
    loop {
        // This stores the secret number the player needs to guess:
        let secret_num = get_secret_number();

        println!("I have thought up a number.");
        println!("You have {MAX_GUESSES} guesses to get it.");

        let mut num_guesses = 1;

        while num_guesses <= MAX_GUESSES {
            let mut guess = String::new();
            // Keep looping until they enter a valid guess:
            while (guess.len() != NUM_DIGITS) || guess.parse::<u32>().is_err() {
                println!("Guess #{num_guesses}: ");

                guess.clear();
                std::io::stdin().read_line(&mut guess)?;
                guess = guess.trim().to_owned();
            }

            let clues = get_clues(&guess, &secret_num);
            println!("{}", clues.expect("There are no clues"));
            num_guesses += 1;

            if guess == secret_num {
                break; // They are correct so break out of the loop
            }

            if num_guesses > MAX_GUESSES {
                println!("You ran out of guesses.");
                println!("The answer was {secret_num}.");
            }
        }

        // Ask player if they want to play again.
        println!("Do you want to play again? (yes or no)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.to_lowercase().contains("yes") {
            break;
        }
    }

    Ok(println!("Thanks for playing!"))
}

/// Returns a a number made up of a number unique random digits.
fn get_secret_number() -> String {
    format!("{0:0nums$}", rand::random::<usize>(), nums = NUM_DIGITS)
        .chars()
        .take(NUM_DIGITS)
        .collect()
}

/// Returns a string with pico, fermi, bagels clues for a guess and secret
/// number pair
fn get_clues(guess: &str, secret_num: &str) -> Option<String> {
    if guess == secret_num {
        return Some(String::from("You got it!"));
    }

    let mut clues = vec![];

    for i in 0..guess.len() {
        if guess.chars().nth(i)? == secret_num.chars().nth(i)? {
            // A correct digit is in the correct place.
            clues.push("Fermi");
        } else if secret_num.contains(guess.chars().nth(i)?) {
            // A correct digit is in the incorrect place.
            clues.push("Pico");
        }
    }

    if clues.is_empty() {
        // There are no correct digits at all
        Some(String::from("Bagels"))
    } else {
        // Sort the clues into alphabetical order so their original order
        // doesn't give information away.
        clues.sort();
        // Make single string from the list of string clues.
        Some(clues.iter().fold(String::new(), |mut init, val| {
            init.push_str(val);
            init
        }))
    }
}
