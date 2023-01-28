use crate::hexgrid;
use crate::hexgrid::PointyHexGrid;
use crate::model;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use std::collections::HashMap;

pub struct ViewPlugin;

#[derive(Resource)]
pub struct ViewConfig {
    pub tile_size: f32,
    pub tile_gap_scale: f32,
    pub tile_layer: f32,
    pub tile_color: Color,
    pub tile_text_font_path: String,
    pub tile_text_size: f32,
    pub tile_text_color: Color,
}

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_view);
    }
}

#[derive(Component)]
struct Tilemap;

#[derive(Resource)]
struct TileIds {
    pub ids: HashMap<PointyHexGrid, Entity>,
}

#[derive(Component)]
struct TileHexGrid {
    pub grid: PointyHexGrid,
}

fn setup_view(
    mut commands: Commands,
    config: Res<ViewConfig>,
    game_board: Res<model::GameBoard>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tiles_per_side = game_board.tiles_per_side();
    let board_center_grid = hexgrid::pointy_hex_grid_to_cartesian(PointyHexGrid {
        x: (tiles_per_side - 1) as i32,
        y: (tiles_per_side - 1) as i32,
    });
    let tilemap_translation =
        -Vec3::from((board_center_grid, config.tile_layer)) * config.tile_size;

    let tile_mesh = shape::RegularPolygon::new(config.tile_size * config.tile_gap_scale, 6);
    let tile_color_material = ColorMaterial::from(config.tile_color);

    let tile_text_font = asset_server.load(&config.tile_text_font_path);
    let tile_text_style = TextStyle {
        font: tile_text_font,
        font_size: config.tile_text_size,
        color: config.tile_text_color,
    };

    let mut ids = HashMap::<PointyHexGrid, Entity>::new();
    let mut tilemap_entity_commands = commands.spawn((
        Tilemap,
        Transform::from_translation(tilemap_translation),
        GlobalTransform::default(),
        Visibility::VISIBLE,
        ComputedVisibility::default(),
    ));
    let tilemap_entity = tilemap_entity_commands.id();

    tilemap_entity_commands.add_children(|parent| {
        for x in 0..(2 * tiles_per_side - 1) {
            for y in 0..(2 * tiles_per_side - 1) {
                let grid = PointyHexGrid {
                    x: x as i32,
                    y: y as i32,
                };
                if let Some(tile_state) = game_board.get(grid) {
                    let tile_position =
                        hexgrid::pointy_hex_grid_to_cartesian(grid) * config.tile_size;

                    let tile_text = if tile_state.is_open() && !tile_state.is_mine() {
                        format!("{}", game_board.count_adjacent_mines(grid).unwrap())
                    } else if tile_state.is_open() && tile_state.is_mine() {
                        "M".to_string()
                    } else if tile_state.is_flag() {
                        "F".to_string()
                    } else {
                        "".to_string()
                    };

                    let id = parent
                        .spawn((
                            TileHexGrid { grid },
                            Transform::from_translation(Vec3::from((tile_position, 0.0))),
                            Mesh2dHandle::from(meshes.add(tile_mesh.into())),
                            materials.add(tile_color_material.clone()),
                            Text::from_section(tile_text, tile_text_style.clone())
                                .with_alignment(TextAlignment::CENTER),
                            Visibility::VISIBLE,
                            GlobalTransform::default(),
                            ComputedVisibility::default(),
                        ))
                        .id();
                    ids.insert(grid, id);
                }
            }
        }
    });

    commands.insert_resource(TileIds { ids });
}
