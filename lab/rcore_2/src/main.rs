#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
use core::fmt::{Write ,self};
use core::panic::PanicInfo;
mod sbi;
#[panic_handler]
fn panic(info :&PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {} : {}{}",location.file(),location.line(),info.message().unwrap());
    }
    else {
        println!("Panicked: {}", info.message().unwrap());   
    }
    shutdown() 
}
const SBI_SHUTDOWN :usize = 8;
const SBI_CONSLOE_PUTCHAR :usize = 1;
const SYSCALL_EXIT :usize = 93;
const SYSCALL_WRITE :usize = 64;
fn syscall (id :usize ,args :[usize ;3] ) -> isize {
    let mut ret : isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret 
}
pub fn console_putchar(c :usize){
    syscall(SBI_CONSLOE_PUTCHAR , [ c, 0 ,0]);
}
struct Stdout ;
impl Write for Stdout{
    fn write_str(& mut self , s :&str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
    Ok(())
    }
    
}
pub fn shutdown() -> ! {
    sbi::sbi_call(SBI_SHUTDOWN ,0 ,0, 0);
    panic!("It should shutdown");
}
pub fn sys_exit(xstate :i32) -> isize {
    syscall(SYSCALL_EXIT , [xstate as usize ,0 ,0])
}
pub fn sys_write(fd :usize, buffer :&[u8]) -> isize{
    syscall(SYSCALL_WRITE ,[fd ,buffer.as_ptr() as usize , buffer.len()])
}
pub fn print(args :fmt::Arguments){
    Stdout.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! print {
    ($fmt: literal $(,$($args: tt)+)?) => {
        $crate::console::print(format_ages!($fmt $(, $($args)+)?));
    };
}
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
global_asm!(include_str!("entry.asm"));
fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}
#[no_mangle]
pub fn rust_main () -> ! {
    loop{}
}