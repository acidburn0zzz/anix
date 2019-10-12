# Anix

Anix is an operating system under GPL licence. So, you can modify and redistribute it.

### Installation
--------------------------

Anix works on real hardware or in Qemu. if you want to use it on real hardware, you must have an USB key or a portable disk.

#### To install on real hardware:
*   First, install Rust (with [Rustup](https://rustup.rs/)), make and
    [Nasm](https://nasm.us) (on Debian or on Ubuntu `sudo apt-get install nasm`)
*   Second, install [Xargo](https://github.com/japaric/xargo) (by running `cargo install xargo`)
*   Install the nightly toolchain with Rustup (`rustup toolchain install nightly`) and set it as the default toolchain (`rustup override set nightly` in the Anix root)
*   Add the rust-src component (`rustup component install rust-src`)
*   Install lld (on Debian open a terminal and type `sudo apt-get install lld`,
    on Ubuntu 16.04 open a terminal and type `sudo apt-get install lld-6.0`, on
    Ubuntu 18.04 open a terminal and type `sudo apt-get install lld-8.0` or
    search `ld.lld your-distribution-name` in internet)
*   Finally, open a terminal and type `make` in the Anix root directory

#### To install in Qemu:
*   First, install Qemu (on Debian or on Ubuntu open a terminal and type `sudo apt-get install qemu`) and
    qemu-kvm (on Debian or on Ubuntu open a terminal and type `sudo apt-get
    install qemu-kvm`)
*   Then, install Rust (with [Rustup](https://rustup.rs/) (install with the
    command on the homepage)) and [Xargo](https://github.com/japaric/xargo) (by running `cargo install xargo`)
    and [Nasm](https://nasm.us) (on Debian or on Ubuntu open a terminal and type `sudo apt-get install nasm`)
*   Install the nightly toolchain with Rustup (`rustup toolchain install nightly`) and set it as the default toolchain (`rustup override set nightly` in the Anix root)
*   Add the rust-src component (`rustup component add rust-src`)
*   Install lld (on Debian open a terminal and type `sudo apt-get install lld`,
    on Ubuntu 16.04 open a terminal and type `sudo apt-get install lld-6.0`, on
    Ubuntu 18.04 open a terminal and type `sudo apt-get install lld-8.0` or
    search `ld.lld your-distribution-name` in internet)
*   Install xorriso (on Debian or on Ubuntu open a terminal and type `sudo apt-get install xorriso`)
*   Finally, type `make qemu` in the Anix root directory

Tested on a Debian GNU/Linux bullseye/sid x86\_64
