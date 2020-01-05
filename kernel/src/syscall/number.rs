/*
 * Copyright (C) 2018-2020 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see https://www.gnu.org/licenses.
 */
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_MMAP: usize = 9;
pub const SYS_BRK: usize = 12;
pub const SYS_SIGACTION: usize = 13;
pub const SYS_SIGPROCMASK: usize = 14;
pub const SYS_EXIT: usize = 60;
pub const SYS_TIME: usize = 96;
pub const SYS_SIGALSTACK: usize = 131;
pub const SYS_ARCHPRCTL: usize = 158;
pub const SYS_TKILL: usize = 200;
pub const SYS_SET_TID_ADDR: usize = 218;
pub const SYS_DEBUG: usize = 543;
