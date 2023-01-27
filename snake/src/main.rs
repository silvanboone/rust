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
        self.pos.push_front(head + &self.direction);
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
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
}

impl Screen {
    fn print(&self) {
        for row in self.pixels {
            for cell in row {
                for pixel in cell {
                    let i = match pixel {
                        Pixel::Black => " ",
                        Pixel::White => "█",
                        Pixel::Red => "r",
                    };
                    print!("{}", i);
                }
            }
            println!("|\r");
        }
        println!("END SCREEN\r");
    }

    fn set(&mut self, points: &VecDeque<Point>) {
        self.pixels = [[[Pixel::Black; 2]; 32]; 32];
        for i in points {
            self.pixels[i.y as usize][i.x as usize][0] = Pixel::White;
            self.pixels[i.y as usize][i.x as usize][1] = Pixel::White;
        }
    }

    fn setup() {
        execute!(stdout(), SetSize(64, 32)).expect("Could not set terminal size");
        enable_raw_mode().expect("Could not enable raw mode");
    }

    fn clear() {
        Clear(ClearType::Purge);
        println!("\r");
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
    };

    let snake: Snake = Snake {
        pos: VecDeque::from([Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 3, y: 3 }]),
        direction: Point { x: 1, y: 0 },
        is_growing: false,
    };

    game(screen, snake);
}

fn game(mut screen: Screen, mut snake: Snake) {
    Screen::setup();

    loop {
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
        screen.set(&snake.pos);
        Screen::clear();

        // TODO
        // delta time
        // iets met directions


        // snake.move_forward();
        screen.print();
    }

    Screen::clean();
}
// cd Programming/School/3,4/rust/snake ; cargo run
