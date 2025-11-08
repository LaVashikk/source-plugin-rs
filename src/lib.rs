//! # Source Engine Server Plugin Template
//!
//! This crate provides a minimal, cross-platform template for building server plugins
//! for the Source Engine in Rust. It correctly handles the different Application Binary Interfaces (ABIs)
//! required by Windows and Linux.
//!
//! ## ABI Differences
//! - **Windows (32-bit):** The engine uses the `__thiscall` calling convention for C++ class methods.
//!   The `this` pointer is passed via the ECX register. We map this to `extern "thiscall"` in Rust.
//! - **Linux (32-bit):** The engine uses the Itanium C++ ABI, where the `this` pointer is passed
//!   as the first hidden argument on the stack. This is compatible with `extern "C"` in Rust.

use std::ffi::{c_char, c_void};
use std::ptr::null_mut;

/// A function pointer type for the `CreateInterface` function provided by the engine.
/// This signature is `extern "C"` on all platforms.
type CreateInterfaceFn = unsafe extern "C" fn(name: *const c_char, return_code: *mut i32) -> *mut c_void;


// --- Platform-Specific V-Table Definitions ---

#[cfg(target_os = "windows")]
/// V-Table structure for IServerPluginCallbacks on Windows.
/// All function pointers must use the `thiscall` calling convention.
#[repr(C)]
struct IServerPluginCallbacksVtable {
    load: unsafe extern "thiscall" fn(this: *mut c_void, factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> bool,
    unload: unsafe extern "thiscall" fn(this: *mut c_void),
    pause: unsafe extern "thiscall" fn(this: *mut c_void),
    unpause: unsafe extern "thiscall" fn(this: *mut c_void),
    get_plugin_description: unsafe extern "thiscall" fn(this: *mut c_void) -> *const c_char,
    level_init: unsafe extern "thiscall" fn(this: *mut c_void, map_name: *const c_char),
    server_activate: unsafe extern "thiscall" fn(this: *mut c_void, edict_list: *const c_void, edict_count: i32, client_max: i32),
    game_frame: unsafe extern "thiscall" fn(this: *mut c_void, simulating: bool),
    level_shutdown: unsafe extern "thiscall" fn(this: *mut c_void),
    client_active: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void),
    client_fully_connect: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void),
    client_disconnect: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void),
    client_put_in_server: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void, player_name: *const c_char),
    set_command_client: unsafe extern "thiscall" fn(this: *mut c_void, index: i32),
    client_settings_changed: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void),
    client_connect: unsafe extern "thiscall" fn(this: *mut c_void, allow_connect: *mut bool, entity: *const c_void, name: *const c_char, address: *const c_char, reject: *mut c_char, reject_len: i32) -> i32,
    client_command: unsafe extern "thiscall" fn(this: *mut c_void, entity: *const c_void, args: *const c_void) -> i32,
    network_id_validated: unsafe extern "thiscall" fn(this: *mut c_void, user_name: *const c_char, network_id: *const c_char) -> i32,
    on_query_cvar_value_finished: unsafe extern "thiscall" fn(this: *mut c_void, cookie: i32, entity: *const c_void, status: i32, cvar_name: *const c_char, cvar_value: *const c_char),
    on_edict_allocated: unsafe extern "thiscall" fn(this: *mut c_void, edict: *const c_void),
    on_edict_freed: unsafe extern "thiscall" fn(this: *mut c_void, edict: *const c_void),
}

#[cfg(target_os = "linux")]
/// V-Table structure for IServerPluginCallbacks on Linux.
/// All function pointers use the standard C calling convention.
#[repr(C)]
struct IServerPluginCallbacksVtable {
    load: unsafe extern "C" fn(this: *mut c_void, factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> bool,
    unload: unsafe extern "C" fn(this: *mut c_void),
    pause: unsafe extern "C" fn(this: *mut c_void),
    unpause: unsafe extern "C" fn(this: *mut c_void),
    get_plugin_description: unsafe extern "C" fn(this: *mut c_void) -> *const c_char,
    level_init: unsafe extern "C" fn(this: *mut c_void, map_name: *const c_char),
    server_activate: unsafe extern "C" fn(this: *mut c_void, edict_list: *const c_void, edict_count: i32, client_max: i32),
    game_frame: unsafe extern "C" fn(this: *mut c_void, simulating: bool),
    level_shutdown: unsafe extern "C" fn(this: *mut c_void),
    client_active: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void),
    client_fully_connect: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void),
    client_disconnect: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void),
    client_put_in_server: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void, player_name: *const c_char),
    set_command_client: unsafe extern "C" fn(this: *mut c_void, index: i32),
    client_settings_changed: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void),
    client_connect: unsafe extern "C" fn(this: *mut c_void, allow_connect: *mut bool, entity: *const c_void, name: *const c_char, address: *const c_char, reject: *mut c_char, reject_len: i32) -> i32,
    client_command: unsafe extern "C" fn(this: *mut c_void, entity: *const c_void, args: *const c_void) -> i32,
    network_id_validated: unsafe extern "C" fn(this: *mut c_void, user_name: *const c_char, network_id: *const c_char) -> i32,
    on_query_cvar_value_finished: unsafe extern "C" fn(this: *mut c_void, cookie: i32, entity: *const c_void, status: i32, cvar_name: *const c_char, cvar_value: *const c_char),
    on_edict_allocated: unsafe extern "C" fn(this: *mut c_void, edict: *const c_void),
    on_edict_freed: unsafe extern "C" fn(this: *mut c_void, edict: *const c_void),
}


/// Represents the plugin object itself. The layout must match the C++ equivalent:
/// a pointer to the virtual function table (`vtable`) must be the first member.
#[repr(C)]
struct ServerPlugin {
    vtable: *const IServerPluginCallbacksVtable,
}

// A macro to define a V-Table function with the correct calling convention for the target OS.
// This avoids duplicating the function body for both Windows and Linux.
macro_rules! define_vtable_fn {
    (fn $name:ident($($arg_name:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? $body:block) => {
        #[cfg(target_os = "windows")]
        unsafe extern "thiscall" fn $name(_this: *mut c_void, $($arg_name: $arg_ty),*) $(-> $ret_ty)? $body

        #[cfg(target_os = "linux")]
        unsafe extern "C" fn $name(_this: *mut c_void, $($arg_name: $arg_ty),*) $(-> $ret_ty)? $body
    };
}


// --- V-Table Function Implementations ---

/// A static, null-terminated byte string for the plugin description.
/// This ensures the pointer we return in `get_plugin_description` is always valid.
static PLUGIN_DESCRIPTION: &[u8] = b"Cross-Platform Rust Plugin Template\0";

// Each function is defined only once using the macro.
define_vtable_fn!(fn load(_factory: CreateInterfaceFn, _game_server_factory: CreateInterfaceFn) -> bool {
    eprintln!("[RUST_PLUGIN] ==> IServerPluginCallbacks::load() called.");
    true
});

define_vtable_fn!(fn unload() {
    eprintln!("[RUST_PLUGIN] ==> IServerPluginCallbacks::unload() called.");
});

define_vtable_fn!(fn server_activate(_edict_list: *const c_void, _edict_count: i32, _client_max: i32) {
    eprintln!("[RUST_PLUGIN] ==> IServerPluginCallbacks::server_activate() called. Map has loaded.");
    // This is the safest place to initialize hooks or game-state-dependent logic.
});

define_vtable_fn!(fn get_plugin_description() -> *const c_char {
    eprintln!("[RUST_PLUGIN] ==> IServerPluginCallbacks::get_plugin_description() called.");
    PLUGIN_DESCRIPTION.as_ptr() as *const c_char
});

// --- Stub implementations for the remaining interface methods ---
define_vtable_fn!(fn pause() {});
define_vtable_fn!(fn unpause() {});
define_vtable_fn!(fn level_init(_map_name: *const c_char) {});
define_vtable_fn!(fn game_frame(_simulating: bool) {});
define_vtable_fn!(fn level_shutdown() {});
define_vtable_fn!(fn client_active(_entity: *const c_void) {});
define_vtable_fn!(fn client_fully_connect(_entity: *const c_void) {});
define_vtable_fn!(fn client_disconnect(_entity: *const c_void) {});
define_vtable_fn!(fn client_put_in_server(_entity: *const c_void, _player_name: *const c_char) {});
define_vtable_fn!(fn set_command_client(_index: i32) {});
define_vtable_fn!(fn client_settings_changed(_entity: *const c_void) {});
define_vtable_fn!(fn client_connect(_allow_connect: *mut bool, _entity: *const c_void, _name: *const c_char, _address: *const c_char, _reject: *mut c_char, _reject_len: i32) -> i32 { 0 });
define_vtable_fn!(fn client_command(_entity: *const c_void, _args: *const c_void) -> i32 { 0 });
define_vtable_fn!(fn network_id_validated(_user_name: *const c_char, _network_id: *const c_char) -> i32 { 0 });
define_vtable_fn!(fn on_query_cvar_value_finished(_cookie: i32, _entity: *const c_void, _status: i32, _cvar_name: *const c_char, _cvar_value: *const c_char) {});
define_vtable_fn!(fn on_edict_allocated(_edict: *const c_void) {});
define_vtable_fn!(fn on_edict_freed(_edict: *const c_void) {});


// --- Static Plugin Instance and V-Table Initialization ---

/// The static, global instance of our V-Table, populated with pointers to our functions.
/// This also needs to be conditionally compiled because the function pointer types differ.
#[cfg(any(target_os = "windows", target_os = "linux"))]
static PLUGIN_VTABLE: IServerPluginCallbacksVtable = IServerPluginCallbacksVtable {
    load,
    unload,
    pause,
    unpause,
    get_plugin_description,
    level_init,
    server_activate,
    game_frame,
    level_shutdown,
    client_active,
    client_fully_connect,
    client_disconnect,
    client_put_in_server,
    set_command_client,
    client_settings_changed,
    client_connect,
    client_command,
    network_id_validated,
    on_query_cvar_value_finished,
    on_edict_allocated,
    on_edict_freed,
};

/// The single, static instance of our plugin that the engine will interact with.
#[cfg(any(target_os = "windows", target_os = "linux"))]
static mut SERVER_PLUGIN: ServerPlugin = ServerPlugin {
    vtable: &PLUGIN_VTABLE,
};


// --- Engine Entry Point ---

/// The primary export that the Source Engine looks for to load the plugin.
/// Its signature is `extern "C"` on all platforms.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CreateInterface(name: *const c_char, return_code: *mut i32) -> *mut c_void {
    eprintln!("[RUST_PLUGIN] CreateInterface called...");

    let requested_interface = unsafe { std::ffi::CStr::from_ptr(name) }.to_string_lossy();
    eprintln!("[RUST_PLUGIN] ... engine is requesting interface: '{}'", requested_interface);

    // The engine requests this specific interface version.
    if requested_interface == "ISERVERPLUGINCALLBACKS003" {
        eprintln!("[RUST_PLUGIN] ... Match! Returning plugin instance.");
        if !return_code.is_null() {
            unsafe { *return_code = 0; } // VRI_OK
        }
        // Return a pointer to our static plugin instance.
        return &raw mut SERVER_PLUGIN as *mut _ as *mut c_void;
    }

    eprintln!("[RUST_PLUGIN] ... No match. Returning null.");
    if !return_code.is_null() {
        unsafe { *return_code = 1 }; // VRI_FAILED
    }
    null_mut()
}
