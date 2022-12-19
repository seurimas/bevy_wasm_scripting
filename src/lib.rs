use assets::{compile_wasm_scripts, WasmAssetLoader, WatAssetLoader};
use bevy::prelude::{AddAsset, App, CoreStage, Plugin};

extern crate anyhow;
extern crate wasmer;
extern crate wat;

mod assets;
mod calls;
mod components;
mod resources;
mod world_pointer;

pub use assets::WasmScript;
pub use calls::{GeneralWasmScriptEnv, WasmScriptComponentEnv, WasmScriptEnv};
use components::instantiate_wasm_scripts;
pub use components::WasmScriptComponent;
pub use resources::WasmerStore;
pub use world_pointer::WorldPointer;

#[derive(Default)]
pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<WasmScript>()
            .init_resource::<WasmerStore>()
            .add_asset_loader(WasmAssetLoader)
            .add_asset_loader(WatAssetLoader)
            .add_system_to_stage(CoreStage::Last, compile_wasm_scripts);
    }
}

pub trait WasmScriptComponentAdder {
    fn add_wasm_script_component<S: WasmScriptComponent>(&mut self) -> &mut Self;
}

impl WasmScriptComponentAdder for App {
    fn add_wasm_script_component<S: WasmScriptComponent>(&mut self) -> &mut Self {
        self.add_system(instantiate_wasm_scripts::<S>);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
