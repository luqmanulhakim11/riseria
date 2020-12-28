use bevy::prelude::*;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityType {
    Human,
}

#[derive(Copy, Clone)]
pub struct Type(pub EntityType);

impl Type {
    pub fn get_type(self) -> EntityType {
        self.0
    }
}

// TODO Make components for humans

/// This struct holds all the materials in a centralized struct
/// All materials shall be declared here
/// To ensure that all resources are only added once
pub struct GameColorMaterials {
    pub human_material: Handle<ColorMaterial>,
}
