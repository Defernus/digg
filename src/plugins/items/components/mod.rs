use bevy::prelude::*;

pub mod presets;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemComponent;
