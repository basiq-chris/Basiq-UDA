use std::fmt::{Debug, Display};

pub fn print_debug<O: Debug + Display>(msg: O) {
    println!("DEBUG: {}", msg);
}

pub fn print_info<O: Debug + Display>(msg: O) {
    println!("INFO: {}", msg);
}

pub fn print_warning<O: Debug + Display>(msg: O) {
    println!("WARN: {}", msg);
}

pub fn print_error<O: Debug + Display>(msg: O) {
    println!("ERROR: {}", msg);
}