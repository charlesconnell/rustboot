/* drivers::arm926ej_s */

use core::option::{Some, None};
use core::mem;
use platform::cpu::interrupt;
use kernel;
use kernel::{serial, screen, sgash};
use kernel::screen::*;
use kernel::sgash::SGASH;
use core::mem::transmute;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
static VIC_INT          : *mut u32 = (0x10140000) as *mut u32;
static VIC_INT_ENABLE   : *mut u32 = (0x10140000 + 0x10) as *mut u32;
static VIC_INT_DISABLE  : *mut u32 = (0x10140000 + 0x14) as *mut u32; // "enable clear"

pub mod screen
{
    use kernel::sgash;
    use kernel::screen::*;
    use kernel::screen::font;
    use super::super::io::*;
    use core::mem::{volatile_store, volatile_load};
    use core::fail::abort;

    pub struct canvas{
        CURSOR : cursor     ,
        CURSOR_BUFFER: [u32, ..8*16],
        SAVE_X: u32         ,
        SAVE_Y: u32         ,
        START_ADDR: u32     ,
        SCREEN_WIDTH: u32   ,
        SCREEN_HEIGHT: u32  ,
    }

    pub static mut Screen0 : canvas = canvas{
            CURSOR : cursor{
                x      : 0,
                y      : 0,
                height : 8,
                width  : 16,
                cursor_color  : ARGBPixel(0, 0, 0, 0xFF),
                fg_color      : ARGBPixel(0, 0, 0, 0),
                bg_color      : ARGBPixel(0, 0xFF, 0xFF, 0xFF),
            },
            CURSOR_BUFFER   : [0x00FF0000, ..8*16],
            SAVE_X          : 0,
            SAVE_Y          : 0,
            START_ADDR      : 1024*1024,
            SCREEN_WIDTH    : 0,
            SCREEN_HEIGHT   : 0, 
    };

    impl ScreenCanvas for canvas
    {
        fn sync(&mut self) -> bool 
        {
            true 
        }
        
        fn setResolution(&mut self, res : Resolution) -> Resolution
        {
            self.SCREEN_WIDTH = res.w as u32;
            self.SCREEN_HEIGHT = res.h as u32;
            /*unsafe {
                sgash::init()
            };*/
            /* For the following magic values, see 
             * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACHEDGD.html
             */
            match res {
                WVGA2 => unsafe {
                    // 800x600
                    ws(0x10000010, 0x2CAC);
                    ws(0x10120000, 0x1313A4C4);
                    ws(0x10120004, 0x0505F657);
                    ws(0x10120008, 0x071F1800);

                    /* See http://forum.osdev.org/viewtopic.php?p=195000 */
                    ws(0x10120010, self.START_ADDR);
                    
                    /* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
                    ws(0x10120018, 0x82B);
                },
                VGA => unsafe {
                    // 640x480
                    // See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
                    ws(0x10000010, 0x2C77);
                    ws(0x10120000, 0x3F1F3F9C);
                    ws(0x10120004, 0x090B61DF);
                    ws(0x10120008, 0x067F1800);

                    /* See http://forum.osdev.org/viewtopic.php?p=195000 */
                    ws(0x10120010, self.START_ADDR);

                    /* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
                    ws(0x10120018, 0x82B);
                },
                _ => abort() 
                    //*/
            } // match resolution                

            self.getResolution()
        } // setResolution

        fn getResolution(&self) -> Resolution
        {
            Resolution{w: self.SCREEN_WIDTH as uint, h: self.SCREEN_HEIGHT as uint}
        }
        
        fn getColorDepth(&self) -> ColorDepth
        {
            ARGB
        }

        fn setColorDepth(&mut self, d : ColorDepth) -> ColorDepth
        {
            self.getColorDepth()
        }
        
        fn drawPixel(&mut self, color : &Pixel, coords : &(uint, uint)) -> bool
        {
            false
        }
                
        fn ready(&mut self) -> bool
        {
            true
        }

    } // impl ScreenCanvas

    impl TerminalCanvas for canvas
    {
        fn getCursor(&self) -> cursor 
        {
            self.CURSOR
        }
        fn setCursor(&mut self, c : &cursor) -> cursor
        {
            self.CURSOR = *c;
            self.CURSOR
        }
        unsafe fn scrollup(&mut self)
        {
            let curHeight = self.CURSOR.height;
            let curWidth = self.CURSOR.width;
            let mut i = curHeight * self.SCREEN_WIDTH;
            while i < (self.SCREEN_WIDTH*self.SCREEN_HEIGHT)
            {
                *((self.START_ADDR + ((i-16*self.SCREEN_WIDTH)*4)) as *mut u32) = *((self.START_ADDR+(i*4)) as *u32); 
                i += 1;
            }
            i = 4*(self.SCREEN_WIDTH*self.SCREEN_HEIGHT - curHeight*self.SCREEN_WIDTH);
            while i < 4*self.SCREEN_WIDTH*self.SCREEN_HEIGHT
            {
                *((self.START_ADDR + (i)) as *mut u32) = self.CURSOR.bg_color.word();
                i += 4;
            }
            self.CURSOR.x = 0x0;
            self.CURSOR.y -= curHeight;
        }
        unsafe fn drawCharacter(&mut self, c: char) -> bool
        {
            let curHeight = self.CURSOR.height;
            let curWidth = self.CURSOR.width;

            if self.CURSOR.x +(self.SCREEN_WIDTH* (self.CURSOR.y)) >= self.SCREEN_WIDTH*self.SCREEN_HEIGHT
            {
                self.scrollup();
            }
            let font_offset = (c as u8) - 0x20;
            let map = font::bitmaps[font_offset];

            let mut i = -1;
            let mut j = 0;
            let mut addr = self.START_ADDR + 4*(self.CURSOR.x + curWidth + 1 + self.SCREEN_WIDTH* (self.CURSOR.y));
            while j < self.CURSOR.height
            {
                while i < self.CURSOR.width
                {
                    //let addr = START_ADDR + 4*(CURSOR.x + CURSOR_WIDTH - i + SCREEN_WIDTH*(CURSOR.y + j));
                    //let addr = START_ADDR + 4*(CURSOR.x + CURSOR_WIDTH + SCREEN_WIDTH*CURSOR.y) - 4*i + 4*SCREEN_WIDTH*j
                    *(addr as *mut u32) = {
                        if ((map[j] >> 4*i) & 1) == 1
                        {
                            self.CURSOR.fg_color.word()
                        }
                        else
                        {
                            self.CURSOR.bg_color.word()
                        }
                    }; 
                    
                    addr-= 4;
                    i += 1;
                }
                addr += 4u32*(i + self.SCREEN_WIDTH);
                i = 0;
                j += 1;
            }
            true
        }
        
        unsafe fn backup(&mut self)
        {
            let mut i = 0;
            let mut j = 0;
            while j < self.CURSOR.height
            {
                while i < self.CURSOR.width
                {
                    let addr = self.START_ADDR + 4*((self.CURSOR.x + i) + self.SCREEN_WIDTH*((self.CURSOR.y + j)));
                    self.CURSOR_BUFFER[i + j*8] = *(addr as *mut u32);
                    i += 1;
                }
            i = 0;
            j += 1;
            }
            self.SAVE_X = self.CURSOR.x;
            self.SAVE_Y = self.CURSOR.y;
        }

        unsafe fn restore(&mut self)
        {
            let mut i = 0;
            let mut j = 0;
            while j < self.CURSOR.height
            {
                while i < self.CURSOR.width
                {
                    let addr = self.START_ADDR + 4*(self.SAVE_X + i + self.SCREEN_WIDTH*(self.SAVE_Y + j));
                    *(addr as *mut u32) = self.CURSOR_BUFFER[i + j*8];
                    i += 1;
                }
                i = 0;
                j += 1;
            }
        }

        unsafe fn drawCursor(&mut self)
        {
            let mut i = 0;
            let mut j = 0;

            while j < self.CURSOR.height
            {
                while i < self.CURSOR.width
                {
                    let addr = self.START_ADDR + 4*(self.CURSOR.x + i + self.SCREEN_WIDTH*(self.CURSOR.y + j));
                    *(addr as *mut u32) = self.CURSOR.cursor_color.word();
                    i += 1;
                }
                i = 0;
                j += 1;
            }

        }

    }

    impl canvas
    {
        pub unsafe fn paint(&mut self, color: u32)
        {
            let mut i = 0; 
            while i < self.SCREEN_WIDTH*self.SCREEN_HEIGHT
            {
                *((self.START_ADDR as u32 + i*4) as *mut u32) = color;
                i+=1;
            }
        }

        pub unsafe fn fill_bg(&mut self)
        {
            let word : u32 = self.CURSOR.bg_color.word();
            self.paint(word);
        }

        pub unsafe fn set_fg(&mut self, color: Pixel)
        {
            self.CURSOR.fg_color = color;
        }

        pub unsafe fn set_bg(&mut self, color: Pixel)
        {
            self.CURSOR.bg_color = color;
        }

        pub unsafe fn set_cursor_color(&mut self, color: Pixel)
        {
            self.CURSOR.cursor_color = color;
        }
    }
}


pub unsafe fn init(r : Resolution)
{
    let cv = screen::Screen0;
    cv.setResolution(r);
    cv.set_bg(kernel::screen::ARGBPixel(0x00, 0x22, 0x2C, 0x38));
    cv.set_fg(kernel::screen::ARGBPixel(0x00, 0xFA, 0xFC, 0xFF));
    cv.set_cursor_color(kernel::screen::ARGBPixel(0x00, 0xFA, 0xFC, 0xFF));
    cv.fill_bg();

    let size : uint = mem::size_of::<SGASH>();
    let shell : &mut kernel::shell::Shell = transmute(kernel::malloc_raw(size));
    shell.attachToScreen(&screen::Screen0);
    shell.attachToSerial(&serial::UART0);
}

pub mod serial
{
    use kernel::serial::*;
    use kernel;
    use platform::cpu::interrupt;

    use core::mem::{volatile_load, volatile_store};
    struct UART{
        base : *mut u32,
        IMSC : *mut u32,
        IRQ : u8,
        rate : baud,
    }

    pub static mut UART0 : UART = UART {
        base : 0x101f1000 as *mut u32,
        IMSC :  (0x101f1000 + 0x038) as *mut u32,
        IRQ : 12,
        /* TODO receive handlers */
        rate : 0,
    }; 

    impl Serial for UART{

        /// Initialize device and begin transmission. Returns true if device successfully opened.
        fn open(&mut self, r : u32) -> bool
        {
            unsafe{
                // enable UART0 IRQ [4]
                *super::VIC_INT_ENABLE = 1 << self.IRQ;
                // enable RXIM interrupt (interrupt on receive)
                /*
                 * See
                 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0183f/I54603.html
                 */
                *self.IMSC = 1 << 4;
                kernel::int_table.map(|t| {
                    t.enable(interrupt::IRQ, UART0_receiveInterrupt);
                });
            }
            false
        }

        fn isOpen(&self) -> bool
        {
            self.rate == 0
        }

        /// End transmission, close device. Returns true if device is closed after operation.
        fn close(&mut self) -> bool
        {
            self.rate = 0;
            true
        }

        /// Number of bytes available to read
        fn available(&self) -> uint
        {
            0
        }
        
        /// Read up to length bytes into buffer. Return number of bytes read.
        fn readBuf(&mut self, buffer : &mut u8, length : uint) -> uint
        {
            0
        }

        /// Read one character into buffer. Return number of bytes read.
        fn read(&mut self, c : &mut char) -> uint
        {
            0
        }

        /// Write a single byte. Return number of bytes written.
        fn write(&self, c : char) -> uint
        {
            unsafe {
                /*
                 * We need to include a blank asm call to prevent rustc
                 * from optimizing this part out
                 */
                asm!("");
                volatile_store(self.base, c as u32);
            }
            1
        }

        /// Write a buffer of bytes. Return number of bytes written.
        fn writeBuf(&self, buffer : &u8, length : uint) -> uint
        {
            0
        }

        fn flush(&self) -> uint
        {
            0
        }

        /// Callback on new data available.
        fn addReceiveHandler(&self, newHandler : serialReceiveHandler) -> bool
        {
            false
        }

        /// Remove all receive handlers
        fn clearReceiveHandlers(&self)
        {
            ()
        }
    }
   
#[no_mangle]
    fn UART0_receiveInterrupt() 
    {
        let x = (*UART0.base) as u8 as char;
        asm!("  pop {r11, lr}
                subs pc, r14, #4") // pc = lr - 4

    }
}
