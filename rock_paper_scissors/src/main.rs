use std::{fmt::Display, io::stdin};

use console::{Key, Term};

use anyhow::{anyhow, Ok, Result};
use rand::{rngs::ThreadRng, Rng};

#[repr(u8)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Move::Rock => "Rcok",
                Move::Paper => "Paper",
                Move::Scissors => "Scissors",
            }
        )
    }
}

struct App {
    wins: usize,
    draws: usize,
    losses: usize,
    term: Term,
}

impl Default for App {
    fn default() -> Self {
        Self {
            wins: Default::default(),
            draws: Default::default(),
            losses: Default::default(),
            term: Term::stdout(),
        }
    }
}

impl App {
    fn run(&mut self) -> Result<()> {
        let mut rng = ThreadRng::default();
        self.term.clear_screen()?;

        loop {
            self.term.write_line("Rock, Paper, Scissors")?;
            self.term.write_line("")?;

            let player_move = match self.get_input() {
                std::result::Result::Ok(value) => match value {
                    Some(value) => value,
                    None => break,
                },
                Err(error) => {
                    self.term.clear_screen()?;

                    println!("{error}");
                    continue;
                }
            };

            let computer_move = match rng.gen_range(0..3) {
                0 => Move::Rock,
                1 => Move::Paper,
                2 => Move::Scissors,
                _ => unreachable!(),
            };

            println!("The computer chose {computer_move}");

            match (player_move, computer_move) {
                (Move::Rock, Move::Paper)
                | (Move::Paper, Move::Scissors)
                | (Move::Scissors, Move::Rock) => {
                    println!("You lose.");
                    self.losses += 1;
                }

                (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper) => {
                    println!("You win.");
                    self.wins += 1;
                }
                _ => {
                    println!("This game was a draw.");
                    self.draws += 1;
                }
            }

            println!(
                "Score: {} wins, {} losses, {} draws",
                self.wins, self.losses, self.draws
            );

            println!("Press Enter To Continue...");

            loop {
                match self.term.read_key()? {
                    Key::Enter => {
                        self.term.clear_screen()?;
                        break;
                    }
                    _ => continue,
                };
            }
        }

        Ok(())
    }

    fn get_input(&self) -> Result<Option<Move>> {
        let mut input = String::new();
        self.term
            .write_line("Choose [r]ock, [p]aper, [s]cissors, or [e]xit:")?;

        stdin().read_line(&mut input)?;

        Ok(match input.trim() {
            "rock" | "r" => Some(Move::Rock),
            "paper" | "p" => Some(Move::Paper),
            "scissors" | "s" => Some(Move::Scissors),
            "exit" | "e" => {
                self.term.clear_screen()?;
                None
            }
            _ => return Err(anyhow!("Invalid Input. Try Again...")),
        })
    }
}

fn main() -> Result<()> {
    let mut app = App::default();
    app.run()?;

    Ok(())
}
