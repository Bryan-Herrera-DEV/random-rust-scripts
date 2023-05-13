use std::io::{self,prelude::*, stdout, Write};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::process;

pub fn run() {
    let mut stdout = stdout();

    // Nos conectamos al servidor
    let mut stream = match TcpStream::connect("localhost:7878") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error al conectar al servidor: {}", e);
            return;
        }
    };

    // Pedimos al usuario que ingrese su nombre
    write!(stdout, "Por favor, ingrese su nombre de usuario: ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error al leer el nombre de usuario: {}", e);
            return;
        }
    };
    let username = input.trim().to_string();

    // Enviamos el nombre de usuario al servidor
    match stream.write(username.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error al enviar el nombre de usuario: {}", e);
            return;
        }
    };

    // Iniciamos un hilo para leer los mensajes del servidor
    let mut stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || loop {
        let mut buffer = [0; 512];
        match stream_clone.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    eprintln!("El servidor se cayó. Terminando el programa.");
                    process::exit(1);
                }
                let message = str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("{}", message);
            },
            Err(e) => {
                eprintln!("Error al leer desde el socket: {}", e);
                eprintln!("El servidor se cayó. Terminando el programa.");
                process::exit(1);
            }
        };
    });

    // Leemos los mensajes del usuario y los enviamos al servidor
    loop {
        write!(stdout, "> ").unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error al leer desde stdin: {}", e);
                return;
            }
        };
        
        match stream.write(input.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error al escribir en el socket: {}", e);
                return;
            }
        };
    }
}