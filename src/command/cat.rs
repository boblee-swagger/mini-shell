use std::process::{Command, Stdio};
use std::io::{self, Read};
use std::str::SplitWhitespace;

use crate::read_input;

pub fn cat(cmd: &str, arg: &mut SplitWhitespace) -> Result<String, io::Error> {    
    if arg.clone().count() == 0{
        loop {
            let output = read_input();
            println!("{}", output);
        }   
    }else {
        let mut cmd_child = Command::new(cmd)
        .args(arg)
        .stdout(Stdio::piped())
        .spawn()?; 
    
        let mut cmd_out = cmd_child.stdout.take().ok_or(io::Error::new(io::ErrorKind::Other, "Failed to open cmd stdout"))?;
    
        let mut output = String::new();
        cmd_out.read_to_string(&mut output)?;
    
        cmd_child.wait()?;
    
        Ok(output)
    } 
    
}