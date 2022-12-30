# bevy_wasm_scripting
Adds support for wasm/wat assets in Bevy, and enables easy scripting. This is enabled through the [wasmer](https://github.com/wasmerio/wasmer) crate. 

- [ ] Prepare for public use. This is very experimental! Please share any feedback in the Bevy Discord!
- [x] Scripts managed through bevy asset management
- [x] Scripts attached to components
- [x] Scripts attached to resources
- [x] Hot-reloading of component- and resource-based scripts
- [x] [Basic examples](examples)
- [x] Harmonize resource- and component-based scripts, as they could both be simpler to define.
- [ ] Put this through its paces with a game project, to find pain points
- [ ] Confirm safety of `WorldPointer` imports strategy.
- [ ] Investigate compilation performance and multi-threading options.
- [ ] Investigate memory usage.
- [x] Investigate cooperation with web builds.
- [ ] Configuration for Wasmer Tunables.
- [ ] Configuration for Wasmer compiler. (The `Cranelift` compiler is currently hardcoded.)
- [ ] Example game (probably a breakout clone with powerups)
- [ ] Rust -> wasm script example
- [ ] Lua -> wasm script example
- [ ] Other language examples?

# Examples
For component-based scripts:
```rust
fn main() {
    App::new()
    ...
        .add_plugin(WasmPlugin)
        .add_wasm_script_component::<AdderScript>()
        .add_startup_system(spawn_script_entity)
        .add_system(call_script_on_entity)
    ...
}

impl WasmScriptComponent for AdderScript {
    type ImportQueriedComponents = ();
    type ImportResources = ();
    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.handle
    }
}

fn spawn_script_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AdderScript {
        handle: asset_server.load("add_one.wat"),
        accumulator: 0,
    });
}

// 
fn call_script_on_entity(
    mut scripted_entities: Query<&mut AdderScript>,
    mut script_env: WasmScriptComponentEnv<AdderScript>,
) {
    for mut scripted_entity in scripted_entities.iter_mut() {
        if let Ok(new_val) = script_env.call_if_instantiated(
            &scripted_entity.handle,
            "main",
            scripted_entity.accumulator,
        ) {
            scripted_entity.accumulator = new_val;
        }
        println!("Accumulated value: {}", scripted_entity.accumulator);
    }
}
```

More examples, for hot reloading, resource-based scripts, and script imports, are available in the [examples](examples) directory.

# Web Build Example

You can run the breakout example, very similar to the bevy examples:
```sh
cargo build --release --example breakout --target wasm32-unknown-unknown --features js --no-default-features
wasm-bindgen --out-name wasm_example   --out-dir examples/wasm/target   --target web target/wasm32-unknown-unknown/release/examples/breakout.wasm
basic-http-server examples/wasm
```