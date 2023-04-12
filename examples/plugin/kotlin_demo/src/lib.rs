use std::ffi::CString;

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::{ValType, WasmVal},
};

pub fn create_module() -> SyncModule<()> {
    let mut module = SyncModule::create("js_code", ()).unwrap();

    fn f0<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        println!("captureStackTrace");
        unreachable!()
    }
    module
        .add_func(
            "kotlin.captureStackTrace",
            (vec![], vec![ValType::ExternRef]),
            f0,
        )
        .unwrap();

    fn f1<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.throwJsError",
            (
                vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
                vec![ValType::AnyRef],
            ),
            f1,
        )
        .unwrap();

    fn f2<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.getJsEmptyString",
            (vec![], vec![ValType::ExternRef]),
            f2,
        )
        .unwrap();

    fn f3<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.getJsTrue",
            (vec![], vec![ValType::ExternRef]),
            f3,
        )
        .unwrap();

    fn f4<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.getJsFalse",
            (vec![], vec![ValType::ExternRef]),
            f4,
        )
        .unwrap();

    fn f5<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.stringLength",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f5,
        )
        .unwrap();

    fn f6<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.jsExportStringToWasm",
            (vec![ValType::I32, ValType::I32, ValType::I32], vec![]),
            f6,
        )
        .unwrap();

    fn f7<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.newJsArray",
            (vec![], vec![ValType::ExternRef]),
            f7,
        )
        .unwrap();

    fn f8<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.jsArrayPush",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f8,
        )
        .unwrap();

    fn f9<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.importStringFromWasm",
            (
                vec![ValType::I32, ValType::I32, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f9,
        )
        .unwrap();

    fn f10<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToString",
            (vec![ValType::ExternRef], vec![ValType::ExternRef]),
            f10,
        )
        .unwrap();

    fn f11<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefEquals",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::I32],
            ),
            f11,
        )
        .unwrap();

    fn f12<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefHashCode",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f12,
        )
        .unwrap();

    fn f13<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.isNullish",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f13,
        )
        .unwrap();

    fn f14<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToInt",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f14,
        )
        .unwrap();

    fn f15<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToBoolean",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f15,
        )
        .unwrap();

    fn f16<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToLong",
            (vec![ValType::ExternRef], vec![ValType::I64]),
            f16,
        )
        .unwrap();

    fn f17<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToFloat",
            (vec![ValType::ExternRef], vec![ValType::F32]),
            f17,
        )
        .unwrap();

    fn f18<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.externrefToDouble",
            (vec![ValType::ExternRef], vec![ValType::F64]),
            f18,
        )
        .unwrap();

    fn f19<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.intToExternref",
            (vec![ValType::I32], vec![ValType::ExternRef]),
            f19,
        )
        .unwrap();

    fn f20<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.longToExternref",
            (vec![ValType::I64], vec![ValType::ExternRef]),
            f20,
        )
        .unwrap();

    fn f21<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.floatToExternref",
            (vec![ValType::F32], vec![ValType::ExternRef]),
            f21,
        )
        .unwrap();

    fn f22<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.doubleToExternref",
            (vec![ValType::F64], vec![ValType::ExternRef]),
            f22,
        )
        .unwrap();

    fn f23<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.wasm.internal.tryGetOrSetExternrefBox_$external_fun",
            (
                vec![ValType::ExternRef, ValType::StructRef],
                vec![ValType::StructRef],
            ),
            f23,
        )
        .unwrap();

    fn f24<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.jsCatch",
            (vec![ValType::ExternRef], vec![ValType::ExternRef]),
            f24,
        )
        .unwrap();

    fn f25<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__convertKotlinClosureToJsClosure_1k75qb4u5dea2_0",
            (vec![ValType::StructRef], vec![ValType::ExternRef]),
            f25,
        )
        .unwrap();

    fn f26<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func("kotlin.js.jsThrow", (vec![ValType::StructRef], vec![]), f26)
        .unwrap();

    fn f27<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.io.printlnImpl",
            (vec![ValType::ExternRef], vec![]),
            f27,
        )
        .unwrap();

    fn f28<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.Promise_$external_fun",
            (vec![ValType::ExternRef], vec![ValType::ExternRef]),
            f28,
        )
        .unwrap();

    fn f29<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__callJsClosure_13wh4vickqu4q",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f29,
        )
        .unwrap();

    fn f30<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__callJsClosure_3dwlowct8k3kt",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f30,
        )
        .unwrap();

    fn f31<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__convertKotlinClosureToJsClosure_2gerki4j4a7ha",
            (vec![ValType::StructRef], vec![ValType::ExternRef]),
            f31,
        )
        .unwrap();

    fn f32<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.then_$external_fun",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f32,
        )
        .unwrap();

    fn f33<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__convertKotlinClosureToJsClosure_39lnmx82fr8a8_0",
            (vec![ValType::StructRef], vec![ValType::ExternRef]),
            f33,
        )
        .unwrap();

    fn f34<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.then_$external_fun_1",
            (
                vec![ValType::ExternRef, ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f34,
        )
        .unwrap();

    fn f35<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.__convertKotlinClosureToJsClosure_2v6y9vywgkg4n",
            (vec![ValType::StructRef], vec![ValType::ExternRef]),
            f35,
        )
        .unwrap();

    fn f36<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.catch_$external_fun",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f36,
        )
        .unwrap();

    fn f37<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.finally_$external_fun",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f37,
        )
        .unwrap();

    fn f38<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.Companion_$external_fun",
            (vec![], vec![ValType::ExternRef]),
            f38,
        )
        .unwrap();

    fn f39<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.reject_$external_fun",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f39,
        )
        .unwrap();

    fn f40<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.resolve_$external_fun",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f40,
        )
        .unwrap();

    fn f41<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.resolve_$external_fun_1",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f41,
        )
        .unwrap();

    fn f42<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.Companion_$external_object_getInstance",
            (vec![], vec![ValType::ExternRef]),
            f42,
        )
        .unwrap();

    fn f43<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.Companion_$external_class_instanceof",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f43,
        )
        .unwrap();

    fn f44<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.js.Promise_$external_class_instanceof",
            (vec![ValType::ExternRef], vec![ValType::I32]),
            f44,
        )
        .unwrap();

    fn f45<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.random.initialSeed",
            (vec![], vec![ValType::I32]),
            f45,
        )
        .unwrap();

    fn f46<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.xdescribe",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f46,
        )
        .unwrap();

    fn f47<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.describe",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f47,
        )
        .unwrap();

    fn f48<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.jsThrow",
            (vec![ValType::ExternRef], vec![]),
            f48,
        )
        .unwrap();

    fn f49<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.xit",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f49,
        )
        .unwrap();

    fn f50<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.xit",
            (vec![ValType::StructRef], vec![ValType::ExternRef]),
            f50,
        )
        .unwrap();

    fn f51<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.it",
            (vec![ValType::ExternRef, ValType::ExternRef], vec![]),
            f51,
        )
        .unwrap();

    fn f52<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.throwableToJsError",
            (
                vec![ValType::ExternRef, ValType::ExternRef],
                vec![ValType::ExternRef],
            ),
            f52,
        )
        .unwrap();

    fn f53<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.d8Arguments",
            (vec![], vec![ValType::ExternRef]),
            f53,
        )
        .unwrap();

    fn f54<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func(
            "kotlin.test.nodeArguments",
            (vec![], vec![ValType::ExternRef]),
            f54,
        )
        .unwrap();

    fn f55<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        _data: &'a mut (),
        _args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        unreachable!()
    }
    module
        .add_func("kotlin.test.isJasmine", (vec![], vec![ValType::I32]), f55)
        .unwrap();

    module
}

#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> PluginDescriptorRef {
    let mut builder = PluginBuilder::create(
        CString::new("kotlin plugin").unwrap(),
        CString::new("a kotlin demo plugin").unwrap(),
    );
    builder.add_module(
        CString::new("js_code").unwrap(),
        CString::new("a kotlin demo module").unwrap(),
        create_module,
    );

    builder.build()
}
