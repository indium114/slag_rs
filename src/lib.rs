use colored::*;
use std::fmt;

mod prefixes;

fn generic_print(prefix: ColoredString, msg: impl fmt::Display) {
    println!("{prefix} {msg}")
}
