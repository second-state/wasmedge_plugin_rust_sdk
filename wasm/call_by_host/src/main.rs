fn main() {
    println!("Hello, world!");
}

#[no_mangle]
pub unsafe extern "C" fn call_by_host(i: i32) {
    println!("accept a int({i}) from host");
}
