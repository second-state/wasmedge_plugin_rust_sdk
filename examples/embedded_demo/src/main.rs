use wasmedge_plugin_sdk::{
    ast_module::Loader,
    config::Config,
    executor::Executor,
    module::{DefaultWasiModule, SyncInstance},
    store::Store,
};

fn main() {
    let config = Config::create().map(|mut cfg| {
        cfg.wasi(true);
        cfg
    });
    let loader = Loader::create(&config).unwrap();
    let wasm = std::fs::read("target/wasm32-wasi/release/hello_wasi.wasm").unwrap();
    let ast_module = loader.load_module_from_bytes(&wasm).unwrap();
    let executor = Executor::create(&config).unwrap();

    let mut store = Store::create().unwrap();
    let mut wasi = DefaultWasiModule::create(vec![], vec![], vec![]).unwrap();
    store
        .register_import_object(&executor, wasi.as_mut())
        .unwrap();

    let mut inst = SyncInstance::instance(executor, &mut store, &ast_module).unwrap();
    inst.call("_start", vec![]).unwrap();
}
