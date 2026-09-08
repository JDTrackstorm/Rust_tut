use std::io;
use crate::clear_screen;

pub fn calc (){
    clear_screen();


    let mut history: Vec<String> = Vec::new();

    'main_loop: loop {
        println!("\nWas möchtest du tun? ([r]echnen, [h]istory anzeigen, [q]uit)");
        let mut command_in = String::new();
        io::stdin()
            .read_line(&mut command_in)
            .expect("Failed to read line");

        match command_in.trim().to_lowercase().as_str() {
            "q" => {
                println!("Programm wird beendet. Tschüss!");
                break 'main_loop;
            }
            "h" => {
                println!("\n--- Bisherige Rechnungen ---");
                for eintrag in &history {
                    println!("{}", eintrag);
                }
                println!("----------------------------\n");
                continue;
            }
            "r" => {
                println!("Dies ist ein Taschenrechner! Gib nun die erste Zahl ein.");

                let num1: f64 = loop {
                    let mut num1_in = String::new();

                    io::stdin()
                        .read_line(&mut num1_in)
                        .expect("Failed to read line");

                    match num1_in.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Schreibe bitte eine Zahl!"),
                    }
                };

                println!("Gib nun die zweite Zahl ein.");

                let num2: f64 = loop {
                    let mut num2_in = String::new();

                    io::stdin()
                        .read_line(&mut num2_in)
                        .expect("Failed to read line");

                    match num2_in.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Schreibe bitte eine Zahl!"),
                    }
                };

                println!("Wähle nun deinen Rechenoperator (+, -, *, /).");

                let op: char = loop {
                    let mut op_in = String::new();
                    io::stdin()
                        .read_line(&mut op_in)
                        .expect("Failed to read line");

                    match op_in.trim().chars().next() {
                        Some(c) => break c,
                        None => println!("Schreibe bitte einen Operator!"),
                    }
                };

                let ergebnis = match op {
                    '+' => num1 + num2,
                    '-' => num1 - num2,
                    '*' => num1 * num2,
                    '/' => num1 / num2,
                    _ => 0.0,
                };

                let eintrag = format!("{} {} {} = {}", num1, op, num2, ergebnis);
                println!("Deine Rechnung sieht so aus : {}.", eintrag);

                history.push(eintrag);
                for eintrag in &history {
                    println!("{}", eintrag);
                }
            }
            _ => {
                println!("Unbekannter Befehl! Bitte 'r', 'h' oder 'q' eingeben.");
                continue;
            }
        }
    }
}