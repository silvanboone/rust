use std::{
    collections::VecDeque,
    io::{stdout, Write},
    ops,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    Result,
};

use rand::Rng;

#[derive(Debug)]
struct Snake {
    pos: VecDeque<Point>,
    direction: Point,
    is_growing: bool,
}

impl Snake {
    fn move_forward(&mut self) {
        if !self.is_growing {
            self.pos.pop_back();
        }

        let head = self.pos.front().unwrap();
        let new_head = head + &self.direction;

        if !new_head.is_valid() || self.pos.contains(&new_head) {
            panic!("You lost!");
            // println!("You lost! pos: {:?}, new head: {:?}", self.pos, new_head);
        }

        self.pos.push_front(new_head);
        self.is_growing = false;
        let computer_choice: usize = rand::thread_rng().gen_range(0..=2);
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn is_valid(&self) -> bool {
        !(self.x > 31 || self.x < 0 || self.y > 31 || self.x < 0)
    }
}

impl ops::Add for &Point {
    type Output = Point;

    fn add(self, point: &Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

#[derive(Debug)]

struct Screen {
    // array pixels, in array rij van pixels, in array scherm
    pixels: [[[Pixel; 2]; 32]; 32],
    apple: Point,
}

impl Screen {
    fn print(&self) {
        println!("┌────────────────────────────────────────────────────────────────┐\r");
        for row in self.pixels {
            print!("│");
            for cell in row {
                for pixel in cell {
                    let i = match pixel {
                        Pixel::Black => " ",
                        Pixel::White => "█",
                        Pixel::Red => "\x1b[91m█\x1b[0m",
                    };
                    print!("{}", i);
                }
            }
            println!("│\r");
        }
        println!("└────────────────────────────────────────────────────────────────┘");
    }

    fn new_apple(&mut self) {
        self.apple = Point {
            x: rand::thread_rng().gen_range(0..=31),
            y: rand::thread_rng().gen_range(0..=31),
        };
    }

    fn set(&mut self, snake: &VecDeque<Point>) {
        // dit kan beter ...
        self.pixels = [[[Pixel::Black; 2]; 32]; 32];
        for i in snake {
            self.pixels[i.y as usize][i.x as usize][0] = Pixel::White;
            self.pixels[i.y as usize][i.x as usize][1] = Pixel::White;
        }
        self.pixels[self.apple.y as usize][self.apple.x as usize][0] = Pixel::Red;
        self.pixels[self.apple.y as usize][self.apple.x as usize][1] = Pixel::Red;
    }

    fn setup() {
        execute!(stdout(), SetSize(66, 35)).expect("Could not set terminal size");
        enable_raw_mode().expect("Could not enable raw mode");
    }

    fn clear() {
        print!("{esc}c", esc = 27 as char);
    }

    fn clean() {
        disable_raw_mode().expect("Could not disable raw mode");
    }
}

#[derive(Clone, Copy, Debug)]
enum Pixel {
    Black,
    White,
    Red,
}

fn main() {
    let screen: Screen = Screen {
        pixels: [[[Pixel::Black; 2]; 32]; 32],
        apple: Point {x: 0, y: 0}
    };

    let snake: Snake = Snake {
        pos: VecDeque::from([
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
        ]),
        direction: Point { x: 1, y: 0 },
        is_growing: false,
    };

    game(screen, snake);
}

fn game(mut screen: Screen, mut snake: Snake) {
    Screen::setup();
    screen.new_apple();

    loop {
        // https://github.com/crossterm-rs/crossterm/blob/master/examples/event-match-modifiers.rs
        if poll(Duration::from_millis(160)).expect("Could not poll") {
            let event: Event = read().expect("Could not read events");

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
            if event == Event::Key(KeyCode::Up.into()) {
                snake.direction = Point { x: 0, y: -1 };
            }
            if event == Event::Key(KeyCode::Left.into()) {
                snake.direction = Point { x: -1, y: 0 };
            }
            if event == Event::Key(KeyCode::Down.into()) {
                snake.direction = Point { x: 0, y: 1 };
            }
            if event == Event::Key(KeyCode::Right.into()) {
                snake.direction = Point { x: 1, y: 0 };
            }
        }
        snake.move_forward();

        if snake.pos.contains(&screen.apple) {
            screen.new_apple();
            snake.is_growing = true;
        }

        screen.set(&snake.pos);

        Screen::clear();
        screen.print();
    }

    Screen::clean();
}
// cd Programming/School/3,4/rust/snake ; cargo run
