# Source Engine Rust Plugin (source-plugin-rs)

A minimal, cross-platform proof-of-concept for creating Source Engine server plugins using Rust.

> [!IMPORTANT]
> This is a proof-of-concept and has only been tested to load successfully in **Portal 2** on Windows and Linux. It serves as a starting point and demonstrates the correct ABI setup for both platforms.

## Building

### Windows (32-bit)

1.  Add the target toolchain:
    ```sh
    rustup target add i686-pc-windows-msvc
    ```
2.  Build the project:
    ```sh
    cargo build --target i686-pc-windows-msvc --release
    ```
3.  The output will be `target/i686-pc-windows-msvc/release/source_plugin_rs.dll`.

### Linux (32-bit)

1.  Install multilib build tools (e.g., `sudo apt-get install gcc-multilib`).
2.  Add the target toolchain:
    ```sh
    rustup target add i686-unknown-linux-gnu
    ```
3.  Build the project:
    ```sh
    cargo build --target i686-unknown-linux-gnu --release
    ```
4.  The output will be `target/i686-unknown-linux-gnu/release/libsource_plugin_rs.so`.

### Key Functions

The plugin implements `IServerPluginCallbacks003`:

| Function | When Called | Use Case |
|----------|------------|----------|
| `load()` | Plugin loaded | Initialize resources |
| `server_activate()` | Map loaded | Hook game state |
| `game_frame()` | Every tick | Game logic |
| `unload()` | Plugin unloaded | Cleanup |

See [Valve's SDK docs](https://developer.valvesoftware.com/wiki/Server_plugins) for details.

### Adding Your Logic

Edit the functions in `src/lib.rs`:

```rust
define_vtable_fn!(fn server_activate(_edict_list: *const c_void, _edict_count: i32, _client_max: i32) {
    eprintln!("[RUST_PLUGIN] Map loaded! Let's do something cool...");

    // Your code here
});
```

## Contributing
Contributions welcome! Please open an issue or PR.

**Credits:** Thanks to **[0xNULLderef](https://github.com/0xNULLderef)** for technical guidance.

## License
MIT License - see [LICENSE](LICENSE) file.
