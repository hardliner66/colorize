use std::io::{self, BufRead};

const RED: (u8, u8, u8) = (255, 0, 0);
const ORANGE: (u8, u8, u8) = (255, 127, 0);
const YELLOW: (u8, u8, u8) = (255, 255, 0);
const GREEN: (u8, u8, u8) = (0, 255, 0);
const BLUE: (u8, u8, u8) = (0, 127, 255);
const INDIGO: (u8, u8, u8) = (75, 0, 130);
const VIOLET: (u8, u8, u8) = (148, 0, 211);

fn main() {
    let colors = [RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET];

    let stdin = io::stdin();

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("error reading stdin: {err}");
                break;
            }
        };

        let (r, g, b) = colors[i % colors.len()];

        println!("\x1b[38;2;{r};{g};{b}m{line}\x1b[0m");
    }
}
