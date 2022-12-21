use bevy::{prelude::*, DefaultPlugins};
use bevy_wasm_scripting::*;
use wasmer::imports;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WasmPlugin)
        // Direct management of resource-based scripts.
        // `add_wasm_script_resource` should be used when only one script is on a resource.
        .add_system(instantiate_resource_script::<AdderResourceScript>(
            |resource: &AdderResourceScript| Some(resource.handle.clone()),
            |_wasmer_store, _world_pointer| imports! {},
        ))
        .add_startup_system(add_script_resource)
        .add_system(call_script_on_resource)
        .run();
}

#[derive(Resource)]
struct AdderResourceScript {
    handle: Handle<WasmScript>,
    accumulator: i32,
}

fn add_script_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AdderResourceScript {
        // We're using a separate asset here, to demonstrate how to instantiate a script manually.
        handle: asset_server.load("add_one_for_resource.wat"),
        accumulator: 0,
    });
}

fn call_script_on_resource(
    mut script_resource: ResMut<AdderResourceScript>,
    mut script_env: WasmScriptEnv,
) {
    if let Ok(new_val) = script_env.call_if_instantiated_1(
        &script_resource.handle,
        "main",
        script_resource.accumulator,
    ) {
        script_resource.accumulator = new_val;
    }
    println!(
        "Accumulated resource value: {}",
        script_resource.accumulator
    );
}
