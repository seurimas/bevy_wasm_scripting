use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

use bevy::{
    ecs::{archetype::*, component::ComponentId, system::*, world::*},
    prelude::*,
};

#[derive(Resource)]
pub struct ScriptCommandQueue<ScriptType: 'static + Sync + Send>(
    pub(crate) CommandQueue,
    PhantomData<ScriptType>,
);

impl<ScriptType: 'static + Sync + Send> Default for ScriptCommandQueue<ScriptType> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

pub struct ScriptSystemWithCommands<F, ScriptType>
where
    F: System,
    ScriptType: 'static + Send + Sync,
{
    base_system: F,
    marker: PhantomData<ScriptType>,
}

impl<F: System, ScriptType: 'static + Send + Sync> ScriptSystemWithCommands<F, ScriptType> {
    pub fn wrap(f: F) -> Self {
        Self {
            base_system: f,
            marker: PhantomData,
        }
    }
}

impl<F: System, ScriptType: 'static + Send + Sync> System
    for ScriptSystemWithCommands<F, ScriptType>
{
    type In = <F as System>::In;
    type Out = <F as System>::Out;

    fn name(&self) -> std::borrow::Cow<'static, str> {
        self.base_system.name()
    }

    fn component_access(&self) -> &bevy::ecs::query::Access<ComponentId> {
        self.base_system.component_access()
    }

    fn archetype_component_access(&self) -> &bevy::ecs::query::Access<ArchetypeComponentId> {
        self.base_system.archetype_component_access()
    }

    fn is_send(&self) -> bool {
        self.base_system.is_send()
    }

    fn is_exclusive(&self) -> bool {
        self.base_system.is_exclusive()
    }

    unsafe fn run_unsafe(&mut self, input: Self::In, world: &World) -> Self::Out {
        self.base_system.run_unsafe(input, world)
    }

    fn apply_buffers(&mut self, world: &mut World) {
        self.base_system.apply_buffers(world);
        world.resource_scope::<ScriptCommandQueue<ScriptType>, ()>(|world, mut command_queue| {
            command_queue.0.apply(world);
        });
    }

    fn initialize(&mut self, world: &mut World) {
        self.base_system.initialize(world)
    }

    fn update_archetype_component_access(&mut self, world: &World) {
        // XXX: Need to get access setup!
        self.base_system.update_archetype_component_access(world)
    }

    fn check_change_tick(&mut self, change_tick: u32) {
        self.base_system.check_change_tick(change_tick)
    }

    fn get_last_change_tick(&self) -> u32 {
        self.base_system.get_last_change_tick()
    }

    fn set_last_change_tick(&mut self, last_change_tick: u32) {
        self.base_system.set_last_change_tick(last_change_tick)
    }
}
