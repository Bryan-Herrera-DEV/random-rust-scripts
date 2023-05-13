// Importamos las librerías necesarias
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Definimos una estructura para guardar a los clientes conectados
struct Clients {
    list: HashMap<String, TcpStream>,
}

impl Clients {
    // Constructor para la estructura Clients
    fn new() -> Clients {
        Clients {
            list: HashMap::new(),
        }
    }
}

// Función para manejar cada cliente conectado
fn handle_client(clients: Arc<Mutex<Clients>>, mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Leemos el nombre de usuario del cliente
    let bytes_read = match stream.read(&mut buffer) {
        Ok(bytes_read) => bytes_read,
        Err(e) => {
            eprintln!("Error al leer el nombre de usuario: {}", e);
            return;
        }
    };

    let username = String::from_utf8(buffer[..bytes_read].to_vec()).unwrap();

    // Añadimos el cliente a la lista de clientes conectados
    {
        let mut clients = clients.lock().unwrap();
        match stream.try_clone() {
            Ok(cloned_stream) => {
                clients.list.insert(username.clone(), cloned_stream);
                println!("Nuevo cliente conectado: {}", username); // log en el servidor
                let msg = format!("{} se ha unido al chat", username);
                for (_, client_stream) in clients.list.iter() {
                    let mut writable_stream =
                        client_stream.try_clone().expect("Failed to clone stream");
                    match writable_stream.write(msg.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error al enviar el mensaje: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Error al clonar el stream: {}", e);
                return;
            }
        }
    }

    loop {
        // Leemos el mensaje del cliente
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                eprintln!("Error al leer el mensaje: {}", e);
                let mut clients = clients.lock().unwrap();
                clients.list.remove(&username);
                return;
            }
        };

        // Si no se leyeron bytes, el cliente se desconectó
        if bytes_read == 0 {
            let mut clients = clients.lock().unwrap();
            clients.list.remove(&username);
            return;
        }

        // Creamos el mensaje a enviar a los otros clientes
        let msg = format!(
            "{}: {}",
            username,
            String::from_utf8(buffer[..bytes_read].to_vec()).unwrap()
        );

        // Enviamos el mensaje a todos los otros clientes
        let clients = clients.lock().unwrap();
        for (client_username, client_stream) in clients.list.iter() {
            if *client_username != username {
                let mut writable_stream =
                    client_stream.try_clone().expect("Failed to clone stream");
                match writable_stream.write(msg.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error al enviar el mensaje: {}", e),
                }
            }
        }
    }
}

pub fn run() {
    // Iniciamos el servidor en el puerto 7878
    let listener = match TcpListener::bind("localhost:7878") {
        Ok(listener) => {
            println!("Servidor en línea en localhost:7878");
            listener
        }
        Err(e) => {
            eprintln!("Error al iniciar el servidor: {}", e);
            return;
        }
    };

    let clients = Arc::new(Mutex::new(Clients::new()));

    // Aceptamos conexiones entrantes
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || handle_client(clients, stream));
            }
            Err(e) => eprintln!("Error al aceptar la conexión: {}", e),
        }
    }
}
