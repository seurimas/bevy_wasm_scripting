use std::sync::{Arc, RwLock};

use bevy::prelude::World;

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
}
