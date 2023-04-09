use std::fmt::Display;

use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::{
    prelude::{LayerInstance, LdtkIntCell},
    IntGridCell,
};

#[derive(Component, Default)]
pub struct Floor;

#[derive(Bundle, LdtkIntCell)]
pub struct FloorBundle {
    pub floor: Floor,
}

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy, Default)]
pub struct Laser {
    pub laser_type: LaserType,
    pub is_open: bool,
}

impl Laser {
    pub fn new(laser_type: LaserType) -> Self {
        Self {
            laser_type,
            is_open: false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LaserType {
    #[default]
    Red,
    Green,
    Blue,
}

impl Display for LaserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LaserType::Red => write!(f, "Red"),
            LaserType::Green => write!(f, "Green"),
            LaserType::Blue => write!(f, "Blue"),
        }
    }
}

#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Laser,
}

#[derive(Bundle)]
pub struct TileBundle {
    pub floor: Floor,
    pub wall: Wall,
    pub door: Laser,
}

impl LdtkIntCell for DoorBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _: &LayerInstance) -> Self {
        let laser_type = match int_grid_cell.value {
            3 => LaserType::Red,
            4 => LaserType::Green,
            5 => LaserType::Blue,
            value => panic!("Invalid door value: {}", value),
        };
        Self {
            door: Laser::new(laser_type),
        }
    }
}
