use bevy::{prelude::*, DefaultPlugins};
use bevy_wasm_scripting::{
    WasmPlugin, WasmScript, WasmScriptComponent, WasmScriptComponentAdder, WasmerStore,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(WasmPlugin)
        .add_startup_system(spawn_script_entity)
        .add_system(call_script_on_entity)
        .add_wasm_script_component::<AdderScript>()
        .run();
}

#[derive(Component)]
struct AdderScript {
    handle: Handle<WasmScript>,
    accumulator: i32,
}

impl WasmScriptComponent for AdderScript {
    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.handle
    }
}

fn spawn_script_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AdderScript {
        handle: asset_server.load("add_n.wat"),
        accumulator: 0,
    });
}

fn call_script_on_entity(
    mut scripted_entities: Query<&mut AdderScript>,
    mut wasmer_store: ResMut<WasmerStore>,
    wasm_scripts: ResMut<Assets<WasmScript>>,
) {
    for mut scripted_entity in scripted_entities.iter_mut() {
        if let Some(script) = wasm_scripts.get(&mut scripted_entity.handle) {
            if let Ok(new_val) =
                script.call_if_instantiated(&mut wasmer_store, "main", scripted_entity.accumulator)
            {
                scripted_entity.accumulator = new_val;
            }
        }
        println!("Accumulated value: {}", scripted_entity.accumulator);
    }
}
