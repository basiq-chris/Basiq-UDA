use std::fmt::{Debug, Display};
use colored::{self, Colorize};

pub fn print_debug<O: Debug + Display>(msg: O) {
    println!("{}: {}", "DEBUG".truecolor(0x33, 0x33, 0x33), msg);
}

pub fn print_info<O: Debug + Display>(msg: O) {
    println!("{}: {}", "INFO".white(), msg);
}

pub fn print_warning<O: Debug + Display>(msg: O) {
    println!("{}: {}", "WARN".bright_yellow(), msg);
}

pub fn print_error<O: Debug + Display>(msg: O) {
    println!("{}: {}", "ERROR".bright_red(), msg);
}