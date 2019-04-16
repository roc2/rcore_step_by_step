#![no_std]
#![no_main]

#[macro_use]
extern crate rcore32_user;

#[no_mangle]
pub fn main() {
    println!("Hello RISCV !");
    let fd = rcore32_user::syscall::sys_open("sh", 0);
    println!("syscall open ret {}", fd);
    let mut buf = [0u8;10];
    rcore32_user::syscall::sys_read(fd as usize, &mut buf);
    for &ch in &buf {
        print!("{:?}", ch as char);
    }
}
