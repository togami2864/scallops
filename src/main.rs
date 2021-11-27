use nix::sys::wait::*;
use nix::unistd::{execv, fork, ForkResult};
use std::ffi::CString;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts.next().unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => match waitpid(child, None) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("exited: pid={:?}, status={:?}", pid, status);
            }
            _ => println!("exit"),
        },
        Ok(ForkResult::Child) => {
            let dir = CString::new(command.to_string()).unwrap();
            let arg = CString::new(args.to_string()).unwrap();
            execv(&dir, &[&dir, &arg]).unwrap();
        }
        Err(_) => panic!("Error"),
    }
}
