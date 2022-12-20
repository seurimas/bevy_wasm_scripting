use bevy::{prelude::*, DefaultPlugins};
use bevy_wasm_scripting::*;
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

#[derive(Debug, Resource)]
struct IncrementStep(i32);

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

fn get_n(env: FunctionEnvMut<WorldPointer>) -> i32 {
    if let Some(increment_step) = env.data().read().get_resource::<IncrementStep>() {
        increment_step.0
    } else {
        1
    }
}

impl<'w> WasmScriptComponent for CallerScript {
    /* We need to give the script hints about what sort of components and resources the script's
    imported functions will use. Otherwise, we could run into safety issues. Heck, this still might
    be unsafe. Anyone reading this understand Bevy ECS better than me? */
    type ImportQueriedComponents = &'static CallerScript;
    type ImportResources = Res<'static, IncrementStep>;

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
                "get_accumulator" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_accumulator),
                "get_n" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_n),
            }
        }
    }
}

fn spawn_script_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(IncrementStep(2));
    commands.spawn(CallerScript {
        handle: asset_server.load("add_n_to_accumulator.wat"),
        accumulator: 0,
    });
}

fn call_script_on_entity(
    mut scripted_entities: Query<(Entity, &mut CallerScript)>,
    // Since we use CallerScript in get_accumulator and in this system, add a Without<CallerScript>.
    // Otherwise, bevy_ecs will consider this a safety issue. XXX: Is it?
    mut script_env: WasmScriptComponentEnv<CallerScript, Without<CallerScript>>,
) {
    for (entity, mut scripted_entity) in scripted_entities.iter_mut() {
        // Here, we're providing the entity id to the function, which is then used in an imported function.
        // Any function name can be used, as long as it is properly exported from wasm.
        match script_env.call_if_instantiated(&scripted_entity.handle, "main", entity.to_bits()) {
            Ok(new_val) => {
                scripted_entity.accumulator = new_val;
                println!(
                    "Accumulated value: {} (increment of {})",
                    scripted_entity.accumulator,
                    // We can access the resources ourselves easily enough.
                    script_env.resources.0
                );
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
