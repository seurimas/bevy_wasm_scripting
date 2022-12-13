use bevy::{prelude::*, DefaultPlugins};
use bevy_wasmer_scripting::{
    WasmPlugin, WasmScript, WasmScriptComponent, WasmScriptComponentAdder, WasmerStore,
    WorldPointer,
};
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WasmPlugin)
        .add_startup_system(spawn_script_entity)
        .add_system(call_script_on_entity)
        .add_wasm_script_component::<CallerScript>()
        .run();
}

#[derive(Debug, Component)]
struct CallerScript {
    handle: Handle<WasmScript>,
    accumulator: i32,
}

fn get_accumulator(env: FunctionEnvMut<WorldPointer>, entity_id: u64) -> i32 {
    if let Some(caller) = env
        .data()
        .read()
        .get::<CallerScript>(Entity::from_bits(entity_id))
    {
        caller.accumulator
    } else {
        println!("Could not get accumulator!");
        0
    }
}

impl WasmScriptComponent for CallerScript {
    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.handle
    }

    fn get_imports_from_world(
        wasmer_store: &mut WasmerStore,
        world: &WorldPointer,
    ) -> wasmer::Imports {
        let env = FunctionEnv::new(&mut wasmer_store.0, world.clone());
        imports! {
            "env" => {
                "get_accumulator" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_accumulator)
            }
        }
    }
}

fn spawn_script_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CallerScript {
        handle: asset_server.load("add_one_to_accumulator.wat"),
        accumulator: 0,
    });
}

fn call_script_on_entity(
    mut scripted_entities: Query<(Entity, &mut CallerScript)>,
    mut wasmer_store: ResMut<WasmerStore>,
    wasm_scripts: ResMut<Assets<WasmScript>>,
) {
    for (entity, mut scripted_entity) in scripted_entities.iter_mut() {
        if let Some(script) = wasm_scripts.get(&mut scripted_entity.handle) {
            match script.call_if_instantiated(&mut wasmer_store, "main", entity.to_bits()) {
                Ok(new_val) => {
                    scripted_entity.accumulator = new_val;
                    println!("Accumulated value: {}", scripted_entity.accumulator);
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
    }
}
