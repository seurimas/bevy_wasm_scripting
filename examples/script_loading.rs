use bevy::{
    prelude::{info, App, AssetServer, Assets, Handle, Res, ResMut, Resource},
    DefaultPlugins,
};
use bevy_wasm_scripting::{WasmPlugin, WasmScript};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WasmPlugin)
        .init_resource::<State>()
        .add_startup_system(load_wat_file)
        .add_system(confirm_compiled)
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<WasmScript>,
}

fn load_wat_file(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("add_one.wat");
}

fn confirm_compiled(state: Res<State>, wasm_scripts: ResMut<Assets<WasmScript>>) {
    let wasm_script = wasm_scripts.get(&state.handle);
    if wasm_script.is_none() {
        return;
    }

    info!("Loaded: {:?}", wasm_script.unwrap());
}
