use bevy::{prelude::*, DefaultPlugins};
use bevy_wasm_scripting::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WasmPlugin)
        .add_wasm_script_resource::<AdderResourceScript>()
        .add_startup_system(add_script_resource)
        .add_system(call_script_on_resource)
        .run();
}

#[derive(Resource)]
struct AdderResourceScript {
    handle: Handle<WasmScript>,
    accumulator: i32,
}

impl WasmScriptResource for AdderResourceScript {
    type ImportQueriedComponents = ();
    // We always want this resource. It is up to the user whether it should be mutable or not,
    // and whether additional resources are needed for the imports.
    type ImportResources = ResMut<'static, AdderResourceScript>;

    fn get_handle(&self) -> Option<&Handle<WasmScript>> {
        Some(&self.handle)
    }
}

fn add_script_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AdderResourceScript {
        // We're using a separate asset here, to demonstrate how to instantiate a script manually.
        handle: asset_server.load("add_one_for_resource.wat"),
        accumulator: 0,
    });
}

fn call_script_on_resource(mut script_env: WasmScriptResourceEnv<AdderResourceScript>) {
    if let Ok(new_val) = script_env.call_if_instantiated_1(
        &script_env.resources.handle.clone(),
        "main",
        script_env.resources.accumulator,
    ) {
        script_env.resources.accumulator = new_val;
    }
    println!(
        "Accumulated resource value: {}",
        script_env.resources.accumulator
    );
}
