use std::env;

const CHARS: [char; 8] = ['─', '│', '╭', '╮', '╯', '╰', '┼', ' '];
// const CHARS: [char; 8] = ['═', '│', '╞', '╡', '╤', '╧', '╪', ' '];
// const CHARS: [char; 8] = ['─', '│', '├', '┤', '┬', '┴', '┼', ' '];
// const CHARS: [char; 8] = ['═', '║', '╠', '╣', '╦', '╩', '╬', ' '];

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("This is a sha256 previewer.");
        }
        2 => {
            let hash = sha256::digest(&args[1]);
            for (i, mut byte) in hash.bytes().enumerate() {
                for _ in 0..3 {
                    print!("{}", CHARS[(byte % 8) as usize]);
                    byte /= 8;
                }
                if (i + 1) % 8 == 0 {
                    println!("");
                }
            }
            println!("")
        }
        _ => {
            println!("Too many arguments.");
        }
    }
}
