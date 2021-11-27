use nix::sys::wait::*;
use nix::unistd::{execv, fork, ForkResult};
use std::ffi::CString;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let command: Vec<&str> = input.trim().split_whitespace().collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => match waitpid(child, None) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("exited: pid={:?}, status={:?}", pid, status);
            }
            _ => println!("exit"),
        },
        Ok(ForkResult::Child) => {
            let dir = CString::new(command[0].to_string()).unwrap();
            let arg = CString::new(command[1].to_string()).unwrap();
            execv(&dir, &[&dir, &arg]).unwrap();
        }
        Err(_) => panic!("Error"),
    }
}
