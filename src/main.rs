use nix::sys::wait::*;
use nix::unistd::{execv, fork, ForkResult};
use std::ffi::CString;
fn main() {
    match unsafe{fork()}  {
        Ok(ForkResult::Parent{child, ..})=> {
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(pid, status))=> {
                    println!("exited: pid={:?}, status={:?}", pid, status);
                }
                Ok(WaitStatus::Signaled(pid, status, _)) => {
                    println!("signaled: pid={:?}, status={:?}", pid, status)
                }
                _ => println!("exit"),
            }

        }
        Ok(ForkResult::Child)=> {
            let dir = CString::new("/bin/ls".to_string()).unwrap();
            let arg = CString::new("-l".to_string()).unwrap();
            execv(&dir, &[&dir, &arg]).unwrap();
        }
        Err(_) => panic!("Error")
    }
}
