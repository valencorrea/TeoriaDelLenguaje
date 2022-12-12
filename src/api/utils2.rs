use crate::api::constants::{ENTER_STR, ENTER_STR2};

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

pub fn delete_enters(input: &mut String) -> String {
    let mut output: String = String::new();
    for i in input.chars() {
        if i.to_string() != ENTER_STR && i.to_string() != ENTER_STR2 {
            output.push_str(&*i.to_string());
        }
    }
    output
}

// TODO OK
pub fn show_welcome() {
    println!("\nBienvenidos al Sokoban!\n");
    println!("El objetivo del juego es empujar cada caja a un objetivo. ¡Suerte!\n");
    show_commands();
}

// TODO OK
pub fn show_goodbye() {
    println!("Gracias por jugar! Nos vemos!");
}

// TODO OK
// todo agregar h de ayuda y que muestre de nuevo los comandos
pub fn show_commands() {
    println!("Comandos validos:");
    println!("\tMOVE A - LEFT");
    println!("\ttMOVE W - UP");
    println!("\ttMOVE D - RIGHT");
    println!("\ttMOVE S - DOWN");
    println!("\ttQUIT - QUIT");
    println!("\n");
}

// TODO OK
pub fn show_victory() {
    println!("\nFelicitaciones!\nHas vencido el juego. Gracias por jugar.\n");
}

// TODO OK
pub fn invalid_command() {
    println!("Comando invalido.");
    show_commands();
    ask_for_command();
}

// TODO OK
pub fn ask_for_command() {
    println!("Escribe tu movimiento o QUIT para cerrar el juego:")
}
