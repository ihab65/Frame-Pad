use bevy::prelude::*;

/// A structural node (like a joint in a frame)
#[derive(Component)]
pub struct Node {
    pub pos: Vec2,
}

/// A structural element (beam/column) connecting 2 nodes
#[derive(Component)]
pub struct Element {
    pub start: Entity,
    pub end: Entity,
}

/// Marker for the background grid
#[derive(Component)]
pub struct Grid;
