use bevy::prelude::Entity;

pub type EntityId = f64;

pub trait EntityIdTrait {
    fn to_entity(&self) -> Entity;
}

impl EntityIdTrait for EntityId {
    fn to_entity(&self) -> Entity {
        Entity::from_bits(self.to_bits())
    }
}
