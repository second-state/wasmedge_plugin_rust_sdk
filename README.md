# A Rust SDK for creating WasmEdge Plugins

WasmEdge plugins are packaged host functions that allow Wasm programs running inside WasmEdge to access native functions and vice versa. The plugin system makes WasmEdge extensible.

## Example 1: Hello world -- Wasm calls plugin

This demo shows how to call a host function registered in the plugin from a Wasm program running inside WasmEdge.

## Example 2: Stateful plugin -- Wasm calls plugin

This demo shows how to manage application state in the plugin, and have the state available to the Wasm programs through host functions.

### Plugin

The plugin source code is [here](examples/plugin/stateful_plugin/). You can build it with

```bash
cd examples/plugin/stateful_plugin
cargo build --release
```

The build result is a `libstateful_plugin.so` file that you can copy into WasmEdge's `lib` directory to install as a plugin.

```bash
cp ../../../target/release/libstateful_plugin.so /usr/local/lib/wasmedge/
... or ...
cp ../../../target/release/libstateful_plugin.so ~/.wasmedge/lib
```

The plugin maintains an internal state in a `(x,y)` tuple, and exposes two host functions, `add_x()` and `add_y()` to manipulate the tuple value. The tuple is initialized to `(0,0)` in the plugin source code.

### Wasm program

The Wasm program source code is [here](examples/wasm/call_stateful_plugin/). It compiles into a Wasm program that calls the host functions in the plugin. Build it with

```bash
cd examples/wasm/call_stateful_plugin
cargo build --release
```

Run it with the following command. It calls the `add_x()` and `add_y()` host functions in the plugin to manipulate the tuple data in the plugin.

```bash
wasmedge ../../../target/wasm32-wasi/release/call_stateful_plugin.wasm
```

## Example 3: Memory access -- Plugin calls Wasm

This demo shows how a host function in the plugin accesses memory in the Wasm program. It allows host functions to exchange dynamic data (e.g., strings and arrays) with the Wasm program.

