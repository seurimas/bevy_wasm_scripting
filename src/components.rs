use anyhow::anyhow;
use bevy::{
    ecs::{
        event::ManualEventReader,
        query::WorldQuery,
        system::{SystemParam, SystemState},
    },
    prelude::*,
    utils::HashSet,
};
use wasmer::{imports, Imports, Instance, Module};

use crate::{world_pointer::WorldPointer, WasmScript, WasmerStore};

/** The WasmScriptComponent represents the configuration point for component-based scripts.
A WasmScriptComponent should have an associated handle, which is returned by `get_wasm_script_handle`.

Each WasmScriptComponent can define its own set of imports, by defining `get_imports_from_world`.
When defining imports which reference the provided `WorldPointer`, you should include a list of
queried components in `ImportQueriedComponents`, and any referenced resources in `ImportedResources`.

SAFETY: If your system uses the components or resources, care should be taken to avoid safety issues
related to concurrent access to those components.

If you are not defining imports or not using the provided `WorldPointer`, both `ImportResources` and
`ImportQueriedComponents` can be set to `()`.

`WasmScriptComponent` types should be registered with the App, using `add_wasm_script_component`. This
will ensure that the script assets are instantiated and usable. Instantiation happens only when the
asset is loaded or reloaded.
 */
pub trait WasmScriptComponent: Component {
    type ImportQueriedComponents: WorldQuery;
    type ImportResources: SystemParam;

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

fn get_added_script_assets<S: WasmScriptComponent>(world: &mut World) -> Vec<Handle<WasmScript>> {
    let mut s_handles = HashSet::new();
    {
        for changed in world
            .query_filtered::<&S, Or<(Changed<S>, Added<S>)>>()
            .iter(world)
        {
            s_handles.insert(changed.get_wasm_script_handle().clone());
        }
    }
    s_handles.drain().collect::<Vec<Handle<WasmScript>>>()
}

fn instantiate_if_compiled<S: WasmScriptComponent>(
    world: &mut World,
    wasm_script_handle: Handle<WasmScript>,
) -> Option<(String, Instance)> {
    // SAFETY: Probably not safe?
    // Need to figure out how world access actually works and how long we can keep a WorldPointer around...
    unsafe {
        let world_pointer = WorldPointer::new(world).clone();
        let mut state =
            SystemState::<(ResMut<Assets<WasmScript>>, ResMut<WasmerStore>)>::new(world);
        let (mut wasm_assets, mut wasmer_store) = state.get_mut(world);
        let wasm_script = wasm_assets.get_mut(&wasm_script_handle)?;
        let name = wasm_script.name();
        if let WasmScript::Compiled(module) = wasm_script {
            bevy::log::warn!("Received compiled module {}...", name);
            match S::instantiate(&world_pointer, &mut wasmer_store, module) {
                Ok(instance) => {
                    bevy::log::warn!("Instantiated module {}...", name);
                    Some((module.name().unwrap_or("").to_string(), instance))
                }
                Err(err) => {
                    bevy::log::error!("Could not instantiate {}: {}", name, err);
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn instantiate_wasm_component_scripts<S: WasmScriptComponent>(world: &mut World) {
    for script_asset in get_modified_script_assets::<S>(world) {
        if let Some((name, instance)) = instantiate_if_compiled::<S>(world, script_asset.clone()) {
            world
                .get_resource_mut::<Assets<WasmScript>>()
                .unwrap()
                .set_untracked(script_asset, WasmScript::Instantiated(name, instance));
        }
    }
    for script_asset in get_added_script_assets::<S>(world) {
        if let Some((name, instance)) = instantiate_if_compiled::<S>(world, script_asset.clone()) {
            world
                .get_resource_mut::<Assets<WasmScript>>()
                .unwrap()
                .set_untracked(script_asset, WasmScript::Instantiated(name, instance));
        }
    }
}
