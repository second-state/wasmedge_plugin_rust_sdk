mod plugin0 {
    #[link(wasm_import_module = "demo_module")]
    extern "C" {
        pub fn add_x(v: i32) -> i32;
        pub fn add_y(v: i32) -> i32;
    }
}

mod plugin1 {
    #[link(wasm_import_module = "demo_module2")]
    extern "C" {
        pub fn add_x(v: i32) -> i32;
        pub fn add_y(v: i32) -> i32;
    }
}

fn main() {
    println!("Hello, world!");
    unsafe {
        let x = plugin0::add_x(1);
        let y = plugin0::add_y(1);
        println!("x:{} y:{}", x, y);
        let x = plugin1::add_x(1);
        let y = plugin1::add_y(1);
        println!("x:{} y:{}", x, y);
    }
}
