use rand::Rng;
use std::io;

const VALID: [&str; 3] = ["steen", "papier", "schaar"];

fn main() {
    loop {
        game();

        println!("Wil je opnieuw spelen? (ja)");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");

        let c = c.trim().to_lowercase();

        if c != "ja" {
            break; // stop met loopen
        }
    }
}

fn game() {
    let choice = get_input();

    // generate een nummer tussen 0 en 2 (inclusief, 0 en 2 zijn dus mogelijke uitkomsten)
    let computer_choice: usize = rand::thread_rng().gen_range(0..=2);

    println!("Computer koos: {}", VALID[computer_choice]);

    if choice == computer_choice {
        // hex ascii escape sequence 93 en 1
        // 93 oranje, 1 dikgedrukt
        println!("\x1b[93;1mGelijkspel!\x1b[0m");
    }
    // rem_euclid geeft de remainder (remainder van -1 en 3 is 2)
    // computer_choice is van het type usize, maar die kan niet negatief zijn, dus casten we eerst computer choice naar i32
    // omdat choice van het type usize is, casten we hem weer terug naar usize
    else if choice == ((computer_choice as i32 - 1).rem_euclid(3) as usize) {
        println!("\x1b[91;1mComputer wint!\x1b[0m");
    } else {
        println!("\x1b[92;1mJij wint!\x1b[0m");
    }
}

fn get_input() -> usize {
    // clear het scherm
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("Kies uit: steen, papier, schaar");

    // bepaal nieuwe lege mutable string
    let mut c = String::new();

    // Schrijf input naar de string
    io::stdin().read_line(&mut c).expect("Failed to read line");

    // Iterate over de array en kijk of de string in de array zit en op welke index
    match VALID.iter().position(|&r| r == c.trim().to_lowercase()) {
        None => return get_input(), // geen overeenkomst
        Some(i) => return i,
    };
}
