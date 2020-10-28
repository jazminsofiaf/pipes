
use nix::sys::signal;
use std::io::{self, BufRead};
use std::process;

//ignoramos la señal SIGPIPE "broken pipe" para que no se cierre el pipe
pub fn reset_signal_pipe_handler() {
    {
        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)
                .map_err(|e| println!("  error: {}", e.to_string())).unwrap();
        }
    }
}


fn main() {
    reset_signal_pipe_handler();
    println!("  pid:{} Running eco chat... ", process::id());
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).expect("Could not read line");
    print!("  Recibi el mensaje: {}",line)
}