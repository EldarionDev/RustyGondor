pub mod colors;

const VGA_ADDRESS: i32 = 0xb8000;

pub struct VGA {
    cursor_position: i32,
    foreground_color: colors::VGAColor,
    background_color: colors::VGAColor
}

impl VGA {
    pub fn new() -> Self {
        VGA {
            cursor_position: 0,
            foreground_color: colors::VGAColor::Black,
            background_color: colors::VGAColor::White
        }
    }

    pub fn new_page(&mut self) {
        for i in 0..(80 * 25) {
            let char_p = (VGA_ADDRESS + i * 2) as *mut i8;
            let color_p = (VGA_ADDRESS + (i * 2 + 1)) as *mut i8;
            unsafe {
                *char_p = 0;
                *color_p = 0x0;
            }
        }
    }

    pub fn fill(&mut self, color: colors::VGAColor) {
        for i in 0..(80 * 25) {
            let color_p = (VGA_ADDRESS + (i * 2 + 1)) as *mut i8;
            let color = color.clone() as i8;
            unsafe {
                *color_p =  (color << 4) | (*color_p & 0x0F);
            }
        }
    }

    pub fn write(&mut self, text: &str) {
        for c in text.chars() {
            let char_p = (VGA_ADDRESS + self.cursor_position * 2) as *mut i8;
            let color_p = (VGA_ADDRESS + (self.cursor_position * 2 + 1)) as *mut i8;
            unsafe {
                *char_p = c as i8;
                *color_p = ((self.background_color.clone() as i8) << 4) | (self.foreground_color.clone() as i8 & 0x7F);
            }
        }
    }
}