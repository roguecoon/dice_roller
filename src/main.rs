use colored::*;
use rand::Rng;
use std::io::stdin;

fn main() {
    loop {
        println!("{}{}", "Wähle einen ", "🎲".blink());
        println!("1: 2 side dice");
        println!("2: 3 side dice");
        println!("3: 4 side dice");
        println!("4: 6 side dice");
        println!("5: 8 side dice");
        println!("6: 10 side dice");
        println!("7: 12 side dice");
        println!("8: 20 side dice");
        println!("9: 100 side dice");
        println!("0: ende");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.trim().parse::<u8>() {
            Ok(i) => {
                if i == 0 {
                    break;
                } else {
                    print!("\x1B[2J");
                    dice_role(i);
                }
            }

            Err(..) => println!("{} {}", "Kein gültiger Wert (0-9)! : ".magenta(), input),
        };
    }
}

fn dice_role(dice: u8) {
    let mut rng = rand::rng();
    let roll_result;
    match dice {
        1 => roll_result = rng.random_range(1..=2),
        2 => roll_result = rng.random_range(1..=3),
        3 => roll_result = rng.random_range(1..=4),
        4 => roll_result = rng.random_range(1..=6),
        5 => roll_result = rng.random_range(1..=8),
        6 => roll_result = rng.random_range(1..=10),
        7 => roll_result = rng.random_range(1..=12),
        8 => roll_result = rng.random_range(1..=20),
        9 => roll_result = rng.random_range(1..=100),
        _ => roll_result = 0,
    };

    if roll_result != 0 {
        let crit = match dice {
            1 if roll_result == 2 => true,
            2 if roll_result == 3 => true,
            3 if roll_result == 4 => true,
            4 if roll_result == 6 => true,
            5 if roll_result == 8 => true,
            6 if roll_result == 10 => true,
            7 if roll_result == 12 => true,
            8 if roll_result == 20 => true,
            9 if roll_result == 100 => true,
            _ => false,
        };
        if crit {
            println!(
                "{} {}",
                "A simulated die roll: CRIT!".green(),
                roll_result.to_string().green(),
            )
        } else if roll_result == 1 {
            println!(
                "{} {}",
                "A simulated die roll: CRIT LOOSE!".red(),
                roll_result.to_string().red(),
            )
        } else {
            println!(
                "{} {}",
                "A simulated die roll: ".blue(),
                roll_result.to_string().blue(),
            )
        }
    } else {
        println!(
            "{} {}",
            "A simulated die roll: ".magenta(),
            roll_result.to_string().magenta(),
        )
    }
}
