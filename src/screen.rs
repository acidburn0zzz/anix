#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

//All colors
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    char: u8,
    color: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
pub struct Screen {
    col: usize,
    row: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}
impl Screen {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.newline();
                }

                let row = self.row;
                let col = self.col;
                let color = self.color;

                self.buffer.chars[row][col] = Char {
                    char: byte,
                    color,
                };
                self.col += 1;
            }
        }
    }
    pub fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
    fn newline(&mut self) {
        self.row += 1;
        self.col = 0;
    }
    fn put_pixel(&mut self, color : ColorCode, x: usize, y: usize){
        self.buffer.chars[x][y] = Char{
            char: 0,
            color: color,
        }
    }
}

pub fn createScreen() {
    let mut screen = Screen {
        col: BUFFER_WIDTH / 2,
        row: BUFFER_HEIGHT / 2,
        color: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    screen.print("
----------------------------------
|     -      |-    |  ||   -   - |
|    | |     | -   |         -   |
|   |   |    |  -  |  ||    - -  |
|  |-----|   |   - |  ||   -   - |
| |       |  |    -|  ||  -     -|
----------------------------------
    ");

    //logoScreen(screen);
    screen.newline();
    screen.print("Loading, please wait...");
    screen.put_pixel(ColorCode::new(Color::Blue, Color::Green), 2, 2);

    for time in 0..100{
        screen.print(".");
    }
}

/*fn logoScreen(screen: Screen){
    screen.print("
----------------------------------
|     -      |-    |  ||   -   - |
|    | |     | -   |         -   |
|   |   |    |  -  |  ||    - -  |
|  |-----|   |   - |  ||   -   - |
| |       |  |    -|  ||  -     -|
----------------------------------
    ");
}*/
