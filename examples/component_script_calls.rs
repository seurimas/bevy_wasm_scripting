use bevy::{prelude::*, DefaultPlugins};
use bevy_wasm_scripting::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WasmPlugin)
        // Component-based scripts
        .add_wasm_script_component::<AdderScript>()
        .add_startup_system(spawn_script_entity)
        .add_system(call_script_on_entity)
        .run();
}

#[derive(Component)]
struct AdderScript {
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
    commands.spawn(AdderScript {
        handle: asset_server.load("multiply_two.wat"),
        accumulator: 1,
    });
}

fn call_script_on_entity(
    mut scripted_entities: Query<&mut AdderScript>,
    mut script_env: WasmScriptComponentEnv<AdderScript>,
) {
    for mut scripted_entity in scripted_entities.iter_mut() {
        if let Ok(new_val) = script_env.call_if_instantiated_1(
            &scripted_entity.handle,
            "main",
            scripted_entity.accumulator,
        ) {
            scripted_entity.accumulator = new_val;
        }
        println!("Accumulated value: {}", scripted_entity.accumulator);
    }
}
