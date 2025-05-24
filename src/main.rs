mod shell;
use crate::shell::shell;
mod parser;
mod executor;
mod utils;
fn main(){
    shell::shell();
}