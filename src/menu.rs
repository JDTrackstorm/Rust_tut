#[derive(Debug)]
enum Aktion {
    Angreifen,
    Heilen,
    Status,
    Beenden
}
pub fn menu() {
    loop {
        println!("\nWas möchtest du tun?");
        println!("[1] Angreifen");
        println!("[2] Heilen");
        println!("[3] Status anzeigen");
        println!("[4] Spiel Beenden");

        let action = match read() {
            1 => Aktion::Angreifen,
            2 => Aktion::Heilen,
            3 => Aktion::Status,
            4 => Aktion::Beenden,
            _ => {
                println!("Bitte wähle 1 bis 4!");
                continue;
            }
        };


        match action {
            Aktion::Angreifen => println!("Du greifst den Gegner an!"),
            Aktion::Heilen => println!("Du heilst dich selbst."),
            Aktion::Status => println!("Dein Status wird angezeigt..."),
            Aktion::Beenden => {
                println!("Spiel wird beendet. Tschüss!");
                break;
            }
        }
    }
}
fn read() -> u8{
    let action: u8 = loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let action_in: u8 = match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    };
    action
}











// let action: Aktion = loop {
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//
//     let action_in: u8 = match input.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Please enter a number!");
//             continue;
//         }
//     };
//
//     match action_in {
//         1 => break Aktion::Angreifen,
//         2 => break Aktion::Heilen,
//         3 => break Aktion::Status,
//         4 => break Aktion::Beenden,
//         _ => println!("Please enter a number!")
//     };
// };