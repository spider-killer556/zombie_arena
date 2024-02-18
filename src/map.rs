use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use itertools::iproduct;

use crate::{assets::Graphics, state::GameState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);

        app.add_systems(OnEnter(GameState::Playing), create_map);
    }
}

#[derive(Component)]
pub struct MapBounds {
    pub x: f32,
    pub y: f32,
}

fn create_map(mut commands: Commands, graphics: Res<Graphics>) {
    let map_size = TilemapSize { x: 64, y: 64 };
    let tilemap_entity = commands
        .spawn(MapBounds {
            x: (map_size.x / 2) as f32,
            y: (map_size.y / 2) as f32,
        })
        .id();

    let mut tile_storage = TileStorage::empty(map_size);

    iproduct!(0..map_size.x, 0..map_size.y).for_each(|(x, y)| {
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    });

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(graphics.map.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
