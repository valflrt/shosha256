mod mat;

use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{self, Read},
};

use colored::Colorize;
use sha2::{Digest, Sha256};
use termsize::Size;

use mat::Mat;

const CHARS: [char; 16] = [
    '─', '│', '╭', '╮', '╯', '╰', '┼', '╴', '╵', '╶', '╷', '├', '┤', '┬', '┴', ' ',
];

const COMMANDS: [&str; 4] = [
    "  string [str]                     preview sha256 hash of a string",
    "  file [path]                      preview sha256 hash of a file",
    "  compare_files [path1] [path2]    compare sha256 hashes of two files",
    "  check_file [path] [hash]         check sha256 of a file against given hash",
];

enum Error {
    MissingArgument,
    TooManyArguments,
    ComputeSha256(io::Error),
    DecodeHex(hex::FromHexError),
    ReadFile(io::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingArgument => {
                writeln!(f, "Missing argument")
            }
            Error::TooManyArguments => {
                writeln!(f, "Too many arguments")
            }
            Error::ComputeSha256(e) => {
                writeln!(f, "Could't compute sha256 hash: {}", e)
            }
            Error::DecodeHex(e) => {
                writeln!(f, "Couldn't decode hexadecimal sha256 hash string: {}", e)
            }
            Error::ReadFile(e) => {
                writeln!(f, "Could't read file: {}", e)
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if let Some(command) = args.get(1) {
        match command.as_str() {
            "string" => {
                if args.len() > 3 {
                    return Err(Error::TooManyArguments);
                }

                let string = args.get(2).ok_or(Error::MissingArgument)?;

                let hash = compute_sha256(string.as_bytes()).map_err(Error::ComputeSha256)?;

                print_pattern(&hash)
            }
            "file" => {
                if args.len() > 3 {
                    return Err(Error::TooManyArguments);
                }

                let path = args.get(2).ok_or(Error::MissingArgument)?;

                let hash = compute_sha256(File::open(path).map_err(Error::ReadFile)?)
                    .map_err(Error::ComputeSha256)?;

                print_pattern(&hash);
            }
            "compare_files" => {
                if args.len() > 4 {
                    return Err(Error::TooManyArguments);
                }

                let path1 = args.get(2).ok_or(Error::MissingArgument)?;
                let path2 = args.get(3).ok_or(Error::MissingArgument)?;

                let hash1 = compute_sha256(File::open(path2).map_err(Error::ReadFile)?)
                    .map_err(Error::ComputeSha256)?;
                let hash2 = compute_sha256(File::open(path1).map_err(Error::ReadFile)?)
                    .map_err(Error::ComputeSha256)?;

                compare_patterns(&hash1, &hash2);
            }
            "check_file" => {
                if args.len() > 4 {
                    return Err(Error::TooManyArguments);
                }

                let path = args.get(2).ok_or(Error::MissingArgument)?;
                let hash = args.get(3).ok_or(Error::MissingArgument)?;

                let hash1 = compute_sha256(File::open(path).map_err(Error::ReadFile)?)
                    .map_err(Error::ComputeSha256)?;
                let hash2 = hex::decode(hash).map_err(Error::DecodeHex)?;

                compare_patterns(&hash1, &hash2);
            }
            _ => {
                println!("\nUnknown command. Available commands are:");
                println!("{}", COMMANDS.join("\n"));
            }
        }
    } else {
        println!("\nshosha256: a sha256 previewer\n");
        println!("Usage:");
        println!("  command [command] [arguments]\n");
        println!("Available commands:");
        println!("{}", COMMANDS.join("\n"));
    }

    Ok(())
}

fn compute_sha256(mut reader: impl Read) -> io::Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; // 8kb buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_vec())
}

const PATTERN_WIDTH: usize = 12;
const PATTERN_HEIGHT: usize = 4;

fn print_pattern(hash: &[u8]) {
    let start_column = termsize::get()
        .map(|size: Size| (size.cols as usize - PATTERN_WIDTH) / 2)
        .unwrap_or(2);

    let pattern = create_pattern(&hash);

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

fn compare_patterns(hash1: &[u8], hash2: &[u8]) {
    let start_column = termsize::get()
        .map(|size: Size| (size.cols as usize - 2 * PATTERN_WIDTH - 2) / 2)
        .unwrap_or(2);

    let pattern1 = create_pattern(&hash1);
    let pattern2 = create_pattern(&hash2);

    println!("\n");
    for j in 0..PATTERN_HEIGHT {
        print!("{}", " ".repeat(start_column as usize));
        for i in 0..PATTERN_WIDTH {
            let c1 = CHARS[*pattern1.get((i, j))];
            let c2 = CHARS[*pattern2.get((i, j))];
            print!(
                "{}",
                if c1 == c2 {
                    c1.to_string().green()
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
                    c2.to_string().green()
                } else {
                    c2.to_string().red()
                }
            );
        }
        println!();
    }
    println!("\n");
}

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
