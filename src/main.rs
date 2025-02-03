
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
    // add an option for choosing whether to have 12 hour or military time, also to set timezone
    // and date format
    //Terminal Rust Ascii SHell Operating System
    //commands should be formated >cmd type
    println!("=============================|");
    println!("[🗑] welcome to TRASH OS!");
    println!("[🗑] for viewing a list of commands, type >ls cmd");
    println!("[🗑] you can also view a guide of the OS with >guide");
    println!("=============================|");

    let mut input = String::new();
    loop {
        print!("landfill:~$ ");
        io::stdout().flush().unwrap();
        intake(&mut input);
        input.clear();


        // let mut _cmd = input.split_whitespace().next().unwrap();




    }
    //add a light that shows when a program is running
}


