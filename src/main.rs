extern crate getopts;
extern crate termion;

mod socket_chat;
use getopts::Options;
use std::collections::HashMap;
use std::env;
use termion::color;

fn bienvenida() {
    println!(
        "{}{}=========================================={}",
        termion::color::Fg(color::LightCyan),
        termion::style::Bold,
        termion::style::Reset
    );

    println!(
        "{}{}   Welcome to Random Rust scripts!   {}{}",
        termion::color::Fg(color::LightCyan),
        termion::style::Bold,
        termion::style::Reset,
        termion::color::Fg(color::Reset)
    );

    println!(
        "{}{}    Created by Bryan Herrera Dev   {}{}",
        termion::color::Fg(color::LightMagenta),
        termion::style::Bold,
        termion::style::Reset,
        termion::color::Fg(color::Reset)
    );

    println!(
        "{}{}=========================================={}",
        termion::color::Fg(color::LightCyan),
        termion::style::Bold,
        termion::style::Reset
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("", "help", "Muestra este mensaje de ayuda");
    opts.optflag("", "chat_websocket", "Inicia el chat a través de websocket");

    bienvenida();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            eprintln!(
                "{}{}No se proporcionaron opciones válidas. Usa --help para obtener ayuda.{}",
                termion::color::Fg(color::Red),
                termion::style::Bold,
                termion::style::Reset
            );
            return;
        }
    };

    let mut handlers: HashMap<String, Box<dyn Fn()>> = HashMap::new();
    handlers.insert(
        "help".to_string(),
        Box::new(|| {
            let brief = format!("Uso: {} [opciones]", args[0]);
            print!("{}", opts.usage(&brief));
        }),
    );
    handlers.insert(
        "chat_websocket".to_string(),
        Box::new(|| {
            println!("Iniciando chat a través de websocket...");
            socket_chat::menu()
        }),
    );

    for option in ["help", "chat_websocket"].iter() {
        if matches.opt_present(option) {
            if let Some(handler) = handlers.get(*option) {
                handler();
                return;
            } else {
                eprintln!(
                    "{}{}Opción no implementada.{}",
                    termion::color::Fg(color::Red),
                    termion::style::Bold,
                    termion::style::Reset
                );
                return;
            }
        }
    }
    eprintln!(
        "{}{}No se proporcionaron opciones válidas. Usa --help para obtener ayuda.{}",
        termion::color::Fg(color::Red),
        termion::style::Bold,
        termion::style::Reset
    );
}
