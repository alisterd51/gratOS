use crate::{
    io::inb,
    print,
    us_qwerty::{Key, KeySet, Set, US_QUERTY},
};

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

struct Modifier {
    left_shift: bool,
    right_shift: bool,
    alt: bool,
    alt_gr: bool,
    left_control: bool,
    right_control: bool,
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
}

impl Modifier {
    fn new() -> Modifier {
        Modifier {
            left_shift: false,
            right_shift: false,
            alt: false,
            alt_gr: false,
            left_control: false,
            right_control: false,
            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
        }
    }

    fn shift(&self) -> bool {
        (self.left_shift | self.right_control) ^ self.caps_lock
    }

    fn alt(&self) -> bool {
        self.alt | self.alt_gr
    }

    fn control(&self) -> bool {
        self.left_control | self.right_control
    }
}

struct Layout {
    layout: &'static [Option<KeySet>],
    modifier: Modifier,
}

impl Layout {
    fn new() -> Layout {
        Layout {
            layout: US_QUERTY,
            modifier: Modifier::new(),
        }
    }

    fn translate(&self, byte: u8) -> Option<char> {
        if (byte as usize) < self.layout.len() {
            if let Some(set) = self.get_set(byte) {
                return match set.set_modifier {
                    Some(set_modifier) => match set_modifier {
                        crate::us_qwerty::SetModifier::Lowercase => {
                            if let Key::Char(c) = set.key {
                                return Some(c);
                            } else {
                                return None;
                            }
                        }
                        crate::us_qwerty::SetModifier::Shift => None,
                        crate::us_qwerty::SetModifier::Alt => None,
                        crate::us_qwerty::SetModifier::AltShift => None,
                        crate::us_qwerty::SetModifier::Control => {
                            if let Key::Char(c) = set.key {
                                let c = ((c as u8) & 0x0F) as char;
                                return Some(c);
                            } else {
                                return None;
                            }
                        }
                        crate::us_qwerty::SetModifier::ControlAlt => None,
                        crate::us_qwerty::SetModifier::ControlAltShift => None,
                        crate::us_qwerty::SetModifier::Num => None,
                    },
                    _ => {
                        if let Key::Char(c) = set.key {
                            return Some(c);
                        } else {
                            return None;
                        }
                    }
                };
            }
        }
        None
    }

    fn apply_modifier(&mut self, byte: u8) {
        let pressed = self.is_pressed(byte);
        let byte = byte & 0x7F;

        if (byte as usize) < self.layout.len() {
            if let Some(set) = self.get_set(byte) {
                match set.key {
                    Key::LCTRL => self.modifier.left_control = pressed,
                    Key::RCTRL => self.modifier.right_control = pressed,
                    Key::LSHIFT => self.modifier.left_shift = pressed,
                    Key::RSHIFT => self.modifier.right_shift = pressed,
                    Key::LALT => self.modifier.alt = pressed,
                    Key::RALT => self.modifier.alt_gr = pressed,
                    Key::CALOCK => {
                        if pressed {
                            self.modifier.caps_lock = !self.modifier.caps_lock;
                        }
                    }
                    Key::NLOCK => {
                        if pressed {
                            self.modifier.num_lock = !self.modifier.num_lock;
                        }
                    }
                    Key::SLOCK => {
                        if pressed {
                            self.modifier.scroll_lock = !self.modifier.scroll_lock;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn get_set(&self, byte: u8) -> Option<Set> {
        if (byte as usize) < self.layout.len() {
            if let Some(key_set) = self.layout[byte as usize] {
                if self.modifier.control() && self.modifier.alt() && self.modifier.shift() {
                    todo!()
                } else if self.modifier.control() && self.modifier.alt() {
                    todo!()
                } else if self.modifier.control() {
                    return Some(key_set.ctrl);
                } else if self.modifier.alt() && self.modifier.shift() {
                    return Some(key_set.alt_shift);
                } else if self.modifier.alt {
                    return Some(key_set.alt1);
                } else if self.modifier.alt_gr {
                    return Some(key_set.alt2);
                } else if self.modifier.shift() {
                    return Some(key_set.shift);
                } else {
                    return Some(key_set.not_shift);
                }
            }
        }
        None
    }

    fn is_pressed(&self, byte: u8) -> bool {
        byte & 0x80 == 0x0
    }
}

pub struct Keyboard {
    buffer: Buffer,
    layout: Layout,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            buffer: Buffer::new(),
            layout: Layout::new(),
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
            // TODO: implement inputs on 2 bytes
            self.layout.apply_modifier(byte);
            if self.layout.is_pressed(byte) {
                if let Some(byte) = self.layout.translate(byte) {
                    print!("{}", byte);
                }
            }
        }
    }
}

// https://github.com/Stichting-MINIX-Research-Foundation/minix/blob/4db99f4012570a577414fe2a43697b2f239b699e/minix/drivers/tty/tty/keymaps/us-std.src
// https://wiki.osdev.org/PS/2_Keyboard
