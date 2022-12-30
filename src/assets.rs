use std::fmt::Debug;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{AssetEvent, Assets, EventReader, Res, ResMut},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use wasmer::{wat2wasm, Imports, Instance, Module};

use crate::WasmerStore;

/**
A WasmScript assets represented a single, eventually-instantiated WASM script. All WasmScript assets
are automatically compiled by the compile_wasm_scripts system, and may come from .wat or .wasm files.
Ideally, scripts should be loaded early so they can be compiled during a loading stage. Compilation
can take some time.

However, for a WasmScript to become Instantiated and used, you must do one of the following:
* Implement a WasmScriptComponent; register that component with `add_wasm_script_component`; and then
add a component of that type, with the asset handle associated as per `get_wasm_script_handle`.
* For resources with one associated script, implement WasmScriptResource and register that resource
with `add_wasm_script_resource`.
* For other resource-based scripts, add a system to your app, using `instantiate_resource_script`.
* Call `instantiate_if_compiled` on the WasmScript directly. This will not work with hot reloading.

Hot reloading is enabled for component and resource-based scripts, though there will be 1-2 frames in
which the asset is in the `Loaded` or `Compiled` state and will not run. `call_if_instantiated` will
simply skip these scripts.
*/
#[derive(Debug, TypeUuid)]
#[uuid = "a0150d40-bffa-487c-ba73-736dc035120e"]
pub enum WasmScript {
    Loaded(String, Vec<u8>),
    Compiled(Module),
    Instantiated(String, Instance),
}

// Does this break everything? I bet this breaks everything.
// There's a raw pointer in JS Modules that marks them as !Send and !Sync.
unsafe impl Send for WasmScript {}
unsafe impl Sync for WasmScript {}

impl WasmScript {
    pub fn instantiate_if_compiled(
        &mut self,
        wasmer_store: &mut WasmerStore,
        imports: &Imports,
    ) -> bool {
        if let WasmScript::Compiled(module) = self {
            if let Ok(instance) = Instance::new(&mut wasmer_store.0, module, imports) {
                *self = WasmScript::Instantiated(module.name().unwrap_or("").to_string(), instance);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Loaded(name, _) | Self::Instantiated(name, _) => name.clone(),
            Self::Compiled(module) => module.name().unwrap_or("").to_string(),
        }
    }
}

pub struct WasmAssetLoader;

impl AssetLoader for WasmAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let name = get_module_name(load_context);
            load_context
                .set_default_asset(LoadedAsset::new(WasmScript::Loaded(name, bytes.to_vec())));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wasm"]
    }
}

pub struct WatAssetLoader;

impl AssetLoader for WatAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let name = get_module_name(load_context);
            let bytes = wat2wasm(bytes)?;
            load_context
                .set_default_asset(LoadedAsset::new(WasmScript::Loaded(name, bytes.to_vec())));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wat"]
    }
}

fn get_module_name(load_context: &mut LoadContext) -> String {
    let name = load_context.path().file_stem().map_or_else(
        || "<unnamed>".to_string(),
        |stem| stem.to_string_lossy().to_string(),
    );
    name
}

pub(crate) fn compile_wasm_scripts(
    mut ev_asset_loaded: EventReader<AssetEvent<WasmScript>>,
    mut wasm_assets: ResMut<Assets<WasmScript>>,
    wasm_store: Res<WasmerStore>,
) {
    for asset in ev_asset_loaded.iter() {
        if let AssetEvent::Created { handle } | AssetEvent::Modified { handle } = asset {
            if let Some(WasmScript::Loaded(name, wasm_script)) = wasm_assets.get(&handle) {
                match Module::new(&wasm_store.0, wasm_script) {
                    Ok(mut module) => {
                        module.set_name(name);
                        wasm_assets.set_untracked(handle, WasmScript::Compiled(module));
                    }
                    Err(err) => {
                        bevy::log::warn!("Could not compile {}: {}", name, err);
                    }
                }
            }
        }
    }
}
