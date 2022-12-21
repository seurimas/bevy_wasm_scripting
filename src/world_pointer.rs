use std::{
    any::type_name,
    sync::{Arc, RwLock},
};

use bevy::prelude::{Commands, World};

use crate::commands::ScriptCommandQueue;

#[derive(Debug, Clone)]
pub struct WorldPointer(pub Arc<RwLock<*mut World>>);

unsafe impl Send for WorldPointer {}
unsafe impl Sync for WorldPointer {}

impl WorldPointer {
    /**
     * SAFETY: This should only be instantiated from the main world, which shouldn't(?) be dropped or moved.
     */
    pub unsafe fn new(world: &mut World) -> Self {
        WorldPointer(Arc::new(RwLock::new(world)))
    }

    pub fn read(&self) -> &World {
        unsafe { &**self.0.read().expect("FIX ME") }
    }

    pub fn write(&self) -> &mut World {
        unsafe { &mut **self.0.write().expect("FIX ME") }
    }

    pub fn commands<ScriptType: 'static + Send + Sync>(&self) -> Commands {
        unsafe {
            // SAFETY: I don't know. I'm still winging it at this point.
            let world = self.write();
            let command_queue = world
                .get_resource_unchecked_mut::<ScriptCommandQueue<ScriptType>>()
                .expect(
                    format!(
                        "No ScriptCommandQueue<{}> found.",
                        type_name::<ScriptType>()
                    )
                    .as_str(),
                );
            Commands::new(&mut command_queue.into_inner().0, world)
        }
    }
}
