use assets::{compile_wasm_scripts, WasmAssetLoader, WatAssetLoader};
use bevy::prelude::{AddAsset, App, CoreStage, FromWorld, Plugin, Resource, World};

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
pub use resources::instantiate_resource_script;
use wasmer::{Cranelift, Store};
pub use world_pointer::WorldPointer;

/** The `WasmerStore` is an essential item for the use of wasm scripts. However, it should not
be referenced directly by systems. `WasmScriptEnv` and `WasmScriptComponentEnv` are better entry
points for running scripts. */
#[derive(Resource)]
pub struct WasmerStore(pub Store);

impl FromWorld for WasmerStore {
    fn from_world(_world: &mut World) -> Self {
        WasmerStore(Store::new(Cranelift::default()))
    }
}

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
