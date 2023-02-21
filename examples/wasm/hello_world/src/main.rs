#[link(wasm_import_module = "hello_module")]
extern "C" {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
