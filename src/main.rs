/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */

use std::io::{self, Read};
use clap::{Arg, App};

fn str_to_meow(input: &String) -> String {
    let bytes = input.as_bytes();
    let mut buffer = String::new();
    for byte in bytes {
        for i in 0..4 {
            let bits = byte >> (i*2) & 0x3;
            // println!("{}[{}] = {}", byte, i, bits);
            buffer.push(match bits {
                0 => 'M',
                1 => 'E',
                2 => 'O',
                3 => 'W',
                _ => panic!("what"),
            });
        }
    }
    buffer
}

fn meow_to_str(input: &String) -> Result<String, Box<dyn std::error::Error>> {
    let input = input.as_bytes().to_owned();
    let input: Vec<u8> = input.into_iter().filter(|c| match *c as char {
                'M' => true,
                'E' => true,
                'O' => true,
                'W' => true,
                _ => false
    }).collect();
    let input = String::from_utf8(input)?;
    let mut buffer = String::new();
    let meows = input.as_bytes().chunks_exact(4);
    for meow in meows {
        let mut byte: u8 = 0;
        for i in 0..4 {
            let bits = match meow[3-i] as char {
                'M' => 0,
                'E' => 1,
                'O' => 2,
                'W' => 3,
                _ => continue,
            };
            byte = (byte << 2) + bits;
            // println!("{} = {}", meow[i] as char, byte);
        }
        buffer.push(byte as char);
    }
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("MEOW")
        .arg(Arg::with_name("encode")
            .short("e")
            .long("encode")
            .conflicts_with("decode")
            .help("encode text to MEOW (default)"))
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .conflicts_with("encode")
            .help("decode MEOW into text (default is encode)"))
        .get_matches();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    // println!("{:?}", buffer.as_bytes());
    let decode = matches.is_present("decode");
    match decode {
        true => print!("{}", meow_to_str(&buffer)?),
        false => println!("{}", str_to_meow(&buffer)),
    };
    Ok(())
}
