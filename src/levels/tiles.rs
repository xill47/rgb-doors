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

#[derive(Component)]
pub enum Door {
    Red,
    Green,
    Blue,
}

#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Door,
}

#[derive(Bundle)]
pub struct TileBundle {
    pub floor: Floor,
    pub wall: Wall,
    pub door: Door,
}

impl LdtkIntCell for DoorBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _: &LayerInstance) -> Self {
        match int_grid_cell.value {
            3 => DoorBundle { door: Door::Red },
            4 => DoorBundle { door: Door::Green },
            5 => DoorBundle { door: Door::Blue },
            value => panic!("Invalid door value: {}", value),
        }
    }
}
