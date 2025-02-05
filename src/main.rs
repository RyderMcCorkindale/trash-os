
use std::io;
use std::io::Write;
// use std::{thread, time};
// use chrono::prelude::*;

fn intake(x: &mut String) -> String {
    //standard rust input
    io::stdin()
        .read_line(x)
        .expect("couldn't read line");

    //trims the excess newline when you press enter
    if x.ends_with('\n') {
        x.pop();
        if x.ends_with('\r') {
            x.pop();
        }
    }
    x.to_string()
}

#[allow(unused_mut)]
fn main() {
    // add an option for choosing whether to have 12 hour or military
    //time, also to set timezone
    // and date format
    //Terminal Rust Ascii SHell Operating System
    //commands should be formated >cmd type
    //option to switch between online terminal friendly and vim-like
    //terminal friendly
    println!("=============================|");
    println!("[🗑] welcome to TRASH OS");
    println!("[🗑] for viewing a list of commands, type >ls cmd (WIP)");
    println!("[🗑] you can also see a guide of the OS with >guide (WIP)");
    println!("[🗑] run >toggle formatio if you are using an online compiler");
    println!("=============================|");

    let mut input = String::new();
    let mut formatio = false;
    print!("landfill:~$ ");
    loop {

        io::stdout().flush().unwrap();
        //input function with newline trimming
        intake(&mut input);

        //toggles how the input is formated
        match (input.as_str(), formatio) {
            (">toggle formatio", false) => formatio = true,
            (&_, false) => print!("landfill:~$ "),
            (">toggle formatio", true) => formatio = false,
            (&_, true) => print!("\nlandfill:~$ {} ", input),
        };

        //wipes the old input for new input
        input.clear();


    }
}





