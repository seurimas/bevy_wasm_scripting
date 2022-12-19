use bevy::prelude::*;
use wasmer::{Cranelift, Store};

#[derive(Resource)]
pub struct WasmerStore(pub Store);

impl FromWorld for WasmerStore {
    fn from_world(_world: &mut World) -> Self {
        WasmerStore(Store::new(Cranelift::default()))
    }
}
