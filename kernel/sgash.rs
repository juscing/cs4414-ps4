/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;

pub static mut buffer: cstr = cstr {
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };

pub static mut filesys: Option<fs> = None;

pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }
}

pub unsafe fn drawstr(msg: &str) {
    let old_fg = super::super::io::FG_COLOR;
    let mut x: u32 = 0x6699AAFF;
    for c in slice::iter(as_bytes(msg)) {
	x = (x << 8) + (x >> 24); 
	super::super::io::set_fg(x);
	drawchar(*c as char);
    }
    super::super::io::set_fg(old_fg);
}

pub unsafe fn drawcstr(s: cstr, newln: bool, space: bool) {
    let old_fg = super::super::io::FG_COLOR;
    let mut x: u32 = 0x6699AAFF;
    let mut p = s.p as uint;
    if newln {
	drawchar('\n');
    }
    if space {
	drawchar(' ');
    }
    while *(p as *char) != '\0' {
	x = (x << 8) + (x >> 24); 
	super::super::io::set_fg(x);
	drawchar(*(p as *char));
	p += 1;
    }
    super::super::io::set_fg(old_fg);
}

pub unsafe fn putcstr(s: cstr)
{
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {
	putchar(*(p as *char));
	p += 1;
    }
}

pub unsafe fn parsekey(x: char) {
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
					putchar('');
					putchar(' ');
					putchar(''); 
					backspace();
				}
			}
			_		=>	{ 
				if (buffer.add_char(x)) { 
					putchar(x as char);
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
	if x == '\n' {
		io::CURSOR_Y += io::CURSOR_HEIGHT;
		io::CURSOR_X = 0u32;
		return;
	}

    io::restore();
    io::draw_char(x);
    io::CURSOR_X += io::CURSOR_WIDTH;
    if io::CURSOR_X >= io::SCREEN_WIDTH {io::CURSOR_X -= io::SCREEN_WIDTH; io::CURSOR_Y += io::CURSOR_HEIGHT}
    io::backup();
    io::draw_cursor();
}

unsafe fn backspace()
{
    io::restore();
    io::CURSOR_X -= io::CURSOR_WIDTH;
    io::draw_char(' ');
    io::backup();
    io::draw_cursor();
}

fn keycode(x: u8) {
	let mut x = x;
	while  x != 0 {
		putchar((x%10+ ('0' as u8) ) as char);
		x = x/10;
	}
	putchar(' ');
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
    buffer = cstr::new(256);
    filesys = Some(fs::new());
    screen();
    prompt(true);
}

unsafe fn prompt(startup: bool) {
	//PROBLEM 1
	putstr(&"\nsgash> ");
	if !startup {drawstr(&"\nsgash> ");}
	buffer.reset();
}

unsafe fn parse() {
	match buffer.getarg(' ', 0) {
	    Some(y)        => {
		if y.len() == 0 {
		    return;
		}
		// COMMANDS echo, ls, cat, cd, rm, mkdir, pwd, wr
		if(y.streq(&"echo")) {
		    let mut i = 1;
		    putstr(&"\n");
		    loop {
			match buffer.getarg(' ', i) {
			    Some(word) => {
				if i != 1 {
				    putstr(&" ");
				}
				putcstr(word);
				if i == 1 {
				    drawcstr(word, true, false);
				} else {
				    drawcstr(word, false, true);
				}
				i+=1;
			    }
			    None => { break; }
			}
		    }
		} else if(y.streq(&"ls")) {
		    let mut i = 0;
		    while i < filesys.get().cwd.len() {
			putstr(&"IS THIS WORKING");
			let ptr = filesys.get().cwd.get_dir(i) as *dnode;
			let t = *ptr;
			putcstr(t.name);
			drawcstr(t.name, true, false);
			i = i + 1;
		    }
		    /*
		    putstr(&"\nTEST ls");
		    drawstr(&"\nTEST ls");
		    */
		} else if(y.streq(&"cat")) {
		    putstr(&"\nTEST cat");
		    drawstr(&"\nTEST cat");
		} else if(y.streq(&"cd")) {
		    putstr(&"\nTEST cd");
		    drawstr(&"\nTEST cd");
		} else if(y.streq(&"rm")) {
		    putstr(&"\nTEST rm");
		    drawstr(&"\nTEST rm");
		} else if(y.streq(&"mkdir")) {
		    match buffer.getarg(' ', 1) {
			Some(mut word) => {
			    if word.len() < 1 {
				putstr(&"Bad Directory Name\n");
				drawstr(&"Bad Directory Name\n");
				return;
			    }
			    let cwdptr = &filesys.get().cwd as *dnode as uint;
			    let dir = Some(dnode::new(256, word, cwdptr));
			    let x = filesys.get().cwd.add_child((&dir.get() as *dnode) as uint);
			    /*
			    if x {
				putstr(&"SUCCESS");
			    } else {
				putstr(&"Fail");
			    }
			    */
			}
			None => {
			    putstr(&"Bad Directory Name\n");
			    drawstr(&"Bad Directory Name\n");
			}
		    }
		    /*
		    putstr(&"\nTEST mkdir");
		    drawstr(&"\nTEST mkdir");
		    */
		} else if(y.streq(&"pwd")) {
		    putcstr(filesys.get().cwd.name);
		    drawcstr(filesys.get().cwd.name, true, false);
		} else if(y.streq(&"wr")) {
		    putstr(&"\nTEST wr");
		    drawstr(&"\nTEST wr");
		} else {
		    putstr(&"\nUnrecognized Command!");
		    drawstr(&"\nUnrecognized Command!");
		}
		/*
		if(y.streq(&"cat")) {
		    
		    match buffer.getarg(' ', 1) {
			Some(x)        => {
			    if(x.streq(&"a")) { 
				putstr( &"\nHowdy!"); 
				drawstr( &"\nHowdy!"); 
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
		*/
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

#[allow(dead_code)]
	unsafe fn from_str(s: &str) -> cstr {
		let mut this = cstr::new(256);
		for c in slice::iter(as_bytes(s)) {
			this.add_char(*c);
		};
		this
	}

#[allow(dead_code)]
	fn len(&self) -> uint { self.p_cstr_i }

	// HELP THIS DOESN'T WORK THERE IS NO GARBAGE COLLECTION!!!
	// -- TODO: exchange_malloc, exchange_free
#[allow(dead_code)]
	unsafe fn destroy(&self) { heap.free(self.p); }

	unsafe fn add_char(&mut self, x: u8) -> bool{
		if (self.p_cstr_i == self.max) { return false; }
		*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
		self.p_cstr_i += 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}
	
	unsafe fn get_char(&mut self, x: uint) -> char{
	    if x >= self.p_cstr_i { return '\0'; }
	    //raw memory address! just index it!
	    *(((self.p as uint)+x) as *mut char)
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

struct fs {
    cwd: dnode,
}

impl fs {
    unsafe fn new() -> fs {
	let rdnode = dnode::new(256, cstr::from_str(&"/"), '\0' as uint);
	let this = fs {
	    cwd: rdnode,
	};
	this
    }
}

struct dnode {
    children: *mut uint,
    curptr: uint,
    name: cstr,
    max: uint,
    parent: uint,
}

impl dnode {
    unsafe fn new(size: uint, name: cstr, parent: uint) -> dnode {
	// Sometimes this doesn't allocate enough memory and gets stuck...
	let (x, y) = heap.alloc(size);
	let this = dnode {
		children: x as *mut uint,
		curptr: 0,
		name: name,
		max: y / 4 as uint,
		parent: parent,
	};
	*(((this.children as uint)+this.curptr) as *mut uint) = '\0' as uint;
	this
    }
    
    fn len(&self) -> uint { self.curptr }
    
    unsafe fn add_child(&mut self, x: uint) -> bool{
	if (self.curptr == self.max) { return false; }
	*(((self.children as uint)+self.curptr) as *mut uint) = x;
	self.curptr += 1;
	*(((self.children as uint)+self.curptr) as *mut uint) = '\0' as uint;
	true
    }
    
    unsafe fn get_dir(&mut self, x: uint) -> u8{
	if x >= self.curptr { return '\0' as u8; }
	//raw memory address! just index it!
	*(((self.children as u8)+x as u8) as *mut u8)
    }
    
    /*
    unsafe fn delete_item(&mut self) -> bool {
	if (self.curptr == 0) { return false; }
	self.curptr -= 1;
	*(((self.children as uint)+self.curptr) as *mut char) = '\0';
	true
    }
    */
}

struct fnode {
    data: cstr,
    curptr: uint,
    name: cstr,
    parent: u8,
}

impl fnode {
    unsafe fn new(size: uint, name: cstr, data: cstr, parent: u8) -> fnode {
	let this = fnode {
	    data: data,
	    curptr: 0,
	    name: name,
	    parent: parent,
	};
	this
    }
}
