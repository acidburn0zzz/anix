use crate::gdt;

pub fn switch_to_usermode(){
       /*__asm__ ("movl %%esp,%%eax\n\t" \
			 "pushl %%ecx\n\t" \
			 "pushl %%eax\n\t" \
			 "pushfl\n\t" \
			 "pushl %%ebx\n\t" \
			 "pushl $1f\n\t" \
			 "iret\n" \
			 "1:\tmovw %%cx,%%ds\n\t" \
			 "movw %%cx,%%es\n\t" \
			 "movw %%cx,%%fs\n\t" \
			 "movw %%cx,%%gs" \
			 ::"b"(USER_CODE_SEL),"c"(USER_DATA_SEL));
*/
}

pub fn task1(){
        loop{print!("a");};
        return;
}

pub fn init_user(){
	unsafe{
		println!("Switch");
		switch_to_usermode();
		println!("Success! You are now in user mode!");
	}
}
