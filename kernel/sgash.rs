/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use core::slice::{Items, Slice, iter, unchecked_get, unchecked_mut_get};
use super::super::platform::*;
use kernel::memory::Allocator;
use kernel::vec::Vec;
use kernel::fs;

pub static mut buffer: cstr = cstr {
	p: 0 as *mut u8,
	p_cstr_i: 0,
	max: 0
};

static ds: cstr = cstr {
	p: 0 as *mut u8,
	p_cstr_i: 0,
	max: 0
};

pub static mut root: fs::directory = fs::directory {
    name: cstr {
        p: 0 as *mut u8,
        p_cstr_i: 0,
        max: 0 
    },
    parent: '\0' as *mut fs::directory,
    fchildren: '\0' as *mut vec::Vec<fs::file>,
    dchildren: '\0' as *mut vec::Vec<fs::directory>,

};

pub static mut cwd: *mut fs::directory = 0 as *mut fs::directory;

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

pub fn putstr(msg: &str) {
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
	root = fs::directory::new(cstr::from_str("/"), '\0' as *mut fs::directory);
	cwd = &mut root as *mut fs::directory;
	screen();
	prompt(true);
}

unsafe fn prompt(startup: bool) {
	//PROBLEM 1
	putstr(&"\nsgash> ");
	if !startup {drawstr(&"\nsgash> ");}
	buffer.reset();
}

unsafe fn cowsay(text: cstr) {
    putstr(&"\n ___");
    drawstr(&"\n ___");
    putstr(&"\n< ");
    drawstr(&"\n< ");
    putcstr(text);
    drawcstr(text, false, false);
    putstr(&" >");
    drawstr(&" >");
    putstr(&"\n ---");
    drawstr(&"\n ---");
    putstr(&"\n        \\   ^__^");
    drawstr(&"\n        \\   ^__^");
    putstr(&"\n         \\  (oo)\\_______");
    drawstr(&"\n         \\  (oo)\\_______");
    putstr(&"\n            (__)\\       )\\/\\");
    drawstr(&"\n            (__)\\       )\\/\\");
    putstr(&"\n                ||----w |");
    drawstr(&"\n                ||----w |");
    putstr(&"\n                ||     ||");
    drawstr(&"\n                ||     ||");
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
			(*cwd).listDir();
		} else if(y.streq(&"cat")) {
		    match buffer.getarg(' ', 1) {
			Some(word) => {
			    if (*cwd).cont_file(word) {
				(*cwd).cat(word);
			    } else {
				putstr(&"No such file");
				drawstr(&"No such file");
			    }
			}
			None => { }
		    }
		} else if(y.streq(&"cd")) {
			match buffer.getarg(' ', 1) {
				Some(mut word) => {
			         let check = fs::cd(cwd,word);
                     match check {
                        (x, y) => {
                            if x {
                                cwd = y;
                                putstr(&"Current folder");
                                putcstr((*cwd).name);
                                drawstr(&"Current folder");
                                drawcstr((*cwd).name,true,false);
                            }
                            else {
                                putstr(&"Directory does not exist\n");
                                drawstr(&"Directory does not exist\n");
                            } 
                        }
                        /*( _, _) => {
                            putstr(&"Something horrible has occurred\n");
                            drawstr(&"Something horrible has occurred\n");
                        }*/
                     }
				}
				None => {
					putstr(&"Bad Directory Name\n");
					drawstr(&"Bad Directory Name\n");
				}
			}
            


		} else if(y.streq(&"rm")) {
			match buffer.getarg(' ', 1) {
				Some(mut filename) => {
					if filename.len() < 1 {
						putstr(&"Bad File Name\n");
						drawstr(&"Bad File Name\n");
						return;
					}
					// let cwdptr = &cwd as *dnode;
					// let dir = dnode::new(256, word, cwdptr as u32);
					// let x = cwd.add_child((&dir as *dnode) as u32);

					if (*cwd).cont_file(filename)
					{
					    (*cwd).remove_file(filename);
					}
					else if (*cwd).cont_dir(filename) {
					    if (*cwd).remove_dir(filename) {
						putstr(&"\nRemoved ");
						drawstr(&"\nRemoved ");
						putcstr(filename);
						drawcstr(filename, false, false);
					    } else {
						putstr(&"\nThe folder is not empty.");
						drawstr(&"\nThe folder is not empty.");
					    }
					} else {
						putstr(&"\nFile not found.");
						drawstr(&"\nFile not found.");
					}

				}
				None => {
					putstr(&"Bad Directory Name\n");
					drawstr(&"Bad Directory Name\n");
				}
			}
		} else if(y.streq(&"mv")) {
			match buffer.getarg(' ', 1) {
				Some(mut filename) => {
					if filename.len() < 1 {
						putstr(&"Bad File Name\n");
						drawstr(&"Bad File Name\n");
						return;
					}
					// let cwdptr = &cwd as *dnode;
					// let dir = dnode::new(256, word, cwdptr as u32);
					// let x = cwd.add_child((&dir as *dnode) as u32);

					if (*cwd).cont_file(filename)
					{
						match buffer.getarg(' ', 2)
						{
							Some(mut destination) => 
							{
								if destination.len() < 1
								{
									putstr(&"\nDestination cannot be blank.");
									drawstr(&"\nDestination cannot be blank.");
									return;
								}

								if (*cwd).cont_file(filename)
								{
									let content = (*cwd).get_file(filename).get().content;
									let f = fs::file::new(destination, cwd, content);
									(*cwd).add_file(f);
									(*cwd).remove_file(filename);
									
								}

								
							}
							None => 
							{
								putstr(&"\nDestination cannot be blank.");
								drawstr(&"\nDestination cannot be blank.");
							}
						}
					}
					else
					{
						putstr(&"\nFile not found.");
						drawstr(&"\nFile not found.");
					}

				}
				None => {
					putstr(&"Bad Directory Name\n");
					drawstr(&"Bad Directory Name\n");
				}
			}
		} else if(y.streq(&"mkdir")) {
			match buffer.getarg(' ', 1) {
				Some(mut word) => {
					if word.len() < 1 {
						putstr(&"Bad Directory Name\n");
						drawstr(&"Bad Directory Name\n");
						return;
					}
					// let cwdptr = &cwd as *dnode;
					// let dir = dnode::new(256, word, cwdptr as u32);
					// let x = cwd.add_child((&dir as *dnode) as u32);

					let d = fs::directory::new(word, cwd);
					(*cwd).add_directory(d);

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
			putcstr((*cwd).name);
			drawcstr((*cwd).name, true, false);
		} else if(y.streq(&"wr")) {

			match buffer.getarg(' ', 1) {
				Some(mut filename) => {

					if filename.len() < 1 {
						putstr(&"Bad File Name\n");
						drawstr(&"Bad File Name\n");
						return;
					}
					let (mut command, mut end) = buffer.split(' ');
					let (mut filename, mut file_content) = end.split(' ');
							if file_content.len() < 1 {
								putstr(&"Content can't be empty\n");
								drawstr(&"Content can't be empty\n");
								return;
							}

							

							let f = fs::file::new(filename, cwd, file_content);
							(*cwd).add_file(f);

				

					
				}

				None => {
					putstr(&"Bad File Name\n");
						drawstr(&"Bad File Name\n");
				}
			}

			// putstr(&"\nTEST wr");
			// drawstr(&"\nTEST wr");
		} else if(y.streq(&"cowsay")) {
		    let (x, cowstr) = buffer.split(' ');
		    if cowstr.len() > 0 {
			cowsay(cowstr);
		    } else {
			putstr(&"\n Please give the cow something to say!");
			drawstr(&"\n Please give the cow something to say!");
		    }

		} /* else if y.streq(&"bg") {
		    match buffer.getarg(' ', 1) {
			Some(color) => {
			    if color.streq(&"red") {
				io::reset();
				io::set_bg(0x0000FF);
				io::fill_bg();
			    } else if color.streq(&"blue") {
				io::reset();
				io::set_bg(0x68320D);
				io::fill_bg();
			    } else if color.streq(&"green") {
				io::reset();
				io::set_bg(0x00FF00);
				io::fill_bg();
			    } else if color.streq(&"orange") {
				io::reset();
				io::set_bg(0x0370FF);
				io::fill_bg();
			    } else {
				putstr(&"\nNot a valid color");
				drawstr(&"\nNot a valid color");
			    }
			}
			None => {
			    putstr(&"\nNot a valid color");
			    drawstr(&"\nNot a valid color");
			}
		    }
		} */ else if y.streq(&"font") {
		    match buffer.getarg(' ', 1) {
			Some(color) => {
			    if color.streq(&"red") {
				io::set_fg(0x0000FF);
			    } else if color.streq(&"green") {
				io::set_fg(0x00FF00);
			    } else if color.streq(&"orange") {
				io::set_fg(0x0370FF);
			    } else {
				putstr(&"\nNot a valid color");
				drawstr(&"\nNot a valid color");
			    }
			}
			None => {
			    putstr(&"\nNot a valid color");
			    drawstr(&"\nNot a valid color");
			}
		    }
		} else {
			putstr(&"\nUnrecognized Command!");
			drawstr(&"\nUnrecognized Command!");
		}
	    y.destroy();
	}
	None        => { }
};
buffer.reset();
}

/* BUFFER MODIFICATION FUNCTIONS */
pub struct cstr {
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
	pub unsafe fn from_str(s: &str) -> cstr {
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
	pub unsafe fn eq(&self, other: &cstr) -> bool {
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
			else if (*(selfp as *u8) == delim as u8) && found {
				end.add_char(*(selfp as *u8));
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

/*
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
*/

struct dnode {
	children: *mut u32,
	curptr: uint,
	name: cstr,
	max: uint,
	parent: u32,
}

impl dnode {
	unsafe fn new(size: uint, name: cstr, parent: u32) -> dnode {
	// Sometimes this doesn't allocate enough memory and gets stuck...
	let (x, y) = heap.alloc(size);
	let this = dnode {
		children: x as *mut u32,
		curptr: 0,
		name: name,
		max: y / 4 as uint,
		parent: parent,
	};
	*(((this.children as u32)+(4*this.curptr) as u32) as *mut u32) = '\0' as u32;
	this
}

fn len(&self) -> uint { self.curptr }

unsafe fn add_child(&mut self, x: u32) -> bool{
	if (self.curptr == self.max) { return false; }
	*(((self.children as u32)+(4 * self.curptr) as u32) as *mut u32) = x;
	self.curptr += 1;
	if self.curptr > 0 {
		putstr(&"INCR\n");
	}
	*(((self.children as u32)+(4 * self.curptr) as u32) as *mut u32) = '\0' as u32;
	true
}

unsafe fn get_dir(&mut self, x: uint) -> u32{
	if x >= self.curptr { return '\0' as u32; }
	//raw memory address! just index it!
	*(((self.children as u32)+(4*x) as u32) as *mut u32)
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
