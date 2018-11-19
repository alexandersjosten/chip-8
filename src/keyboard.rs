pub struct Keypad {
    /* Keys are represented as an array, 0 - F, where true on position x means
     * button X was pressed.
     */
    pub keys: [bool; 16];
}

// Implementation of the keypad
impl Keypad {
    // Static method
    fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }
}

/* key_down takes a mutable self parameter (we're going to modify the keys
 * array), and the index of the key that is pressed down
 */
fn key_down(&mut self, index: u8) {
    self.keys[index] = true;
}

/* key_up takes a mutable self parameter (we're going to modify the keys
 * array), and the index of the key that is released
 */
fn key_up(&mut self, index: u8) {
    self.keys[index] = false;
}

/* is_key_pressed takes a non-mutable self parameter (we're not going to modify
 * anything), and the index of the key that is to be checked, and returns true
 * if the key is pressed, false otherwise
 */
fn is_key_pressed(&self, index: u8) -> bool {
    self.keys[index]
}
