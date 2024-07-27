use crate::print;

pub struct Shell {
    initialized: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if !self.initialized {
            print!("> ");
            self.initialized = true;
        }
    }
}
