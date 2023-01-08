use std::{io::Write, ops::Range};

use anyhow::{anyhow, Ok, Result};
use rand::Rng;

/// Structure containing all application data
#[derive(Debug, Default)]
struct Game {
    secret: usize,
    range: Range<usize>,
    guesses: usize,
    input: String,
}

impl Game {
    /// Creates a new [`Game`] object
    fn new() -> Self {
        let range = 1..100;
        let secret = rand::thread_rng().gen_range(range.clone());

        Self {
            secret,
            range,
            guesses: 0,
            input: String::default(),
        }
    }

    /// Plays a loop of the [`Game`] object
    fn play(&mut self) -> Result<()> {
        loop {
            print!(
                "Guess a number ({} - {}): ",
                self.range.start,
                self.range.end - 1
            );
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut self.input)?;

            let flag = self.check();

            if let Err(error) = flag {
                println!("{error}");
            } else if let std::result::Result::Ok(correct_guess) = flag {
                if correct_guess {
                    break;
                }
            }

            self.guesses += 1;
            self.input = String::new();
        }
        Ok(())
    }

    /// Check if input was valid
    fn check(&self) -> Result<bool> {
        let valid = match self.input.trim().parse::<usize>() {
            std::result::Result::Ok(value) => value,
            Err(_) => return Err(anyhow!("The given input was invalid. Use only numbers.")),
        };

        match valid.cmp(&self.secret) {
            std::cmp::Ordering::Less => println!("Too Low!"),
            std::cmp::Ordering::Equal => {
                println!("Congrats! You are correct!");
                println!("You completed the game in {} guesses.", self.guesses);
                return Ok(true);
            }
            std::cmp::Ordering::Greater => println!("Too High"),
        };

        Ok(false)
    }
}

fn main() -> Result<()> {
    let mut game = Game::new();
    game.play()?;

    Ok(())
}
