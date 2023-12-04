use std::fs::File;
use std::io::{BufRead,BufReader};

// how to read a file line by line w/ buffer
fn example1() -> Result<(), std::io::Error> {
    let file = File::open("example1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line?); // stores each line in String
    }
    Ok(())
}

fn advent1() -> Result<(), std::io::Error> {
    let mut t = 0;
    let mut n = 0;
    
    let file = File::open("advent1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        n += 1; // n lines
        let number = get_number(line?);
        t += number; // sum of numbers
    }
    println!("read through all {} lines, sum of numbers is {}", n, t);
    Ok(())
}

// return 2 digit number from a string containing numbers
fn get_number(input: String) -> u32 {
    let mut first_digit = 0_u32;
    let mut last_digit = 0_u32;
    let mut output_str = String::from("");
    let test_chars = input.chars();
    for c in test_chars {
        if c.is_numeric() {
            if first_digit == 0 {
                output_str.push(c);
                first_digit = c.to_digit(10).unwrap();
            }
            else {
                last_digit = c.to_digit(10).unwrap();
            }
        }
    }
    // some lines have only 1 number
    if last_digit == 0 {last_digit = first_digit}
    
    //nummbers are made with 2 digits
    let last_digit = last_digit.to_string();
    output_str.push_str(&last_digit);
    // return result
    output_str.parse::<u32>().unwrap()
}

fn main() {

    //advent1(v);
    let test1 = get_number("as3kk88111s2".to_string());
    println!("{}", test1);

    advent1();
    
}
