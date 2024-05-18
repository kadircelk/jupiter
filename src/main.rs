/// Jupiter is a minimal, very fast, and infinity customizable greeter for any shell.
/// Report issues here: https://github.com/kadirceliky/jupiter/issues
/// This project is licensed under the GNU Public License v3.0.
/// You can find the license here: https://github.com/kadirceliky/jupiter/blob/master/LICENSE

pub mod config;
pub mod style;
pub mod ui;

fn main() {
    ui::run();
}
