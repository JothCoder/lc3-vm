/// IO related utility functions
pub mod io {
    use std::io::{self, Read};
    use termios::{tcsetattr, Termios};
    use termios::{
        BRKINT, ECHO, ICANON, ICRNL, IGNBRK, IGNCR, INLCR, ISTRIP, IXON, PARMRK, TCSANOW,
    };

    pub fn read_next_byte() -> u8 {
        let mut single_byte_buffer = [0];
        io::stdin()
            .read_exact(&mut single_byte_buffer)
            .expect("Error while reading next byte from stdin");
        single_byte_buffer[0]
    }

    pub fn disable_input_buffering() -> termios::Termios {
        let original_termios = Termios::from_fd(0).unwrap();

        let mut new_termios = original_termios.clone();
        new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &mut new_termios).unwrap();

        original_termios
    }

    pub fn restore_input_buffering(original_termios: termios::Termios) {
        tcsetattr(0, TCSANOW, &original_termios).unwrap();
    }
}

/// Bit operation utility functions
pub mod bit_ops {
    /// Sign extends `bit_count` bits of the given `value` to 16 bits
    pub fn sign_extend(value: u16, bit_count: usize) -> u16 {
        match (value >> (bit_count - 1)) & 0x1 {
            0x1 => value | (0xFFFF << bit_count),
            0x0 => value,
            _ => unreachable!(),
        }
    }
}
