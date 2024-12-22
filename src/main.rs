use std::{
    env,
    io::{self, Read},
};

use atty;
use termsize::Size;

const CHARS: [char; 16] = [
    '─', '│', '╭', '╮', '╯', '╰', '┼', '╴', '╵', '╶', '╷', '├', '┤', '┬', '┴', ' ',
];
// const CHARS: [char; 8] = ['═', '│', '╞', '╡', '╤', '╧', '╪', ' '];
// const CHARS: [char; 8] = ['─', '│', '├', '┤', '┬', '┴', '┼', ' '];
// const CHARS: [char; 8] = ['═', '║', '╠', '╣', '╦', '╩', '╬', ' '];

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if atty::is(atty::Stream::Stdin) {
                println!("This is a sha256 previewer.");
            } else {
                let mut buf = Vec::new();
                io::stdin().lock().read_to_end(&mut buf).unwrap();

                print_pattern(&buf);
            }
        }
        2 => {
            print_pattern(&args[1].bytes().collect::<Vec<_>>());
        }
        _ => {
            println!("Too many arguments.");
        }
    }
}

fn print_pattern(input: &[u8]) {
    let hash = sha256::digest(input);
    let bytes = hash.bytes().collect::<Vec<_>>();

    let start_i = termsize::get()
        .map(|size: Size| (size.cols - 12) / 2)
        .unwrap_or(2);

    println!("\n");
    for (i, bytes) in bytes.chunks_exact(2).enumerate() {
        if i % 4 == 0 {
            print!("{}", " ".repeat(start_i as usize));
        }
        if let &[a, b] = bytes {
            let (mut a, mut b) = (a, b);
            for _ in 0..3 {
                print!("{}", CHARS[(a % 8 + b % 8) as usize]);
                a /= 8;
                b /= 8;
            }
        };
        if (i + 1) % 4 == 0 {
            println!("");
        }
    }
    println!("\n");
}
