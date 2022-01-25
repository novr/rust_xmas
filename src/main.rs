use rand::seq::SliceRandom;
use std::env;

const SS: char = '\u{2605}';
const SF: char = '\u{FF0F}';
const SB: char = '\u{FF3C}';
const SL: char = '\u{005e}';
const SO: [char; 19] = [
    '\u{0069}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}',
    '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{0020}', '\u{2E1B}', '\u{2042}', '\u{2E2E}',
    '&', '@', '\u{FF61}',
];
const OC: [i32; 6] = [21, 33, 34, 35, 36, 37];

fn t(h: i32) {
    let mut rng = rand::thread_rng();
    for _i in 0..h {
        print!(" ");
    }
    println!(" \x1b[33m{}", SS);
    let m = h * 2 - 1;
    for l in 1..=h {
        let o = l * 2 - 2;
        let s = (m - o) / 2;
        for _i in 0..s {
            print!(" ");
        }
        print!(" \x1b[32m{}", SF);
        for _i in 0..o {
            print!(
                "\x1b[{}m{}\x1b[0m",
                OC.choose(&mut rng).unwrap(),
                SO.choose(&mut rng).unwrap()
            );
        }
        println!("\x1b[32m{}", SB);
    }
    print!(" ");
    for _i in 1..h {
        print!("\x1b[32m{}", SL);
    }
    print!("|  |");
    for _i in 1..h {
        print!("\x1b[32m{}", SL);
    }
    if h > 10 {
        println!();
        for _i in 0..h {
            print!(" ");
        }
        print!("|  |");
    }
    println!("\x1b[0m")
}

fn help() {
    println!("usage: num <int>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            t(20);
        }
        2 => {
            let number: i32 = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: not an integer");
                    help();
                    return;
                }
            };
            t(number);
        }
        _ => {
            help();
        }
    }
    println!("Happy Rust Xmas");
}
