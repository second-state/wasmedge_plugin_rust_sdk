pub use lazy_static::lazy_static;
pub use paste::paste;
use std::any::TypeId;
use std::ffi::CString;
use std::slice;
pub use wasmedge_sys_ffi as ffi;

#[macro_export]
macro_rules! register_plugin {
    (
        plugin_name = $plugin_name:expr,
        plugin_description = $plugin_description:expr,
        version = ($major:expr, $minor: expr, $patch: expr, $build: expr),
        modules = [$({$module_name:expr, $module_description:expr, $module_create_fn:ident}),*]
    ) =>
    {
        wasmedge_plugin_sdk::plugin::register_plugin!(
            plugin_name = $plugin_name,
            plugin_description = $plugin_description,
            version = ($major, $minor, $patch, $build),
            modules = [$({$module_name, $module_description, $module_create_fn}),*],
            options = []
        );
    };
    (
        plugin_name = $plugin_name:expr,
        plugin_description = $plugin_description:expr,
        version = ($major:expr, $minor: expr, $patch: expr, $build: expr),
        modules = [$({$module_name:expr, $module_description:expr, $module_create_fn:ident}),*],
        options = [$(
                {
                    $option_name:expr,
                    $option_description:expr,
                    $option_type:ty,
                    $option_default:expr
                }
            ),*]
    ) =>
    {
        wasmedge_plugin_sdk::plugin::paste! {
            use std::sync::atomic::{AtomicPtr, Ordering};
            use wasmedge_plugin_sdk::plugin::ffi::*;
            use wasmedge_plugin_sdk::plugin::Placeholder;

            // Generate option getter
            $(
                pub fn [<$option_name>]() -> *const $option_type {
                    [<$option_name:upper _STORAGE>].load(Ordering::SeqCst)
                }
            )*

            #[export_name = "WasmEdge_Plugin_GetDescriptor"]
            pub extern "C" fn plugin_hook() -> *const WasmEdge_PluginDescriptor {
                PLUGIN_DESCRIPTION.load(Ordering::SeqCst)
            }

            // Generate module create extern C wrap functions
            $(
                unsafe extern "C" fn [<$module_name _ $module_create_fn _wrap>](
                    _: *const WasmEdge_ModuleDescriptor,
                ) -> *mut WasmEdge_ModuleInstanceContext {
                    $module_create_fn().into()
                }
            )*

            // Generate static options vec
            static mut [<PROGRAM_OPTION_VEC>]: Vec<WasmEdge_ProgramOption> = Vec::new();

            // Generate static module description vec
            static mut [<MODULE_DESCRIPTION_VEC>]: Vec<WasmEdge_ModuleDescriptor> = Vec::new();

            wasmedge_plugin_sdk::plugin::lazy_static! {
                // Generate options storage and default value
                $(
                    static ref [<$option_name:upper _DEFAULT>]: AtomicPtr<$option_type> = {
                        let v = Box::new($option_default);
                        AtomicPtr::new(Box::into_raw(v))
                    };

                    static ref [<$option_name:upper _STORAGE>]: AtomicPtr<$option_type> = {
                        let v = $option_type::create_placeholder();
                        AtomicPtr::new(Box::into_raw(v))
                    };
                )*

                // Generate options content
                static ref [<PROGRAM_OPTIONS>]: AtomicPtr<WasmEdge_ProgramOption> =
                {
                    unsafe {
                        $(
                            [<PROGRAM_OPTION_VEC>].push(
                                WasmEdge_ProgramOption {
                                    Name: concat!($option_name,'\0').as_ptr().cast(),
                                    Description: concat!($option_description,'\0').as_ptr().cast(),
                                    Type: wasmedge_plugin_sdk::plugin::select_type::<$option_type>(),
                                    Storage: [<$option_name:upper _STORAGE>].load(Ordering::SeqCst)
                                        as *const _ as *mut ::std::os::raw::c_void,
                                    DefaultValue: [<$option_name:upper _DEFAULT>].load(Ordering::SeqCst)
                                        as *const _ as *const ::std::os::raw::c_void,
                                }
                            );
                        )*
                        AtomicPtr::new([<PROGRAM_OPTION_VEC>].as_mut_ptr())
                    }
                };

                // Generate module descriptions content
                static ref [<MODULE_DESCRIPTIONS>]: AtomicPtr<WasmEdge_ModuleDescriptor> =
                {
                    unsafe {
                        $(
                            [<MODULE_DESCRIPTION_VEC>].push(
                                WasmEdge_ModuleDescriptor {
                                    Name: concat!($module_name,'\0').as_ptr().cast(),
                                    Description: concat!($module_description,'\0').as_ptr().cast(),
                                    Create: Some([<$module_name _ $module_create_fn _wrap>]),
                                }
                            );
                        )*
                        AtomicPtr::new([<MODULE_DESCRIPTION_VEC>].as_mut_ptr())
                    }
                };

                // Generate plugin description content
                static ref PLUGIN_DESCRIPTION: AtomicPtr<WasmEdge_PluginDescriptor> =
                {
                    let plugin_description = Box::new(WasmEdge_PluginDescriptor {
                        Name: concat!($plugin_name,'\0').as_ptr().cast(),
                        Description: concat!($plugin_description,'\0').as_ptr().cast(),
                        APIVersion: WasmEdge_Plugin_CurrentAPIVersion,
                        Version: WasmEdge_PluginVersionData {
                            Major: $major,
                            Minor: $minor,
                            Patch: $patch,
                            Build: $build,
                        },
                        ModuleCount: wasmedge_plugin_sdk::plugin::count(&[$($module_name),*]),
                        ModuleDescriptions: [<MODULE_DESCRIPTIONS>].load(Ordering::SeqCst),
                        ProgramOptionCount: wasmedge_plugin_sdk::plugin::count(&[$($option_name),*]),
                        ProgramOptions: [<PROGRAM_OPTIONS>].load(Ordering::SeqCst),
                    });
                    AtomicPtr::new(Box::into_raw(plugin_description))
                };
            }
        }
    };
}

pub const fn count<const N: usize>(_: &'static [&str; N]) -> u32 {
    return N as u32;
}

// Define OptionString like WasmEdgeString but without derive(Copy)
#[repr(C)]
#[derive(Debug)]
pub struct OptionString {
    pub length: u32,
    pub buf: *const ::std::os::raw::c_char,
}
impl From<OptionString> for ffi::WasmEdge_String {
    fn from(source: OptionString) -> ffi::WasmEdge_String {
        ffi::WasmEdge_String {
            Length: source.length,
            Buf: source.buf,
        }
    }
}
impl OptionString {
    pub fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let buf_slice =
            unsafe { slice::from_raw_parts(self.buf as *const u8, self.length as usize) };
        let vec_buf = Vec::from(buf_slice);
        let c_string = CString::new(vec_buf)?;
        Ok(c_string.into_string()?)
    }
}
impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.buf as *mut i8);
        }
    }
}
pub trait Placeholder {
    fn create_placeholder() -> Box<Self>;
}
impl<T> Placeholder for T
where
    T: Default + 'static + Copy,
{
    fn create_placeholder() -> Box<Self> {
        Box::new(T::default())
    }
}
impl Placeholder for OptionString {
    fn create_placeholder() -> Box<Self> {
        let length: u32 = 128;
        let buf = vec![0i8; length as usize].into_boxed_slice();
        let buf_ptr = Box::into_raw(buf) as *const u8;
        Box::new(OptionString {
            length,
            buf: buf_ptr,
        })
    }
}

pub fn select_type<T: 'static>() -> ffi::WasmEdge_ProgramOptionType {
    let type_id = TypeId::of::<T>();
    if type_id == TypeId::of::<bool>() {
        ffi::WasmEdge_ProgramOptionType_Toggle
    } else if type_id == TypeId::of::<i8>() {
        ffi::WasmEdge_ProgramOptionType_Int8
    } else if type_id == TypeId::of::<i16>() {
        ffi::WasmEdge_ProgramOptionType_Int16
    } else if type_id == TypeId::of::<i32>() {
        ffi::WasmEdge_ProgramOptionType_Int32
    } else if type_id == TypeId::of::<i64>() {
        ffi::WasmEdge_ProgramOptionType_Int64
    } else if type_id == TypeId::of::<u8>() {
        ffi::WasmEdge_ProgramOptionType_UInt8
    } else if type_id == TypeId::of::<u16>() {
        ffi::WasmEdge_ProgramOptionType_UInt16
    } else if type_id == TypeId::of::<u32>() {
        ffi::WasmEdge_ProgramOptionType_UInt32
    } else if type_id == TypeId::of::<u64>() {
        ffi::WasmEdge_ProgramOptionType_UInt64
    } else if type_id == TypeId::of::<f32>() {
        ffi::WasmEdge_ProgramOptionType_Float
    } else if type_id == TypeId::of::<f64>() {
        ffi::WasmEdge_ProgramOptionType_Double
    } else if type_id == TypeId::of::<OptionString>() {
        ffi::WasmEdge_ProgramOptionType_String
    } else {
        panic!("Unsupported option type")
    }
}

#[macro_export]
macro_rules! option_string {
    ($value:expr) => {
        OptionString {
            length: $value.len() as u32 + 1,
            buf: concat!($value, '\0').as_ptr().cast(),
        }
    };
}
pub use option_string;
pub use register_plugin;
