use std::io;

struct profile{
    id: u32,
    name: String,
    date: struct date,
    place: String,
    data: String,
}

struct date{
    year: u16,
    month: u8,
    day: u8,
}

fn get_line() -> String{
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line")
}

fn main(){

    let mut line = String::new();

    line = get_line();

    plintln!("Input: {}",line);
}
    
