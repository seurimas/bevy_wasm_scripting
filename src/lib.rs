use assets::{compile_wasm_scripts, WasmAssetLoader, WatAssetLoader};
use bevy::prelude::{AddAsset, App, CoreStage, FromWorld, Plugin, Resource, World};

extern crate anyhow;
extern crate wasmer;
extern crate wat;

mod assets;
#[macro_use]
mod calls;
mod commands;
mod components;
mod entity;
mod resources;
mod world_pointer;

pub use assets::WasmScript;
pub use calls::{
    GeneralWasmScriptEnv, WasmScriptComponentEnv, WasmScriptEnv, WasmScriptResourceEnv,
};
use commands::ScriptCommandQueue;
pub use commands::ScriptSystemWithCommands;
use components::instantiate_wasm_component_scripts;
pub use components::WasmScriptComponent;
pub use entity::*;
use resources::instantiate_wasm_resource_scripts;
pub use resources::{instantiate_resource_script, WasmScriptResource};
#[cfg(feature = "non-js")]
use wasmer::Cranelift;
use wasmer::Store;
pub use world_pointer::WorldPointer;

/** The `WasmerStore` is an essential item for the use of wasm scripts. However, it should not
be referenced directly by systems. `WasmScriptEnv`, `WasmScriptComponentEnv`, and
`WasmScriptResourceEnv` are better entry points for running scripts. */
#[derive(Resource)]
pub struct WasmerStore(pub Store);

impl FromWorld for WasmerStore {
    #[cfg(feature = "non-js")]
    fn from_world(_world: &mut World) -> Self {
        WasmerStore(Store::new(Cranelift::default()))
    }
    #[cfg(feature = "js")]
    fn from_world(_world: &mut World) -> Self {
        WasmerStore(Store::new())
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

pub trait WasmScriptAdder {
    fn add_wasm_script_component<S: WasmScriptComponent>(&mut self) -> &mut Self;
    fn add_wasm_script_resource<R: WasmScriptResource>(&mut self) -> &mut Self;
}

impl WasmScriptAdder for App {
    fn add_wasm_script_component<S: WasmScriptComponent>(&mut self) -> &mut Self {
        self.add_system(instantiate_wasm_component_scripts::<S>)
            .init_resource::<ScriptCommandQueue<S>>()
    }

    fn add_wasm_script_resource<R: WasmScriptResource>(&mut self) -> &mut Self {
        self.add_system(instantiate_wasm_resource_scripts::<R>)
            .init_resource::<ScriptCommandQueue<R>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
