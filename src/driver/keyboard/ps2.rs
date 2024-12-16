use super::{
    fifo_buffer::FifoBuffer,
    keymaps::{Keymap, KeymapValue, us_qwerty::US_QUERTY_KEYMAP},
};
use crate::{
    driver::{
        console::{
            self, CURSOR_DOWN, CURSOR_LEFT, CURSOR_RIGHT, CURSOR_UP, SCROLL_DOWN, SCROLL_UP,
        },
        keyboard::ScanCodeSet,
    },
    io::inb,
    print,
};

#[allow(clippy::struct_excessive_bools)]
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
    const fn new() -> Self {
        Self {
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

    const fn shift(&self) -> bool {
        (self.left_shift | self.right_shift) ^ self.caps_lock
    }

    const fn alt(&self) -> bool {
        self.alt | self.alt_gr
    }

    const fn control(&self) -> bool {
        self.left_control | self.right_control
    }
}

pub struct Keyboard {
    buffer: FifoBuffer,
    key_modifier: KeyModifier,
    keymap: &'static Keymap,
    scan_code_set: ScanCodeSet,
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            buffer: FifoBuffer::new(),
            key_modifier: KeyModifier::new(),
            keymap: &US_QUERTY_KEYMAP,
            scan_code_set: ScanCodeSet::new(),
        }
    }

    pub fn get_input(&mut self) {
        let status = unsafe { inb(0x64) };
        if status & 0x1 == 0x1 {
            let mut data = unsafe { u16::from(inb(0x60)) };
            if data == 0xE0 {
                data = data << 8 | unsafe { u16::from(inb(0x60)) };
            }
            let _ = self.buffer.push(data);
        }
    }

    pub fn interpret_to_vga_text_mode(&mut self) {
        while let Some(scancode) = self.buffer.pop() {
            self.interpret(scancode);
        }
    }

    fn interpret(&mut self, scancode: u16) {
        if let Some(keymap_value) = self.scancode_to_keymapvalue(scancode) {
            self.interpret_keymapvalue(keymap_value, is_pressed(scancode));
        }
    }

    // TODO: refactor thi function
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)]
    fn interpret_keymapvalue(&mut self, keymap_value: KeymapValue, pressed: bool) {
        match keymap_value {
            KeymapValue::Ascii(c) | KeymapValue::Lowercase(c) | KeymapValue::Alt(c) => {
                if pressed {
                    print!("{c}");
                }
            }
            KeymapValue::Control(c) | KeymapValue::ControlAlt(c) => {
                if pressed {
                    let c = ((c as u8) & 0x3F) as char;
                    print!("{c}");
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
                    print!("{}", 0x7Fu8 as char);
                }
            }
            KeymapValue::Right | KeymapValue::AltRight | KeymapValue::ControlRight => {
                if pressed {
                    print!("{CURSOR_RIGHT}");
                }
            }
            KeymapValue::Left | KeymapValue::AltLeft | KeymapValue::ControlLeft => {
                if pressed {
                    print!("{CURSOR_LEFT}");
                }
            }
            KeymapValue::Down | KeymapValue::AltDown | KeymapValue::ControlDown => {
                if pressed {
                    print!("{CURSOR_DOWN}");
                }
            }
            KeymapValue::Up | KeymapValue::AltUp | KeymapValue::ControlUp => {
                if pressed {
                    print!("{CURSOR_UP}");
                }
            }
            KeymapValue::PageUp | KeymapValue::AltPageUp | KeymapValue::ControlPageUp => {
                if pressed {
                    print!("{SCROLL_UP}");
                }
            }
            KeymapValue::PageDown | KeymapValue::AltPageDown | KeymapValue::ControlPageDown => {
                if pressed {
                    print!("{SCROLL_DOWN}");
                }
            }
            KeymapValue::F1 => {
                if pressed {
                    console::change_tty_id(0);
                }
            }
            KeymapValue::F2 => {
                if pressed {
                    console::change_tty_id(1);
                }
            }
            KeymapValue::F3 => {
                if pressed {
                    console::change_tty_id(2);
                }
            }
            KeymapValue::F4 => {
                if pressed {
                    console::change_tty_id(3);
                }
            }
            KeymapValue::F5 => {
                if pressed {
                    console::change_tty_id(4);
                }
            }
            KeymapValue::F6 => {
                if pressed {
                    console::change_tty_id(5);
                }
            }
            KeymapValue::F7 => {
                if pressed {
                    console::change_tty_id(6);
                }
            }
            KeymapValue::F8 => {
                if pressed {
                    console::change_tty_id(7);
                }
            }
            KeymapValue::F9 => {
                if pressed {
                    console::change_tty_id(8);
                }
            }
            KeymapValue::F10 => {
                if pressed {
                    console::change_tty_id(9);
                }
            }
            KeymapValue::F11 => {
                if pressed {
                    console::change_tty_id(10);
                }
            }
            KeymapValue::F12 => {
                if pressed {
                    console::change_tty_id(11);
                }
            }
            _ => {}
        }
    }

    const fn scancode_to_keymapvalue(&self, scancode: u16) -> Option<KeymapValue> {
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
                }
                return Some(keymap_set.not_shift);
            }
        }
        None
    }
}

const fn is_pressed(scancode: u16) -> bool {
    scancode & 0x80 == 0x0
}
