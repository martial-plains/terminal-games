use std::io::{Read, Write};

use rand::{thread_rng, Rng};

fn main() {
    let mut player_points = 0;
    let mut rival_points = 0;

    let mut rng = thread_rng();

    println!("Dice Game");
    println!();
    println!("In this game you and a computer Rival will play 10 rounds");
    println!("where you will each roll a 6-sided dice, and the player");
    println!("with the highest dice value will win the round. The player");
    println!("who wins the most rounds wins the game. Good luck!");
    println!();

    print!("Press Enter key to start...");
    std::io::stdout()
        .flush()
        .expect("Could not flush to standard output");

    {
        let mut buffer = [0; 1];
        std::io::stdin()
            .read_exact(&mut buffer)
            .expect("Failed to read input");
    };

    println!();
    println!();

    for i in 0..10 {
        println!("Round {}", i + 1);

        let rival_random_num = rng.gen_range(1..7);
        println!("Rival rolled a {rival_random_num}");

        print!("Press Enter key to roll the dice...");
        std::io::stdout()
            .flush()
            .expect("Could not flush to standard output");

        {
            let mut buffer = [0; 1];
            std::io::stdin()
                .read_exact(&mut buffer)
                .expect("Failed to read input");
        };

        println!();

        let player_random_num = rng.gen_range(1..7);
        println!("You rolled a {player_random_num}");

        match player_random_num.cmp(&rival_random_num) {
            std::cmp::Ordering::Less => {
                rival_points += 1;
                println!("The Rival won this round.");
            }
            std::cmp::Ordering::Equal => println!("This round is a draw."),
            std::cmp::Ordering::Greater => {
                player_points += 1;
                println!("You won this round.");
            }
        }

        println!("The score is now - You : {player_points}. Rival : {rival_points}.");

        print!("Press Enter key to continue...");
        std::io::stdout()
            .flush()
            .expect("Could not flush to standard output");

        {
            let mut buffer = [0; 1];
            std::io::stdin()
                .read_exact(&mut buffer)
                .expect("Failed to read input");
        };

        println!();
        println!();
    }

    println!("Game over.");
    println!("The score is now - You : {player_points}. Rival : {rival_points}.");

    match player_points.cmp(&rival_points) {
        std::cmp::Ordering::Less => println!("You lost!"),
        std::cmp::Ordering::Equal => println!("You won!"),
        std::cmp::Ordering::Greater => println!("This game is a draw."),
    }

    println!("Press Enter key to exit...");
    {
        let mut buffer = [0; 1];
        std::io::stdin()
            .read_exact(&mut buffer)
            .expect("Failed to read input");
    };
}
