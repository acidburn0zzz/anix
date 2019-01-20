/*Copyright (C) 2018-2019 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
pub struct Time {
    pub deciseconds: u8,
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

pub static mut time: Time = Time {deciseconds: 0, seconds: 0, minutes: 0, hours: 0};

pub fn sleep(timeForSleep: u8){
	unsafe{
		let secondsForSleep = time.seconds + timeForSleep;
	
		//let minutesForSleep = time.minute;
		//if(secondsForSleep >= 60){minutesForSleep += secondsForSleep / 60}
		//secondsForSleep = secondsForSleep % 60;
	
		while(true){
			if(time.seconds == secondsForSleep){
				break;
			}
		}
	}
}
