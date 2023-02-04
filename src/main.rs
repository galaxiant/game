mod interface;
mod models;

use std::mem::size_of_val;
use std::time::Instant;
use console::{Color, Style, style, StyledObject, Term};
use crate::models::resource_models::resource::{Resource};

fn main() {
    let term = Term::stdout();

    term.set_title("galaxiant");

    term.write_line(&*style("Hello").red().underlined().bold().to_string()).expect("Failed to write line.");
    println!("{}", style("Tigerros").underlined().italic());
    term.read_key().expect("Failed to read key.");
}