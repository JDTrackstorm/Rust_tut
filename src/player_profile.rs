use std::io;
use crate::clear_screen;

#[derive(Debug)]
enum Status {
    Online,
    Offline,
    Inactive,
}
struct Player{
    name: String,
    level: u8,
    status: Status,
}

pub fn pl_prof(){
    clear_screen();

    println!("Wie soll der Spieler heißen?");

    let mut name_in = String::new();
    std::io::stdin()
        .read_line(&mut name_in)
        .expect("Fehler beim Lesen");
    let name = name_in.trim().to_string();

    println!("Welches Level soll der Spieler haben?");
    let level: u8 = loop {
        let mut level_in = String::new();
        std::io::stdin()
        .read_line(&mut level_in)
        .expect("Fehler beim Lesen");

        match level_in.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Bitte gib eine gültige Zahl ein")
        }
    };

    println!("Ist der Spieler Online? Ja -> [1], nein -> [2], inaktiv -> [3]");
    let status: Status = loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Fehler beim Lesen");

        let status_in: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte gib eine gültige Zahl ein!");
                continue;
            }
        };

        match status_in {
            1 => break Status::Online,
            2 => break Status::Offline,
            3 => break Status::Inactive,
            _ => println!("Bitte wähle 1, 2 oder 3!"),
        }
    };


    let player = Player{
        name,
        level,
        status,
    };
    println!("\nDein erstellter Spieler:");
    println!("Name: {}", player.name);
    println!("Level: {}", player.level);
    println!("Status: {:?}", player.status);
}