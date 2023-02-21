mod plugin {
    #[link(wasm_import_module = "stateful_module")]
    extern "C" {
        pub fn add_x(v: i32) -> i32;
        pub fn add_y(v: i32) -> i32;
    }
}

fn main() {
    unsafe {
        let x = plugin::add_x(1);
        let y = plugin::add_y(1);
        println!("[wasm] x:{} y:{}", x, y);
    }
}
