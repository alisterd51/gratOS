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
pub const SCROLL_UP: &str = "\x1B[S";
#[allow(dead_code)]
pub const SCROLL_DOWN: &str = "\x1B[T";

pub const RESET: &str = "\x1B[0m";

pub const FG_BLACK: &str = "\x1B[30m";
pub const FG_RED: &str = "\x1B[31m";
pub const FG_GREEN: &str = "\x1B[32m";
pub const FG_YELLOW: &str = "\x1B[33m";
pub const FG_BLUE: &str = "\x1B[34m";
pub const FG_MAGENTA: &str = "\x1B[35m";
pub const FG_CYAN: &str = "\x1B[36m";
pub const FG_WHITE: &str = "\x1B[37m";
pub const FG_BRIGHT_BLACK: &str = "\x1B[90m";
pub const FG_BRIGHT_RED: &str = "\x1B[91m";
pub const FG_BRIGHT_GREEN: &str = "\x1B[92m";
pub const FG_BRIGHT_YELLOW: &str = "\x1B[93m";
pub const FG_BRIGHT_BLUE: &str = "\x1B[94m";
pub const FG_BRIGHT_MAGENTA: &str = "\x1B[95m";
pub const FG_BRIGHT_CYAN: &str = "\x1B[96m";
pub const FG_BRIGHT_WHITE: &str = "\x1B[97m";
pub const FG_RESET: &str = "\x1B[39m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";
pub const BG_BRIGHT_BLACK: &str = "\x1B[100m";
pub const BG_BRIGHT_RED: &str = "\x1B[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1B[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1B[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1B[104m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1B[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1B[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1B[107m";
pub const BG_RESET: &str = "\x1B[49m";
