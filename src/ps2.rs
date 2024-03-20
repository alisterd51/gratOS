use crate::{io::inb, vga_buffer};

const BUFFER_SIZE: usize = 500;

// this is a circular fifo type buffer
struct Buffer {
    begin: usize,
    end: usize,
    chars: [u8; BUFFER_SIZE],
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            begin: 0,
            end: 0,
            chars: [0u8; BUFFER_SIZE],
        }
    }

    fn push(&mut self, val: u8) -> Result<(), ()> {
        if (self.end + 1) % BUFFER_SIZE == self.begin {
            return Err(());
        }
        self.chars[self.end] = val;
        self.end = (self.end + 1) % BUFFER_SIZE;
        Ok(())
    }

    fn pop(&mut self) -> Option<u8> {
        if self.end == self.begin {
            return None;
        }
        let val = self.chars[self.begin];
        self.begin = (self.begin + 1) % BUFFER_SIZE;
        Some(val)
    }
}

struct KeyCodes {
    keyccodes: &'static [ScanCodes],
}

impl KeyCodes {
    fn new() -> KeyCodes {
        KeyCodes {
            keyccodes: US_QUERTY,
        }
    }

    fn translate(&self, byte: u8) -> Option<u8> {
        if (byte as usize) < self.keyccodes.len() {
            SCANCODES_TO_ASCII[self.keyccodes[byte as usize] as usize]
        } else {
            None
        }
    }

    fn is_pressed(&self, byte: u8) -> bool {
        byte & 0x80 == 0x0
    }
}

pub struct Keyboard {
    buffer: Buffer,
    keycodes: KeyCodes,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            buffer: Buffer::new(),
            keycodes: KeyCodes::new(),
        }
    }

    pub fn get_input(&mut self) {
        unsafe {
            let status = inb(0x64);
            if status & 1 == 1 {
                let data = inb(0x60);
                let _ = self.buffer.push(data);
            }
        }
    }

    pub fn to_vga_text_mode(&mut self) {
        while let Some(byte) = self.buffer.pop() {
            if self.keycodes.is_pressed(byte) {
                if let Some(byte) = self.keycodes.translate(byte) {
                    vga_buffer::WRITER.lock().write_byte(byte);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanCodes {
    NoCode,
    Escape,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Dash,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftSquareBracket,
    RightSquareBracket,
    Enter,
    LeftControl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    SingleQuote,
    BackTick,
    LeftShift,
    BackSlash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RightShift,
    KeypadStar,
    LeftAlt,
    Space,
    CapsLock,
    FOne,
    FTwo,
    FThree,
    FFour,
    FFive,
    FSix,
    FSeven,
    FEight,
    FNine,
    FTen,
    NumberLock,
    ScrollLock,
    KeypadSeven,
    KeypadEight,
    KeypadNine,
    KeypadDash,
    KeypadFour,
    KeypadFive,
    KeypadSix,
    KeypadPlus,
    KeypadOne,
    KeypadTwo,
    KeypadThree,
    KeypadZero,
    KeypadDot,
    FEleven,
    FTwelve,
}

static SCANCODES_TO_ASCII: &'static [Option<u8>] = &[
    None,
    None,
    Some(b'1'),
    Some(b'2'),
    Some(b'3'),
    Some(b'4'),
    Some(b'5'),
    Some(b'6'),
    Some(b'7'),
    Some(b'8'),
    Some(b'9'),
    Some(b'0'),
    Some(b'-'),
    Some(b'='),
    Some(0x08),
    Some(b'\t'),
    Some(b'q'),
    Some(b'w'),
    Some(b'e'),
    Some(b'r'),
    Some(b't'),
    Some(b'y'),
    Some(b'u'),
    Some(b'i'),
    Some(b'o'),
    Some(b'p'),
    Some(b'['),
    Some(b']'),
    Some(b'\n'),
    None,
    Some(b'a'),
    Some(b's'),
    Some(b'd'),
    Some(b'f'),
    Some(b'g'),
    Some(b'h'),
    Some(b'j'),
    Some(b'k'),
    Some(b'l'),
    Some(b';'),
    Some(b'\''),
    Some(b'`'),
    None,
    Some(b'\\'),
    Some(b'z'),
    Some(b'x'),
    Some(b'c'),
    Some(b'v'),
    Some(b'b'),
    Some(b'n'),
    Some(b'm'),
    Some(b','),
    Some(b'.'),
    Some(b'/'),
    None,
    Some(b'*'),
    None,
    Some(b' '),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(b'7'),
    Some(b'8'),
    Some(b'9'),
    Some(b'-'),
    Some(b'4'),
    Some(b'5'),
    Some(b'6'),
    Some(b'+'),
    Some(b'1'),
    Some(b'2'),
    Some(b'3'),
    Some(b'0'),
    Some(b'.'),
    None,
    None,
];

// TODO: It is important to remember that the scancode can have modifiers: 'alt', 'shift', 'ctrl', ...
// this is not done here :(

static US_QUERTY: &'static [ScanCodes] = &[
    ScanCodes::NoCode,
    ScanCodes::Escape,
    ScanCodes::One,
    ScanCodes::Two,
    ScanCodes::Three,
    ScanCodes::Four,
    ScanCodes::Five,
    ScanCodes::Six,
    ScanCodes::Seven,
    ScanCodes::Eight,
    ScanCodes::Nine,
    ScanCodes::Zero,
    ScanCodes::Dash,
    ScanCodes::Equal,
    ScanCodes::Backspace,
    ScanCodes::Tab,
    ScanCodes::Q,
    ScanCodes::W,
    ScanCodes::E,
    ScanCodes::R,
    ScanCodes::T,
    ScanCodes::Y,
    ScanCodes::U,
    ScanCodes::I,
    ScanCodes::O,
    ScanCodes::P,
    ScanCodes::LeftSquareBracket,
    ScanCodes::RightSquareBracket,
    ScanCodes::Enter,
    ScanCodes::LeftControl,
    ScanCodes::A,
    ScanCodes::S,
    ScanCodes::D,
    ScanCodes::F,
    ScanCodes::G,
    ScanCodes::H,
    ScanCodes::J,
    ScanCodes::K,
    ScanCodes::L,
    ScanCodes::Semicolon,
    ScanCodes::SingleQuote,
    ScanCodes::BackTick,
    ScanCodes::LeftShift,
    ScanCodes::BackSlash,
    ScanCodes::Z,
    ScanCodes::X,
    ScanCodes::C,
    ScanCodes::V,
    ScanCodes::B,
    ScanCodes::N,
    ScanCodes::M,
    ScanCodes::Comma,
    ScanCodes::Dot,
    ScanCodes::Slash,
    ScanCodes::RightShift,
    ScanCodes::KeypadStar,
    ScanCodes::LeftAlt,
    ScanCodes::Space,
    ScanCodes::CapsLock,
    ScanCodes::FOne,
    ScanCodes::FTwo,
    ScanCodes::FThree,
    ScanCodes::FFour,
    ScanCodes::FFive,
    ScanCodes::FSix,
    ScanCodes::FSeven,
    ScanCodes::FEight,
    ScanCodes::FNine,
    ScanCodes::FTen,
    ScanCodes::NumberLock,
    ScanCodes::ScrollLock,
    ScanCodes::KeypadSeven,
    ScanCodes::KeypadEight,
    ScanCodes::KeypadNine,
    ScanCodes::KeypadDash,
    ScanCodes::KeypadFour,
    ScanCodes::KeypadFive,
    ScanCodes::KeypadSix,
    ScanCodes::KeypadPlus,
    ScanCodes::KeypadOne,
    ScanCodes::KeypadTwo,
    ScanCodes::KeypadThree,
    ScanCodes::KeypadZero,
    ScanCodes::KeypadDot,
    ScanCodes::FEleven,
    ScanCodes::FTwelve,
];
