use std::io::prelude::*;
use std::process::{Command, Stdio};

static FILE: &'static str = "example2pipes/main.rs";
static WORD: &'static str = "process";

//cat main.rs | grep  process 
fn main() {

    // generamos el comando cat 
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
    
    //leemos el output del proceso grep
    let mut grep_output = String::new();
    grep_cmd_process.stdout.unwrap().read_to_string(&mut grep_output)
                            .unwrap_or_else(|e| {
                                panic!("failed to execute process: {}", e) 
                            });

    println!("command result: `{}`", grep_output)
  
}