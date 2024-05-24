#[cfg(debug_assertions)]
use crate::{print, println};

// https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
// escape sequences:

#[allow(dead_code)]
pub const CURSOR_UP: &str = "\x1B[A";
#[allow(dead_code)]
pub const CURSOR_DOWN: &str = "\x1B[B";
#[allow(dead_code)]
pub const CURSOR_RIGHT: &str = "\x1B[C";
#[allow(dead_code)]
pub const CURSOR_LEFT: &str = "\x1B[D";

#[allow(dead_code)]
pub const RESET: &str = "\x1B[0m";

#[allow(dead_code)]
pub const FG_BLACK: &str = "\x1B[30m";
#[allow(dead_code)]
pub const FG_RED: &str = "\x1B[31m";
#[allow(dead_code)]
pub const FG_GREEN: &str = "\x1B[32m";
#[allow(dead_code)]
pub const FG_YELLOW: &str = "\x1B[33m";
#[allow(dead_code)]
pub const FG_BLUE: &str = "\x1B[34m";
#[allow(dead_code)]
pub const FG_MAGENTA: &str = "\x1B[35m";
#[allow(dead_code)]
pub const FG_CYAN: &str = "\x1B[36m";
#[allow(dead_code)]
pub const FG_WHITE: &str = "\x1B[37m";
#[allow(dead_code)]
pub const FG_BRIGHT_BLACK: &str = "\x1B[90m";
#[allow(dead_code)]
pub const FG_BRIGHT_RED: &str = "\x1B[91m";
#[allow(dead_code)]
pub const FG_BRIGHT_GREEN: &str = "\x1B[92m";
#[allow(dead_code)]
pub const FG_BRIGHT_YELLOW: &str = "\x1B[93m";
#[allow(dead_code)]
pub const FG_BRIGHT_BLUE: &str = "\x1B[94m";
#[allow(dead_code)]
pub const FG_BRIGHT_MAGENTA: &str = "\x1B[95m";
#[allow(dead_code)]
pub const FG_BRIGHT_CYAN: &str = "\x1B[96m";
#[allow(dead_code)]
pub const FG_BRIGHT_WHITE: &str = "\x1B[97m";
#[allow(dead_code)]

pub const FG_RESET: &str = "\x1B[39m";
#[allow(dead_code)]
pub const BG_BLACK: &str = "\x1B[40m";
#[allow(dead_code)]
pub const BG_RED: &str = "\x1B[41m";
#[allow(dead_code)]
pub const BG_GREEN: &str = "\x1B[42m";
#[allow(dead_code)]
pub const BG_YELLOW: &str = "\x1B[43m";
#[allow(dead_code)]
pub const BG_BLUE: &str = "\x1B[44m";
#[allow(dead_code)]
pub const BG_MAGENTA: &str = "\x1B[45m";
#[allow(dead_code)]
pub const BG_CYAN: &str = "\x1B[46m";
#[allow(dead_code)]
pub const BG_WHITE: &str = "\x1B[47m";
#[allow(dead_code)]
pub const BG_BRIGHT_BLACK: &str = "\x1B[100m";
#[allow(dead_code)]
pub const BG_BRIGHT_RED: &str = "\x1B[101m";
#[allow(dead_code)]
pub const BG_BRIGHT_GREEN: &str = "\x1B[102m";
#[allow(dead_code)]
pub const BG_BRIGHT_YELLOW: &str = "\x1B[103m";
#[allow(dead_code)]
pub const BG_BRIGHT_BLUE: &str = "\x1B[104m";
#[allow(dead_code)]
pub const BG_BRIGHT_MAGENTA: &str = "\x1B[105m";
#[allow(dead_code)]
pub const BG_BRIGHT_CYAN: &str = "\x1B[106m";
#[allow(dead_code)]
pub const BG_BRIGHT_WHITE: &str = "\x1B[107m";
#[allow(dead_code)]
pub const BG_RESET: &str = "\x1B[49m";

#[cfg(debug_assertions)]
pub fn test_colors() {
    const FGS: [&str; 16] = [
        FG_BLACK,
        FG_RED,
        FG_GREEN,
        FG_YELLOW,
        FG_BLUE,
        FG_MAGENTA,
        FG_CYAN,
        FG_WHITE,
        FG_BRIGHT_BLACK,
        FG_BRIGHT_RED,
        FG_BRIGHT_GREEN,
        FG_BRIGHT_YELLOW,
        FG_BRIGHT_BLUE,
        FG_BRIGHT_MAGENTA,
        FG_BRIGHT_CYAN,
        FG_BRIGHT_WHITE,
    ];
    const BGS: [&str; 16] = [
        BG_BLACK,
        BG_RED,
        BG_GREEN,
        BG_YELLOW,
        BG_BLUE,
        BG_MAGENTA,
        BG_CYAN,
        BG_WHITE,
        BG_BRIGHT_BLACK,
        BG_BRIGHT_RED,
        BG_BRIGHT_GREEN,
        BG_BRIGHT_YELLOW,
        BG_BRIGHT_BLUE,
        BG_BRIGHT_MAGENTA,
        BG_BRIGHT_CYAN,
        BG_BRIGHT_WHITE,
    ];

    println!("test: colors escape sequences");
    println!("all:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}", background, foreground, RESET);
        }
        println!();
    }
    println!("foreground reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}b{}", background, foreground, FG_RESET, RESET);
        }
        println!();
    }
    println!("background reset:");
    for background in BGS {
        for foreground in FGS {
            print!("{}{}a{}b{}", background, foreground, BG_RESET, RESET);
        }
        println!();
    }
    println!("blue hello:");
    {
        println!("\x1B");
        println!("[");
        println!("3");
        println!("4");
        println!("m");
        println!("hello");
        println!("\x1B");
        println!("[");
        println!("0");
        println!("m");
        println!("hello");
    }
    println!("end test: colors escape sequences");
}
