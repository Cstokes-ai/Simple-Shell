mod shell;
use crate::shell::shell;
mod parser;
mod executor;
mod utils;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
fn main(){
    shell::shell();
    
}