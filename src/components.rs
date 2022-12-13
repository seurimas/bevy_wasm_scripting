use bevy::{ecs::event::ManualEventReader, prelude::*, utils::HashSet};
use wasmer::{imports, Imports, Instance, Module};

use crate::{resources::WasmerStore, world_pointer::WorldPointer, WasmScript};

pub trait WasmScriptComponent: Component {
    fn get_imports_from_world(_wasmer_store: &mut WasmerStore, _world: &WorldPointer) -> Imports {
        // No imports, nothing to do.
        imports! {}
    }

    fn get_wasm_script_handle(&self) -> &Handle<WasmScript>;

    fn instantiate(
        world_pointer: &WorldPointer,
        wasmer_store: &mut WasmerStore,
        module: &Module,
    ) -> Result<Instance, anyhow::Error> {
        let imports = Self::get_imports_from_world(wasmer_store, world_pointer);
        println!("{:?}", imports);
        let instance = Instance::new(&mut wasmer_store.0, module, &imports)?;
        Ok(instance)
    }
}

fn get_modified_script_assets<S: WasmScriptComponent>(
    world: &mut World,
) -> Vec<Handle<WasmScript>> {
    let mut result = Vec::new();
    {
        let ev_asset_loaded = world
            .get_resource_mut::<Events<AssetEvent<WasmScript>>>()
            .unwrap();
        let mut event_reader = ManualEventReader::<AssetEvent<WasmScript>>::default();
        for asset in event_reader.iter(&ev_asset_loaded) {
            if let AssetEvent::Modified { handle } = asset {
                result.push(handle.clone());
            }
        }
    }
    // Don't try to filter if there's nothing to filter.
    if result.len() > 0 {
        let mut scripts_on_entities: QueryState<&S> = world.query();
        let mut s_handles = HashSet::new();
        {
            for component in scripts_on_entities.iter(&world) {
                s_handles.insert(component.get_wasm_script_handle());
            }
        }
        result.retain(|updated_script_handle| s_handles.contains(updated_script_handle));
    }
    result
}

fn instantiate_if_compiled<S: WasmScriptComponent>(
    world: &mut World,
    wasm_script_handle: Handle<WasmScript>,
) -> Option<(String, Instance)> {
    // SAFETY: Probably not safe?
    // Need to figure out how world access actually works and how long we can keep a WorldPointer around...
    unsafe {
        let world_pointer = WorldPointer::new(world).clone();
        let wasm_assets = world.get_resource_unchecked_mut::<Assets<WasmScript>>()?;
        let wasm_script = wasm_assets.get(&wasm_script_handle)?;
        let mut wasmer_store = world.get_resource_unchecked_mut::<WasmerStore>()?;
        if let WasmScript::Compiled(module) = wasm_script {
            match S::instantiate(&world_pointer, &mut wasmer_store, module) {
                Ok(instance) => Some((module.name().unwrap_or("").to_string(), instance)),
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn instantiate_wasm_scripts<S: WasmScriptComponent>(world: &mut World) {
    for script_asset in get_modified_script_assets::<S>(world) {
        if let Some((name, instance)) = instantiate_if_compiled::<S>(world, script_asset.clone()) {
            world
                .get_resource_mut::<Assets<WasmScript>>()
                .unwrap()
                .set_untracked(script_asset, WasmScript::Instantiated(name, instance));
        }
    }
}
