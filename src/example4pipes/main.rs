use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::process;

fn main() {

    //Ejecutamos nuestro main.rs en un proceso
    let mut process = match Command::new("../../target/debug/chat")
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped()) 
                            .spawn() {
                                Err(error) => panic!("couldn't spawn command: {}", error),
                                Ok(process) => process,
                            };

    
    //Enviamos informacion al proceso recien creado
    let msg = format!("HOLAAAAAA DESDE EL PROCESO {}\n", process::id());
    match process.stdin.as_mut().unwrap().write_all(msg.as_bytes()) {
        Err(error) => panic!("couldn't write to sort stdin: {}", error),
        Ok(_) => println!("sent argument to program"),
    }

    //Leemos el output
    let mut output = String::new();
    match process.stdout.unwrap().read_to_string(&mut output) {
        Err(e) => panic!("couldn't read stdout: {}", e),
        Ok(_) => println!("program responded with:\n{}", output),
    }

      
     


}