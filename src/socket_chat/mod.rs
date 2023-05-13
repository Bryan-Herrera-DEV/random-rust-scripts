use std::io::{self, Write};

mod cliente;
mod servidor;

pub fn menu() {

    let mut input = String::new();
    print!("Por favor ingresar 'server' o 'client' para iniciar el programa: ");

    io::stdout().flush().unwrap(); // Para asegurarse de que el mensaje se imprime de inmediato
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "server" => {
            println!("Ejecutando acción para valor1...");
            servidor::run();
        },
        "client" => {
            println!("Ejecutando acción para valor2...");
            cliente::run();
        },
        _ => {
            println!("El valor que ingresaste no es válido.");
            std::process::exit(1);
        },
    }
}