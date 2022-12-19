use anyhow::anyhow;
use bevy::{ecs::event::ManualEventReader, prelude::*};
use wasmer::{imports, Imports, Instance, Module};

use crate::{WasmScript, WasmerStore, WorldPointer};

fn instantiate_with_imports(
    wasmer_store: &mut WasmerStore,
    module: &Module,
    imports: &Imports,
) -> Result<Instance, anyhow::Error> {
    let instance = Instance::new(&mut wasmer_store.0, module, imports)?;
    Ok(instance)
}

fn instantiate_if_compiled(
    world: &mut World,
    wasm_script_handle: Handle<WasmScript>,
    get_imports: &impl Fn(&mut WasmerStore, &mut WorldPointer) -> Option<Imports>,
) -> Result<bool, anyhow::Error> {
    // SAFETY: Probably not safe?
    // Need to figure out how world access actually works and how long we can keep a WorldPointer around...
    unsafe {
        let mut world_pointer = WorldPointer::new(world).clone();
        let mut wasm_assets = world
            .get_resource_unchecked_mut::<Assets<WasmScript>>()
            .ok_or(anyhow!("Assets<WasmScript> missing"))?;
        let wasm_script = wasm_assets
            .get_mut(&wasm_script_handle)
            .ok_or(anyhow!("Asset not properly loaded?"))?;
        let mut wasmer_store = world
            .get_resource_unchecked_mut::<WasmerStore>()
            .ok_or(anyhow!("WasmerStore missing"))?;
        if let WasmScript::Compiled(module) = wasm_script {
            let imports =
                &get_imports(&mut wasmer_store, &mut world_pointer).unwrap_or(imports! {});
            match instantiate_with_imports(&mut wasmer_store, module, imports) {
                Ok(instance) => {
                    let name = module.name().unwrap_or("").to_string();
                    *wasm_script = WasmScript::Instantiated(name, instance);
                    Ok(true)
                }
                Err(err) => {
                    // TODO: Error reporting!
                    Err(anyhow!(err))
                }
            }
        } else {
            Ok(false)
        }
    }
}

fn is_script_asset_modified(world: &mut World, resource_handle: &Handle<WasmScript>) -> bool {
    let ev_asset_loaded = world
        .get_resource_mut::<Events<AssetEvent<WasmScript>>>()
        .unwrap();
    let mut event_reader = ManualEventReader::<AssetEvent<WasmScript>>::default();
    for asset in event_reader.iter(&ev_asset_loaded) {
        if let AssetEvent::Modified { handle } = asset {
            if handle == resource_handle {
                return true;
            }
        }
    }
    false
}

/**
Add this system to automatically instantiate a script attached to a resource instead of a component.
This will work well with hot reloading, as asset modifications are listened for.
The two parameters are:
* First, to retrieve the handle for the asset from the resource. If it has not been loaded yet, it may return `None`.
* Next, to return imports for the script to be instantiated with.

Instantiation happens only when the asset is loaded or reloaded.
 */
pub fn instantiate_resource_script<R: Resource>(
    get_handle: impl Fn(&R) -> Option<Handle<WasmScript>>,
    get_imports: impl Fn(&mut WasmerStore, &mut WorldPointer) -> Option<Imports>,
) -> impl Fn(&mut World) {
    move |mut world| {
        if let Some(resource_handle) = world
            .get_resource::<R>()
            .and_then(|resource| get_handle(resource))
        {
            if is_script_asset_modified(&mut world, &resource_handle) {
                // TODO: Error reporting.
                instantiate_if_compiled(&mut world, resource_handle, &get_imports);
            }
        }
    }
}
