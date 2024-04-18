use crate::{
    driver::{keyboard::ScanCodeSet, vga},
    io::inb,
    print,
};

use super::{
    fifo_buffer::FifoBuffer,
    keymaps::{us_qwerty::US_QUERTY_KEYMAP, Keymap, KeymapValue},
};

struct KeyModifier {
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

impl KeyModifier {
    fn new() -> KeyModifier {
        KeyModifier {
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
        (self.left_shift | self.right_shift) ^ self.caps_lock
    }

    fn alt(&self) -> bool {
        self.alt | self.alt_gr
    }

    fn control(&self) -> bool {
        self.left_control | self.right_control
    }
}

pub struct Keyboard {
    buffer: FifoBuffer,
    key_modifier: KeyModifier,
    keymap: Keymap,
    scan_code_set: ScanCodeSet,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            buffer: FifoBuffer::new(),
            key_modifier: KeyModifier::new(),
            keymap: US_QUERTY_KEYMAP,
            scan_code_set: ScanCodeSet::new(),
        }
    }

    pub fn get_input(&mut self) {
        let status = unsafe { inb(0x64) };
        if status & 0x1 == 0x1 {
            let mut data = unsafe { inb(0x60) as u16 };
            if data == 0xE0 {
                data = data << 8 | unsafe { inb(0x60) as u16 };
            }
            let _ = self.buffer.push(data);
        }
    }

    pub fn interpret_to_vga_text_mode(&mut self) {
        while let Some(scancode) = self.buffer.pop() {
            self.interpret(scancode)
        }
    }

    fn interpret(&mut self, scancode: u16) {
        if let Some(keymap_value) = self.scancode_to_keymapvalue(scancode) {
            self.interpret_keymapvalue(keymap_value, self.is_pressed(scancode));
        }
    }

    fn interpret_keymapvalue(&mut self, keymap_value: KeymapValue, pressed: bool) {
        match keymap_value {
            KeymapValue::Ascii(c) => {
                if pressed {
                    print!("{}", c);
                }
            }
            KeymapValue::Lowercase(c) => {
                if pressed {
                    print!("{}", c);
                }
            }
            KeymapValue::Alt(c) => {
                if pressed {
                    print!("{}", c);
                }
            }
            KeymapValue::Control(c) => {
                if pressed {
                    let c = ((c as u8) & 0x3F) as char;
                    print!("{}", c);
                }
            }
            KeymapValue::ControlAlt(c) => {
                if pressed {
                    let c = ((c as u8) & 0x3F) as char;
                    print!("{}", c);
                }
            }
            KeymapValue::CapsLock => {
                if pressed {
                    self.key_modifier.caps_lock = !self.key_modifier.caps_lock;
                }
            }
            KeymapValue::NumberLock => {
                if pressed {
                    self.key_modifier.num_lock = !self.key_modifier.num_lock;
                }
            }
            KeymapValue::ScrollLock => {
                if pressed {
                    self.key_modifier.scroll_lock = !self.key_modifier.scroll_lock;
                }
            }
            KeymapValue::LeftControl => self.key_modifier.left_control = pressed,
            KeymapValue::LeftShift => self.key_modifier.left_shift = pressed,
            KeymapValue::LeftAlt => self.key_modifier.alt = pressed,
            KeymapValue::RightControl => self.key_modifier.right_control = pressed,
            KeymapValue::RightShift => self.key_modifier.right_shift = pressed,
            KeymapValue::RightAlt => self.key_modifier.alt_gr = pressed,
            KeymapValue::Delete => {
                if pressed {
                    print!("{}", 0x7F as char);
                }
            }
            KeymapValue::Right | KeymapValue::AltRight | KeymapValue::ControlRight => {
                if pressed {
                    vga::text_mode::cursor_right();
                }
            }
            KeymapValue::Left | KeymapValue::AltLeft | KeymapValue::ControlLeft => {
                if pressed {
                    vga::text_mode::cursor_left();
                }
            }
            KeymapValue::Down | KeymapValue::AltDown | KeymapValue::ControlDown => {
                if pressed {
                    vga::text_mode::cursor_down();
                }
            }
            KeymapValue::Up | KeymapValue::AltUp | KeymapValue::ControlUp => {
                if pressed {
                    vga::text_mode::cursor_up();
                }
            }
        }
    }

    fn scancode_to_keymapvalue(&mut self, scancode: u16) -> Option<KeymapValue> {
        if let Some(scan_code_value) = self.scan_code_set.from(scancode) {
            if let Some(keymap_set) = self.keymap[scan_code_value as usize] {
                if self.key_modifier.control() {
                    return Some(keymap_set.ctrl);
                } else if self.key_modifier.alt() && self.key_modifier.shift() {
                    return Some(keymap_set.alt_shift);
                } else if self.key_modifier.alt {
                    return Some(keymap_set.alt1);
                } else if self.key_modifier.alt_gr {
                    return Some(keymap_set.alt2);
                } else if self.key_modifier.shift() {
                    return Some(keymap_set.shift);
                } else {
                    return Some(keymap_set.not_shift);
                }
            }
        }
        None
    }

    fn is_pressed(&self, scancode: u16) -> bool {
        scancode & 0x80 == 0x0
    }
}
