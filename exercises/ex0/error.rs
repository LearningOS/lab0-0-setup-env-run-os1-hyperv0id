use std::ptr;

fn main() {
    // Attempt to access unknown memory address
    let unknown_ptr: *const i32 = 0x12345678 as *const i32;
    
    // Read the value at the unknown address (volatile read)
    let _value = unsafe { ptr::read_volatile(unknown_ptr) };
}
// 139 == SIGSEGV信号
// [1]    37917 segmentation fault (core dumped)  ./error