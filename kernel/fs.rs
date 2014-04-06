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

    for dir in iter((*givenDir.dchildren).as_slice()) {
        putcstr(dir.name);
        drawcstr(dir.name, true, false);
    }
    for fi in iter((*givenDir.fchildren).as_slice()) {
        putcstr(fi.name);
        drawcstr(fi.name, true, false);
    }
}

pub unsafe fn cd(givenDir: directory, goal: cstr) -> (bool,directory) {
    for dir in iter((*givenDir.dchildren).as_slice()) {
        if dir.name.eq(&goal) {
            return (true,*dir)
        }
    }
    return (false,givenDir)
}
