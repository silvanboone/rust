use io::{Read, Write};
use rand::seq::SliceRandom;
use std::{fs, io};
struct Game {
    word: String,
    guesed_letters: String,
    lives: i32,
}

fn main() {
    loop {
        hangman();
        pause();
    }
}

fn hangman() {
    let mut game: Game = Game {
        word: random_word(),
        guesed_letters: String::new(),
        lives: 10,
    };

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        print_image(game.lives);

        // We geven een reference naar de strings, omdat anders die functie de eigenaar wordt van de variabelen
        println!("Woord: {}", format_word(&game.word, &game.guesed_letters));
        println!("Gerade letters: {}", game.guesed_letters);

        if game.lives <= 0 {
            println!("\x1b[91;1mJe hebt verloren! Het woord was: {}\x1b[0m", game.word);
            break;
        }

        if !format_word(&game.word, &game.guesed_letters).contains("_") {
            println!("\x1b[92;1mJe hebt gewonnen!\x1b[0m");
            break;
        }

        print!("Raad een letter: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let guessed_letter: String = match get_letter(&game.guesed_letters) {
            Some(c) => c,
            None => continue,
        };

        game.guesed_letters.push_str(&guessed_letter);

        if !game.word.contains(&guessed_letter) {
            game.lives -= 1;
        }
    }
}

fn pause() {
    let mut stdout = io::stdout();
    stdout.write(b"Druk op Enter om door te gaan...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}

// Nadat de functie is uitgevoerd leegt Rust alle memory
// de woorden in words zijn allemaal een reference naar de originele String, in dit geval f
// het resultaat is dan dat de woorden allemaal een reference zijn naar een variabele die niet meer bestaat
// Rust is memory safe, dus dat is niet toegestaan en de code zal niet compilen

// De oplossing is om de reference naar de string (&str) te casten naar een String
// to_string()
fn random_word() -> String {
    let f: String = fs::read_to_string("words.txt").expect("Could not read file");
    let w: Vec<&str> = f.split("\n").collect();
    w.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn format_word(word: &str, letters: &str) -> String {
    word.chars()
        .map(|c| {
            if letters.contains(c) {
                format!("{c} ")
            } else {
                "_ ".to_string()
            }
        })
        .collect()
}

fn get_letter(l: &str) -> Option<String> {
    let mut i: String = String::new();
    io::stdin().read_line(&mut i).expect("Unable to read stdin");

    i = i.trim().to_lowercase().to_string();

    if i.len() != 1 || l.contains(&i) {
        return None;
    }

    Some(i)
}

fn print_image(lives: i32) {
    match lives {
        0 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r" / \  |");
            println!(r"      |");
            println!(r"=========");
        }
        1 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r" /    |");
            println!(r"      |");
            println!(r"=========");
        }
        2 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        3 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        4 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r"  |   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        5 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        6 => {
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        7 => {
            println!(r"  +---+");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        8 => {
            println!(r"       ");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }

        9 => {
            println!();
            println!();
            println!();
            println!();
            println!();
            println!();
            println!("=========");
        }

        _ => {
            println!();
            println!();
            println!();
            println!();
            println!();
            println!();
            println!();
        }
    }
}
