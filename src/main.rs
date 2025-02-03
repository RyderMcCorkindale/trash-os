
use std::io;
// use std::{thread, time};
// use chrono::prelude::*;

fn intake(x: &mut String) -> &String {
    io::stdin()
        .read_line(x)
        .expect("couldn't read line");
    x
}

#[allow(unused_mut)]
fn main() {
    // add an option for choosing whether to have 12 hour or military time, also to set timezone
    // and date format
    //Terminal Rust Ascii SHell Operating System
    //dump command that dumps input


    println!("[🗑] Welcome to TRASH OS! [🗑]");

    let mut input = String::new();
    loop {
        intake(&mut input);
        let mut _cmd = input.split_whitespace().next().unwrap();


    }
    //add a light that shows when a program is running
}


