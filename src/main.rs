mod mat;

use std::{
    env,
    fs::File,
    io::{self, Read},
};

use colored::Colorize;
use termsize::Size;

use mat::Mat;

const CHARS: [char; 16] = [
    '─', '│', '╭', '╮', '╯', '╰', '┼', '╴', '╵', '╶', '╷', '├', '┤', '┬', '┴', ' ',
];

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    MissingCommand,
    MissingArgument,
    ReadFile(io::Error),
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("This is a sha256 previewer.\n");
        println!("Available commands are:\n\n");
        println!(" string [string]                        preview sha256 of a string");
        println!(
            " compare_strings [string1] [string2]    preview sha256 of two strings for comparison"
        );
        println!(" file [path]                            preview sha256 of a file");
        println!(
            " compare_files [path1] [path2]          preview sha256 of two files for comparison"
        );
    } else {
        let command = args.get(1).ok_or(Error::MissingCommand)?.as_str();

        match command {
            "string" => print_single_pattern(args.get(2).ok_or(Error::MissingArgument)?.as_bytes()),
            "compare_strings" => print_and_compare_patterns(
                args.get(2).ok_or(Error::MissingArgument)?.as_bytes(),
                args.get(3).ok_or(Error::MissingArgument)?.as_bytes(),
            ),
            "file" => {
                let path = args.get(2).ok_or(Error::MissingArgument)?;

                let mut buf = Vec::new();
                File::open(path)
                    .map_err(Error::ReadFile)?
                    .read_to_end(&mut buf)
                    .map_err(Error::ReadFile)?;

                print_single_pattern(&buf);
            }
            "compare_files" => {
                let path1 = args.get(2).ok_or(Error::MissingArgument)?;
                let path2 = args.get(3).ok_or(Error::MissingArgument)?;

                let mut buf1 = Vec::new();
                File::open(path1)
                    .map_err(Error::ReadFile)?
                    .read_to_end(&mut buf1)
                    .map_err(Error::ReadFile)?;

                let mut buf2 = Vec::new();
                File::open(path2)
                    .map_err(Error::ReadFile)?
                    .read_to_end(&mut buf2)
                    .map_err(Error::ReadFile)?;

                print_and_compare_patterns(&buf1, &buf2);
            }
            _ => {
                println!("Unknown command");
            }
        }
    }

    Ok(())
}

const PATTERN_WIDTH: usize = 12;
const PATTERN_HEIGHT: usize = 8;

fn create_pattern(hash_bytes: &[u8]) -> Mat<usize> {
    let mut pattern = Mat::filled_with(0, PATTERN_WIDTH, PATTERN_HEIGHT);

    let (mut i, mut j) = (0, 0);

    for (k, bytes) in hash_bytes.chunks_exact(2).enumerate() {
        if let &[a, b] = bytes {
            let (mut a, mut b) = (a, b);
            for _ in 0..3 {
                pattern.set((i, j), (a % 8 + b % 8) as usize);
                i += 1;
                i %= PATTERN_WIDTH;
                a /= 8;
                b /= 8;
            }
        };
        if (k + 1) % 4 == 0 {
            j += 1;
            j %= PATTERN_HEIGHT;
        }
    }

    pattern
}

fn print_single_pattern(input: &[u8]) {
    let hash = sha256::digest(input);
    let hash_bytes = hash.bytes().collect::<Vec<_>>();

    let start_column = termsize::get()
        .map(|size: Size| (size.cols as usize - PATTERN_WIDTH) / 2)
        .unwrap_or(2);

    let pattern = create_pattern(&hash_bytes);

    println!("\n");
    for j in 0..PATTERN_HEIGHT {
        print!("{}", " ".repeat(start_column));
        for i in 0..PATTERN_WIDTH {
            print!("{}", CHARS[*pattern.get((i, j))]);
        }
        println!();
    }
    println!("\n");
}

fn print_and_compare_patterns(input1: &[u8], input2: &[u8]) {
    let hash1 = sha256::digest(input1);
    let hash_bytes1 = hash1.bytes().collect::<Vec<_>>();

    let hash2 = sha256::digest(input2);
    let hash_bytes2 = hash2.bytes().collect::<Vec<_>>();

    let start_column = termsize::get()
        .map(|size: Size| (size.cols as usize - 2 * PATTERN_WIDTH - 4) / 2)
        .unwrap_or(2);

    let pattern1 = create_pattern(&hash_bytes1);
    let pattern2 = create_pattern(&hash_bytes2);

    println!("\n");
    for j in 0..PATTERN_HEIGHT {
        print!("{}", " ".repeat(start_column as usize));
        for i in 0..PATTERN_WIDTH {
            let c1 = CHARS[*pattern1.get((i, j))];
            let c2 = CHARS[*pattern2.get((i, j))];
            print!(
                "{}",
                if c1 == c2 {
                    c1.to_string().normal()
                } else {
                    c1.to_string().red()
                }
            );
        }
        print!("    ");
        for i in 0..PATTERN_WIDTH {
            let c1 = CHARS[*pattern1.get((i, j))];
            let c2 = CHARS[*pattern2.get((i, j))];
            print!(
                "{}",
                if c1 == c2 {
                    c2.to_string().normal()
                } else {
                    c2.to_string().red()
                }
            );
        }
        println!();
    }
    println!("\n");
}
