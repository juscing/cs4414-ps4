use core::*;
use kernel::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::slice::{Items, Slice, iter, unchecked_get, unchecked_mut_get};
use kernel::vec::Vec;
use super::super::platform::*;
use kernel::sgash::cstr;
use kernel::sgash::drawcstr;
use kernel::sgash::putcstr;
use kernel::sgash::putstr;
use kernel::sgash::drawstr;

pub struct directory {
    name: cstr,
    parent: *directory,
    fchildren: *mut Vec<file>,
    dchildren: *mut Vec<directory>,
}

impl directory {
    pub unsafe fn new(title: cstr, parent: *directory) -> directory {
        let mut this = directory {
            name: title,
            fchildren: &mut Vec::new() as *mut Vec<file>,
            dchildren: &mut Vec::new() as *mut Vec<directory>,
            parent: parent,
        };
        this
    }

    pub unsafe fn add_directory(&mut self, d : directory) { 
       (*self.dchildren).push(d);
    }

    pub unsafe fn add_file(&mut self, f : file) { 
       (*self.fchildren).push(f);
    }

    // pub unsafe fn move(mut self, filename : cstr, destination : cstr) { 

    //     let mut flag = false;
    //     let mut new_vec = &mut Vec::new() as *mut Vec<file>;

    //     for fi in iter((*self.fchildren).as_slice()) {
    //         if fi.name.eq(&filename) {
    //             flag = true;

    //             let f = file::new(destination, &self, fi.content);

    //             (*new_vec).push(f);
    //             continue;
    //         }
    //         (*new_vec).push(*fi);
            
    //     }

    //     if flag
    //     {
    //         putstr(&"\nMove");
    //         self.fchildren = new_vec;
    //     }        
        
    // }

    pub unsafe fn remove_file(&mut self, filename : cstr) { 

        let mut flag = false;
        let mut new_vec = &mut Vec::new() as *mut Vec<file>;

        for fi in iter((*self.fchildren).as_slice()) {
            if fi.name.eq(&filename) {
                flag = true;
                continue;
            }
            (*new_vec).push(*fi);
        }

        if flag
        {
            (*self.fchildren).truncate(0);
            for fi in iter((*new_vec).as_slice()) {
               (*self.fchildren).push(*fi);
            }
        }        
        
    }
    
    pub unsafe fn remove_dir(&mut self, dirname: cstr) -> bool {
    	let mut flag = false;
    	let mut new_vec = &mut Vec::new() as *mut Vec<directory>;
    	
    	for &mut dir in iter((*self.dchildren).as_slice()) {
    	    /*
    	    let mut x = dir.dchildren as u32;
    	    let mut y = x as *mut Vec<directory>;
    	    let mut z = *y;
    	    if z.len() == 0 {
    	    */
    		if dir.name.eq(&dirname) {
    		    flag = true;
    		    continue;
    		}
    	    //}
    	    (*new_vec).push(dir);
    	}

    	if flag
        {
            (*self.dchildren).truncate(0);
            for dir in iter((*new_vec).as_slice()) {
               (*self.dchildren).push(*dir);
            }
        }

        return flag;
    }
}


pub struct file {
    name: cstr,
    parent: *directory,
    content: cstr,
}

impl file {
    pub unsafe fn new(title: cstr, parent: *directory, content: cstr) -> file {
        let this = file {
            name: title,
            content: content,
            parent: parent,
        };
        this
    }
}


pub unsafe fn listDir(givenDir: directory) {

    for fi in iter((*givenDir.fchildren).as_slice()) {
        putcstr(fi.name);
        drawcstr(fi.name, true, false);
    }

    for dir in iter((*givenDir.dchildren).as_slice()) {
        putcstr(dir.name);
        drawcstr(dir.name, true, false);
    }






    // for dir in *(givenDir.dchildren) {
    //     let name = dir.name;
    //     ret.push(name);
    // }
    // for fi in *(givenDir.fchildren) {
    //     let name = fi.name;
    //     ret.push(name);
    // }
}

pub unsafe fn cont_file(givenDir: directory, name: cstr) -> bool {
    for fi in iter((*givenDir.fchildren).as_slice()) {
        if fi.name.eq(&name) {
	    return true;
	}
    }
    return false;
}

pub unsafe fn cont_dir(givenDir: directory, name: cstr) -> bool {
    for di in iter((*givenDir.dchildren).as_slice()) {
        if di.name.eq(&name) {
	    return true;
	}
    }
    return false;
}

pub unsafe fn get_file(givenDir: directory, name: cstr) -> Option<&file> {
    for fi in iter((*givenDir.fchildren).as_slice()) {
        if fi.name.eq(&name) {
	    return Some(fi);
	}
    }
    return None;
}

pub unsafe fn cat(givenDir: directory, filename: cstr) {
    let file = get_file(givenDir, filename);
    drawcstr(file.get().content, true, false);
    putcstr(file.get().content);
}

pub unsafe fn cd(givenDir: directory, goal: cstr) -> (bool,directory) {
    for dir in iter((*givenDir.dchildren).as_slice()) {
        if dir.name.eq(&goal) {
            return (true,*dir)
        }
    }
    return (false,givenDir)
}

/*
pub fn open(node: *tree_node, file: cstr) -> (*tree_node, bool, bool)
{
    if dir.isLeaf() || file == ""
    {
	return (node, dir.isLeaf(), file == "");
    }
    let mut children: uint = (*node).child_count;
    let mut i: uint = 0;
    let mut split = file.before('/');
    while i < children
    {
	if (*node).children[i].name == k
	{
	    return open((*node).children[i], file.remainder('/'));
	} else
	{
	    i += 1;
	}
    }
    return cstr::new();
}
*/