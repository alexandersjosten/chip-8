pub struct Cpu {
    /* The program counter, 16 bit unsigned integer, used to store the currently
     * executing address.*/
    pub pc: u16;

    // 16 general purpose 8-bit registers (V0 - VF)
    pub vx: [u8; 16];

    /* This register is generally used to store memory addresses, so only the
     * lowest (rightmost) 12 bits are usually used.*/
    pub i: u16;

    // General purpose registers for timers
    pub delay: u8;
    pub sound: u8;

    /* Stack pointer can be 8-bit, it is used to point to the topmost level of
     * the stack.*/
    pub sp: u8;

    // The stack is an array of 16 16-bit values
    pub stack: [u16; 16];

    // 16-key hexadecimal keypad
    pub keyboard: Keypad;

    // 64x32-pixel monochrome display
    pub display: Display;
}
