
use std::process::{Command, Stdio};

//ls
fn main() {

    //creamos un proceso hijo que ejecute el comando ls
    let child_cmd_process = match Command::new("ls")
                            .stdout(Stdio::piped()) 
                            .spawn() {
                                Err(error) => panic!("couldn't spawn command: {}", error),
                                Ok(process) => process,
                            };
  
                            
    //esperamos a que termine para obtener el output                        
    let output = child_cmd_process.wait_with_output().expect("failed to wait on child");

    // std::process::Child  output struct { status, stdout, stderr }
    println!("\ncommand result:\n {:?}", output);
  
}