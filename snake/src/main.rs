use std::{
    collections::VecDeque,
    fmt::Display,
    io::Write,
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

use anyhow::{anyhow, Ok, Result};
use console::{Key, Term};
use rand::{rngs::ThreadRng, Rng};
use terminal_size::{Height, Width};

/// The direction the the possition is facing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// The opposit direction
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Checks if `other` is the opposite duirection
    fn is_opposite(&self, other: Direction) -> bool {
        self.opposite() == other
    }
}

impl From<&Direction> for char {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.into();
        write!(f, "{c}")
    }
}

/// Each pixel that is rendered to the console.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(usize)]
enum Tile {
    #[default]
    Open,
    Snake,
    Food,
}

/// The `x` and `y` positions represented in one object.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Position {
    x: isize,
    y: isize,
}

/// The global struct that contains all the data used for the program.
struct Game {
    random: ThreadRng,
    term: Term,
    sleep: Duration,
    width: isize,
    height: isize,
    close_requested: bool,
    direction: Option<Direction>,
    position: Position,
    map: Vec<Vec<Option<Tile>>>,
    snake: VecDeque<Position>,
}

/// The message enum containing all types of messages that the program can use
/// to pass data.
enum Msg {
    MoveDirection(Direction),
    OnKeyPress(Key),
    Close { message: String },
}

impl Game {
    /// Creates a new [`Game`] object.
    fn new() -> Self {
        let (width, height): (isize, isize);

        if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
            width = w as isize;
            height = h as isize;
        } else {
            width = 0;
            height = 0;
        }

        Self {
            random: ThreadRng::default(),
            term: Term::stdout(),
            direction: None,
            width,
            height,
            sleep: Duration::from_millis(70),
            snake: VecDeque::default(),
            position: Position {
                x: width / 2,
                y: height / 2,
            },
            map: vec![vec![None; height as usize]; width as usize],
            close_requested: false,
        }
    }

    /// Initializes the [`Game`] object.
    fn init(&mut self) -> Result<()> {
        self.term.hide_cursor()?;
        self.term.clear_screen()?;
        self.snake.push_back(self.position);
        self.map[self.position.x as usize][self.position.y as usize] = Some(Tile::Snake);
        self.position_food()?;
        self.term
            .move_cursor_to(self.position.x as usize, self.position.y as usize)?;
        self.term.write_all(b"@")?;

        while self.direction.is_none() && !self.close_requested {
            let key = self.term.read_key()?;
            self.update(Msg::OnKeyPress(key))?;
        }

        Ok(())
    }

    /// Renders the [`Game`] object to the console.
    fn render(&mut self) -> Result<()> {
        if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
            if self.width != w as isize && self.height != h as isize {
                self.update(Msg::Close {
                    message: "Console was resized. Snake game has ended.".to_string(),
                })?;

                return Ok(());
            }

            if let Some(direction) = self.direction {
                match direction {
                    Direction::Up => self.position.y -= 1,
                    Direction::Down => self.position.y += 1,
                    Direction::Left => self.position.x -= 1,
                    Direction::Right => self.position.x += 1,
                }
            }

            if self.position.x < 0
                || self.position.y < 0
                || self.position.x >= self.width
                || self.position.y >= self.height
                || matches!(
                    self.map[self.position.x as usize][self.position.y as usize],
                    Some(Tile::Snake)
                )
            {
                self.update(Msg::Close {
                    message: format!("Game Over. Score: {}.\n", self.snake.len() - 1),
                })?;

                return Ok(());
            }

            self.term
                .move_cursor_to(self.position.x as usize, self.position.y as usize)?;

            if let Some(direction) = self.direction {
                self.term.write_all(direction.to_string().as_bytes())?
            }

            self.snake.push_back(Position {
                x: self.position.x,
                y: self.position.y,
            });

            if self.map[self.position.x as usize][self.position.y as usize] == Some(Tile::Food) {
                self.position_food()?
            } else {
                let Position { x, y } = self.snake.pop_front().ok_or(anyhow!("Cannot dequeue"))?;
                self.map[x as usize][y as usize] = Some(Tile::Open);
                self.term.move_cursor_to(x as usize, y as usize)?;
                self.term.write_all(b" ")?;
            }

            self.map[self.position.x as usize][self.position.y as usize] = Some(Tile::Snake);

            sleep(self.sleep);

            if let Some(direction) = self.direction {
                self.update(Msg::MoveDirection(direction))?;
            }
        }
        Ok(())
    }

    /// Updates the [`Game`] object.
    fn update(&mut self, msg: Msg) -> Result<()> {
        match msg {
            Msg::MoveDirection(direction) => {
                if let Some(current_direction) = self.direction {
                    if direction.is_opposite(current_direction) {
                        return Ok(());
                    }
                }

                self.direction = Some(direction)
            }
            Msg::OnKeyPress(key) => match key {
                console::Key::ArrowLeft => self.update(Msg::MoveDirection(Direction::Left))?,
                console::Key::ArrowRight => self.update(Msg::MoveDirection(Direction::Right))?,
                console::Key::ArrowUp => self.update(Msg::MoveDirection(Direction::Up))?,
                console::Key::ArrowDown => self.update(Msg::MoveDirection(Direction::Down))?,
                console::Key::Escape => self.update(Msg::Close {
                    message: "Snake game closed".to_string(),
                })?,
                _ => return Ok(()),
            },
            Msg::Close { message } => {
                self.close_requested = true;
                self.term.show_cursor()?;
                self.term.clear_screen()?;
                self.term.write_all(message.as_bytes())?;
            }
        };
        Ok(())
    }

    /// Places food in a random position.
    fn position_food(&mut self) -> Result<()> {
        let mut posible_coords = Vec::new();

        for i in 0..self.width {
            for j in 0..self.height {
                posible_coords.push(Position { x: i, y: j })
            }
        }

        let index = self.random.gen_range(0..posible_coords.len());

        let Position { x, y } = posible_coords[index];
        self.map[x as usize][y as usize] = Some(Tile::Food);
        self.term.move_cursor_to(x as usize, y as usize)?;
        self.term.write_all(b"+")?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut game = Game::new();
    game.init()?;

    let captured_term = game.term.clone();

    // This thread is for getting key input from the console because if the
    // `read_key` is called on the main thread the game will stop and wait for
    // the player to press a key to continue every time the snake moves.
    thread::spawn(move || -> Result<()> {
        let mut last_key = Key::Unknown;

        loop {
            if game.direction.is_some() {
                let read_key = captured_term.read_key()?;

                if last_key != read_key {
                    tx.send(read_key.clone())?;
                    last_key = read_key;
                }
            }

            if game.close_requested {
                break Ok(());
            }
        }
    });

    // This is the main game loop that will be rendering the game
    loop {
        game.render()?;
        if let std::result::Result::Ok(key) = rx.try_recv() {
            game.update(Msg::OnKeyPress(key))?;
        }

        if game.close_requested {
            break;
        }
    }

    Ok(())
}
