/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
use super::pic::*;
use crate::screen::{fill, ColorCode, Color};

//Types
pub enum Type{
	FILE = 0x01,
	DIRECTORY = 0x02,
	CHARDEVICE = 0x03,
	BLOCKDEVICE = 0x04,
	PIPE = 0x05,
}

pub enum Flag{
	O_RDONLY = 0x0001,
	O_WRONLY = 0x0002,
	O_RDWR = 0x0003,
	O_APPEND = 0x0008,
}

extern "C"{
	fn test();
}

pub fn fsmain(){
	fill(ColorCode::new(Color::Black, Color::Black));
	unsafe{
		test();
	}
}

//-------------------------------------------------------------------------Files-----------------------------------------------------
pub struct FSNODE {
  pub name: &'static str, //Filename.
  pub r#type: Type, //Type
  pub flags: Flag, //Read, write, append, ...
  pub uid: u32,
  pub gid: u32,
  pub size: u32,
}

pub struct DIRENTRY {
  pub name: [char; 128], //Directory name
  pub inodes: [u32; 10], //Inodes link
  pub number_of_inodes: u32, //Inodes number
}

use ascii::*;

impl core::fmt::Display for Flag{
	fn fmt(self: &Self, format: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		Ok(())
	}
}
impl core::fmt::Display for Type{
	fn fmt(self: &Self, format: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		Ok(())
	}
}

impl FSNODE {
	pub fn read(self: Self){

	}

	pub fn write(self: Self){

	}

	pub fn close(self: Self){

	}
	
	pub fn stat(self: Self){
		print!("{} is a {}. You can {} on it and it size is {}", self.name, self.r#type, self.flags, self.size);
	}
}
pub fn open(path: &'static str, flags: Flag) -> File {
		
	//Name
	//let mut name = "";
	//let n = name.split("/"); //TODO: Not work
	
	//let arr: Vec<&str> = n.collect();
	
	//print!("LEN");
	//let l = arr.len();
	//print!("ASCII");
	//let na = arr[l].as_ascii_str();
		
	//match na{
	//	Ok(n) => name = n.as_str(),
	//	Err(e) => name = "Error",
	//}
		
	FSNODE{
		name: path,
		r#type: Type::FILE,
		flags: flags,
		uid: 0,
		gid: 0,
		size: 0,
	}
}

type File = FSNODE;
type Dir = DIRENTRY;
