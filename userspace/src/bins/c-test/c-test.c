void main() {
        char *string = "Hello world!\n";
        asm("syscall" :
                      : "a"(1),              // Syscall number
                        "b"((void *)string),              // File ID (stdout)
                        "c"((void *)string), // Address of the string
                        "d"(13)              // Size of the string
        );
        asm("syscall" :: "a"(60));
}
