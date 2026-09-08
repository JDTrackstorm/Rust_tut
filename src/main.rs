mod calc;
mod player_profile;
mod menu;

fn main() {
    calc::calc();
    player_profile::pl_prof();
    menu::menu();
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}