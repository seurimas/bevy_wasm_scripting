use bevy::prelude::Entity;

pub type EntityId = f64;

pub trait EntityIdTrait {
    fn to_entity(&self) -> Entity;
    fn from_entity(entity: Entity) -> Self;
}

impl EntityIdTrait for EntityId {
    fn to_entity(&self) -> Entity {
        Entity::from_bits(self.to_bits())
    }

    fn from_entity(entity: Entity) -> Self {
        Self::from_bits(entity.to_bits())
    }
}
