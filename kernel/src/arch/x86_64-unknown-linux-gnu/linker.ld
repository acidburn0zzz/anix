/*
 * Copyright (C) 2018-2019 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
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

ENTRY(start)

SECTIONS {
	. = 1M;

	.rodata : ALIGN(4K)
	{
		/* ensure that the multiboot header is at the beginning */
		KEEP(*(.multiboot_header))
		*(.rodata .rodata.*)
		. = ALIGN(4K);
	}

	.text : ALIGN(4K)
	{
		*(.text .text.*)
		. = ALIGN(4K);
	}

	.tdata : ALIGN(4K)
	{
		*(.tdata .tdata.*)
		. = ALIGN(4K);
	}
	
	.tbss : ALIGN(4K)
	{
		*(.tbss .tbss.*)
		. = ALIGN(4K);
	}
	
	.got : ALIGN(4K)
	{
		*(.got)
		. = ALIGN(4K);
	}
	
	.bss : ALIGN(4K)
	{
		*(.bss .bss.*)
		. = ALIGN(4K);
	}

	.eh_frame : ALIGN(4K)
	{
		*(.eh_frame)
		. = ALIGN(4K);
	}
	
	.fini : ALIGN(4K)
	{
		*(.fini)
		. = ALIGN(4K);
	}
	
	.data : ALIGN(4K)
	{
		*(.data .data.*)
		. = ALIGN(4K);
	}
	
	.data.rel.ro : ALIGN(4K) {
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	}

	.got.plt : ALIGN(4K)
	{
		*(.got.plt)
		. = ALIGN(4K);
	}

	.gcc_except_table : ALIGN(4K) {
		*(.gcc_except_table)
		. = ALIGN(4K);
	}
} 
