use std::io::prelude::*;
use std::process::{Command, Stdio};

static FILE: &'static str =  "example3pipes/main.rs";
static WORD: &'static str = "process";

//cat main.rs | grep  process | sort
fn main() {
     // generamos el comando cat en un proceso hijo
    let cat_cmd_process = match Command::new("cat").arg(FILE)  
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn() {
                            Err(error) => panic!("couldn't spawn cat command: {}", error),
                            Ok(process) => process,
                        };

    //el proceso grep toma como imput la salida del proceso cat 
    let grep_cmd_process = match Command::new("grep").arg(WORD)
                        .stdin(cat_cmd_process.stdout.unwrap())
                        .stdout(Stdio::piped())
                        .spawn() {
                            Err(error) => panic!("couldn't spawn grep command: {}", error),
                            Ok(process) => process,
                        };
    
    //obtenemos el resultado de ejecutar cat `main.rs | grep  process'
    let mut grep_output = String::new();
    grep_cmd_process.stdout.unwrap().read_to_string(&mut grep_output)
                            .unwrap_or_else(|e| {
                                panic!("failed to execute process: {}", e) 
                            });

    let sort_cmd_process = match Command::new("sort")  
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn() {
                            Err(error) => panic!("couldn't spawn sort command: {}", error),
                            Ok(process) => process,
                        };

    //el proceso sort toma como imput la salida del proceso cat 
    match sort_cmd_process.stdin.unwrap().write_all(grep_output.as_bytes()) {
        Err(error) => panic!("couldn't write to sort stdin: {}", error),
        Ok(_) => println!("sent argument to sort command"),
    }

    let mut output = String::new();
    match sort_cmd_process.stdout.unwrap().read_to_string(&mut output) {
        Err(e) => panic!("couldn't read wc stdout: {}", e),
        Ok(_) => println!("sort responded with:\n{}", output),
    }
                    
   
}