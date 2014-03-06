/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::mem::transmute;
use kernel::*;
use kernel::screen::*;
use kernel::memory::Allocator;
use kernel::serial::*;

use super::super::platform::drivers::arm926ej_s;
use super::super::platform::drivers::arm926ej_s::serial;

pub static uart : &'static mut Serial = &'static mut arm926ej_s::serial::UART0 as &'static mut Serial;
pub static scr : &'static mut TerminalCanvas = &'static mut arm926ej_s::screen::Screen0 as &'static mut TerminalCanvas;

pub static mut buffer: cstr = cstr 
{
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };

fn putstr(msg: &str) 
{
    for c in slice::iter(as_bytes(msg)) {
    	uart.write(*c as char);
    }
}

pub unsafe fn drawstr(msg: &str) 
{
    let old_fg = scr.getCursor().fg_color;
    let mut x : u32 = 0x6699AAFF;
    for c in slice::iter(as_bytes(msg)) {
        x = (x << 8) + (x >> 24); 
        let mut cur = scr.getCursor();
        cur.fg_color = screen::ARGBPixel(
                    (x >> 24) as u8,
                    (x >> 16) as u8,
                    (x >> 8)  as u8,
                    x       as u8
            );
        scr.setCursor(&cur);
        drawchar(*c as char);
    }
    let mut cur = scr.getCursor();
    cur.fg_color = old_fg;
    scr.setCursor(&cur);
}

pub unsafe fn putcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {
        uart.write(*(p as *char));
        p += 1;
    }
}

pub unsafe fn parsekey(x: char) 
{
	let x = x as u8;
	// Set this to false to learn the keycodes of various keys!
	// Key codes are printed backwards because life is hard
		
	if (true) {
		match x { 
			13		=>	{ 
						parse();
						prompt(false); 
			}
			127		=>	{ 
				if (buffer.delete_char()) { 
					uart.write('');
					uart.write(' ');
					uart.write(''); 
					backspace();
				}
			}
			_		=>	{ 
				if (buffer.add_char(x)) { 
					uart.write(x as char);
					drawchar(x as char);
				}
			}
		}
	}
	else {
		keycode(x);
	}
}

unsafe fn drawchar(x: char)
{
    let res = scr.getResolution();
    let mut cur = scr.getCursor();
	if x == '\n' {
		cur.y += cur.height;
		cur.x = 0;
        scr.setCursor(&cur);
		return;
	}

    scr.restore();
    scr.drawCharacter(x);
    cur.x += cur.width;
    if cur.x >= res.w as u32 
    {
        cur.x -= res.w as u32;
        cur.y += cur.height;
    }
    scr.setCursor(&cur);
    scr.backup();
    scr.drawCursor();
}

unsafe fn backspace()
{
    scr.restore();
    let mut cur = scr.getCursor();
    cur.x -= cur.width;
    scr.setCursor(&cur);
    scr.drawCharacter(' ');
    scr.backup();
    scr.drawCursor();
}

fn keycode(x: u8) 
{
	let mut x = x;
	while  x != 0 {
		uart.write((x%10+ ('0' as u8) ) as char);
		x = x/10;
	}
	uart.write(' ');
}

fn screen() {	
	putstr(&"\n                                                               "); 
	putstr(&"\n                                                               ");
	putstr(&"\n                       7=..~$=..:7                             "); 
	putstr(&"\n                  +$: =$$$+$$$?$$$+ ,7?                        "); 
	putstr(&"\n                  $$$$$$$$$$$$$$$$$$Z$$                        ");
	putstr(&"\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    ");
	putstr(&"\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 ");
	putstr(&"\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                ");
	putstr(&"\n           Z$$$$$?                    .+ZZZZ$$                 ");
	putstr(&"\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            "); 
	putstr(&"\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              "); 
	putstr(&"\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           "); 
	putstr(&"\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           "); 
	putstr(&"\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            ");	 
	putstr(&"\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          "); 
	putstr(&"\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          "); 
	putstr(&"\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           "); 
	putstr(&"\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          "); 
	putstr(&"\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         "); 
	putstr(&"\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           "); 
	putstr(&"\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          "); 
	putstr(&"\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          "); 
	putstr(&"\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            "); 
	putstr(&"\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           "); 
	putstr(&"\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           "); 
	putstr(&"\n        IZZZZZ:.                        . ,ZZZZZ$              "); 
	putstr(&"\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             "); 
	putstr(&"\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 "); 
	putstr(&"\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                ");
	putstr(&"\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                ");
	putstr(&"\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    ");
	putstr(&"\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    "); 			
	putstr(&"\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        ");
	putstr(&"\n                       $7..I$7..I$,                            ");
	putstr(&"\n"); 
	putstr(&"\n _                     _     _                         _  ");
	putstr(&"\n| |                   (_)   | |                       | | ");
	putstr(&"\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | ");
	putstr(&"\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | ");
	putstr(&"\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | ");
	putstr(&"\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\n\n");

}

pub unsafe fn init() {
    uart = &'static mut arm926ej_s::serial::UART0 as &'static mut Serial;
    scr =  &'static mut arm926ej_s::screen::Screen0 as &'static mut TerminalCanvas;


    buffer = cstr::new(256);
    screen();
    prompt(true);
}

unsafe fn prompt(startup: bool) {
	putstr(&"\nsgash > ");
	if !startup {drawstr(&"\nsgash > ");}

	buffer.reset();
}

unsafe fn parse() {
	if (buffer.streq(&"ls")) { 
	    putstr( &"\na\tb") ;
	    drawstr( &"\na    b") ;
	};
	match buffer.getarg(' ', 0) {
	    Some(y)        => {
		if(y.streq(&"cat")) {
		    match buffer.getarg(' ', 1) {
			Some(x)        => {
			    if(x.streq(&"a")) { 
				putstr( &"\nHello"); 
				drawstr( &"\nHello"); 
			    }
			    if(x.streq(&"b")) {
				putstr( &"\nworld!");
				drawstr( &"\nworld!");
			    }
			}
			None        => { }
		    };
		}
		if(y.streq(&"open")) {
		    putstr(&"\nTEST YO");
		    drawstr(&"\nTEST YO");
		}
	    }
	    None        => { }
	};
	buffer.reset();
}

/* BUFFER MODIFICATION FUNCTIONS */

struct cstr {
	p: *mut u8,
	p_cstr_i: uint,
	max: uint 
}

impl cstr {
	pub unsafe fn new(size: uint) -> cstr {
		// Sometimes this doesn't allocate enough memory and gets stuck...
		let (x, y) = heap.alloc(size);
		let this = cstr {
			p: x,
			p_cstr_i: 0,
			max: y
		};
		*(((this.p as uint)+this.p_cstr_i) as *mut char) = '\0';
		this
	}

	unsafe fn from_str(s: &str) -> cstr {
		let mut this = cstr::new(256);
		for c in slice::iter(as_bytes(s)) {
			this.add_char(*c);
		};
		this
	}

	fn len(&self) -> uint { self.p_cstr_i }

	// HELP THIS DOESN'T WORK THERE IS NO GARBAGE COLLECTION!!!
	// -- TODO: exchange_malloc, exchange_free
	unsafe fn destroy(&self) { heap.free(self.p); }

	unsafe fn add_char(&mut self, x: u8) -> bool{
		if (self.p_cstr_i == self.max) { return false; }
		*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
		self.p_cstr_i += 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	unsafe fn delete_char(&mut self) -> bool {
		if (self.p_cstr_i == 0) { return false; }
		self.p_cstr_i -= 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	unsafe fn reset(&mut self) {
		self.p_cstr_i = 0; 
		*(self.p as *mut char) = '\0';
	}

	unsafe fn eq(&self, other: &cstr) -> bool {
		if (self.len() != other.len()) { return false; }
		else {
			let mut x = 0;
			let mut selfp: uint = self.p as uint;
			let mut otherp: uint = other.p as uint;
			while x < self.len() {
				if (*(selfp as *char) != *(otherp as *char)) { return false; }
				selfp += 1;
				otherp += 1;
				x += 1;
			}
			true
		}
	}

	unsafe fn streq(&self, other: &str) -> bool {
		let mut x = 0;
		let mut selfp: uint = self.p as uint;
		for c in slice::iter(as_bytes(other)) {
			if( *c != *(selfp as *u8) ) { return false; }
			selfp += 1;
		};
		*(selfp as *char) == '\0'
	}

	unsafe fn getarg(&self, delim: char, mut k: uint) -> Option<cstr> {
		let mut ind: uint = 0;
		let mut found = k == 0;
		let mut selfp: uint = self.p as uint;
		let mut s = cstr::new(256);
		loop {
			if (*(selfp as *char) == '\0') { 
				// End of string
				if (found) { return Some(s); }
				else { return None; }
			};
			if (*(selfp as *u8) == delim as u8) { 
				if (found) { return Some(s); }
				k -= 1;
			};
			if (found) {
				s.add_char(*(selfp as *u8));
			};
			found = k == 0;
			selfp += 1;
			ind += 1;
			if (ind == self.max) { 
				putstr(&"\nSomething broke!");
				return None; 
			}
		}
	}

	unsafe fn split(&self, delim: char) -> (cstr, cstr) {
		let mut selfp: uint = self.p as uint;
		let mut beg = cstr::new(256);
		let mut end = cstr::new(256);
		let mut found = false;
		loop {
			if (*(selfp as *char) == '\0') { 
				return (beg, end);
			}
			else if (*(selfp as *u8) == delim as u8) {
				found = true;
			}
			else if (!found) {
				beg.add_char(*(selfp as *u8));
			}
			else if (found) {
				end.add_char(*(selfp as *u8));
			};
			selfp += 1;
		}
	}


}

