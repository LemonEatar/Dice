/*
    Made By @Lemonater aka Jakob
    Pls Help Me lol
*/
use rand::Rng;
use std::io;
//use std::io::stdin;
use std::io::Write;

static mut ONE: i32 = 0;
static mut TWO: i32 = 0;
static mut THREE: i32 = 0;
static mut FOUR: i32 = 0;
static mut FIVE: i32 = 0;
static mut SIX: i32 = 0;

fn main() {
    let turns: i32 = input("How many turns? ")
        .expect("Error")
        .parse::<i32>()
        .expect("Failed to read turns");

    for _ in 0..turns {
        dice();
    }
    unsafe {
        println!("Ones/ Nummer Eins: {}", ONE);
        println!("Twos/ Nummer Zwei: {}", TWO);
        println!("Threes/ Nummer Drei: {}", THREE);
        println!("Fours/ Nummer Vier: {}", FOUR);
        println!("Fives/ Nummer Fünf: {}", FIVE);
        println!("Sixes/ Nummer Sechs: {}", SIX);
    }
    fn dice() {
        let dice: i32 = rand::thread_rng().gen_range(1..7);
        println!("{}", dice);

        unsafe {
            if dice == 1 {
                ONE += 1;
            } else if dice == 2 {
                TWO += 1;
            } else if dice == 3 {
                THREE += 1;
            } else if dice == 4 {
                FOUR += 1;
            } else if dice == 5 {
                FIVE += 1;
            } else if dice == 6 {
                SIX += 1;
            } else {
                println!("Well this went as wrong as the db!");
            }
        }
    }
    let _end = input("End ").expect("Error");
    println!("{}", _end)
}

fn input(user_message: &str) -> io::Result<String> {
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}

// NOICE
