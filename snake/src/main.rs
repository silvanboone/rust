use std::{
    collections::VecDeque,
    io::{stdout, Write},
    ops,
};

use crossterm::{
    execute,
    terminal::{size, SetSize},
};

struct Snake {
    pos: VecDeque<Point>,
    direction: Point,
    is_growing: bool,
}

impl Snake {
    fn move_to(&mut self, point: Point) {
        if !self.is_growing {
            self.pos.pop_back();
        }

        let head = self.pos.front().unwrap();
        self.pos.push_front(head + &point);
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
        }
        println!();
    }

    fn set(&mut self, points: VecDeque<Point>) {
        for i in points {
            self.pixels[i.y as usize][i.x as usize][0] = Pixel::White;
            self.pixels[i.y as usize][i.x as usize][1] = Pixel::White;
        }
    }

    fn setup() {
        execute!(stdout(), SetSize(64, 32)).expect("Could not set terminal size");
        // raw mode
    }
}

#[derive(Clone, Copy, Debug)]
enum Pixel {
    Black,
    White,
    Red,
}

fn main() {
    Screen::setup();

    let mut screen: Screen = Screen {
        pixels: [[[Pixel::Black; 2]; 32]; 32],
    };

    let mut snake: Snake = Snake {
        pos: VecDeque::from([
            Point { x: 0, y: 2 },
            Point { x: 1, y: 3 },
            Point { x: 1, y: 4 },
        ]),
        direction: Point { x: 0, y: 0 },
        is_growing: false,
    };

    screen.set(snake.pos);
    screen.print();
    // println!("Old pos: {:?}", snake.pos);
    // snake.move_to(Point { x: 1, y: 1 });
    // println!("New pos: {:?}", snake.pos);
    // snake.is_growing = true;
    // snake.move_to(Point { x: 1, y: 1 });
    // println!("New pos: {:?}", snake.pos);
}
