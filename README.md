# A Rust SDK for creating WasmEdge Plugins

WasmEdge plugins are packaged host functions that allow Wasm programs running inside WasmEdge to access native functions.

## Example 1: Wasm calls plugin

### Plugin

The plugin source code is [here](examples/plugin_demo). You can build it with

```bash
cd examples/plugin_demo
cargo build --release
```

The build result is a `libplugin_demo.so` file that you can copy into WasmEdge's `lib` directory to install as a plugin.

```bash
cp ../../target/release/libplugin_demo.so ~/.wasmedge/lib
... or ...
cp ../../target/release/libplugin_demo.so /usr/local/lib
```

The plugin maintains an internal state in a `(x,y)` tuple, and exposes two host functions, `add_x()` and `add_y()` to manipulate the tuple value. The tuple is initialized to `(0,0)` in the plugin source code.

### Wasm program

The Wasm program source code is [here](wasm/call_plugin). It compiles into a Wasm program that calls the host functions in the plugin. Build it with

```bash
cd wasm/call_plugin
cargo build --release
```

Run it with the following command. It calls the `add_x()` and `add_y()` host functions in the plugin to manipulate the tuple data in the plugin.

```bash
wasmedge ../../target/wasm32-wasi/release/call_plugin.wasm
```



