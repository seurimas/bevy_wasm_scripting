# bevy_wasm_scripting
Adds support for wasm/wat assets in Bevy, and enables easy scripting.

- [x] Scripts managed through bevy asset management
- [x] Scripts attached to components
- [x] Scripts attached to resources
- [x] Hot-reloading of component and resource-based scripts
- [x] [Basic examples](examples)
- [ ] Confirm safety of `WorldPointer` imports strategy.
- [ ] Investigate compilation performance and multi-threading options.
- [ ] Investigate memory usage.
- [ ] Investigate cooperation with web builds.
- [ ] Configuration for Wasmer Tunables and Compiler.
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

#[derive(Resource)]
struct AdderResourceScript {
    handle: Handle<WasmScript>,
    accumulator: i32,
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
