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
        let this = directory {
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

    let mut count = 0;

    for fi in iter((*givenDir.fchildren).as_slice()) {
        putcstr(fi.name);
        drawcstr(fi.name, true, false);
        count += 1;
    }

    for dir in iter((*givenDir.dchildren).as_slice()) {
        putcstr(dir.name);
        drawcstr(dir.name, true, false);
    }

    

    if count > 0
    {
        putstr(&"works");
        drawstr(&"works");
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

pub fn append(node: *tree_node, file: cstr, content: cstr) -> bool
{
    let (mut f, _, _) = open(node, file);
    if f == cstr::new()
    {
	return false;
    }
    let mut x = 0;
    let mut f_contents = (*f).contents;
    while x < content.len()
    {
	let b = f_contents.push_char(content.char_at(x));
	if !b
	{
	    return false;
	}
    }
    let (*f).contents = f_contents;
    return true;
}

pub fn new(node: *tree_node, dir: cstr, name: cstr) -> bool
{
    let (mut n, _, _) = open(node, file);
    if !n.isLeaf()
    {
	n.insert(name);
    }
    return false;

}
*/