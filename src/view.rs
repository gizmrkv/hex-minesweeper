use crate::hexgrid;
use crate::hexgrid::PointyHexGrid;
use crate::{model, Config};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::collections::HashMap;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_view);
    }
}

#[derive(Component, Default)]
struct Tilemap;

#[derive(Resource)]
struct TileIds {
    pub material_mesh_ids: HashMap<PointyHexGrid, Entity>,
    pub text_ids: HashMap<PointyHexGrid, Entity>,
}

#[derive(Component, Default)]
struct TileHexGrid {
    pub grid: PointyHexGrid,
}

#[derive(Bundle, Default)]
struct TilemapBundle {
    tilemap: Tilemap,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

#[derive(Bundle)]
struct TileMaterialMeshBundle {
    grid: TileHexGrid,

    #[bundle]
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle)]
struct TileTextBundle {
    grid: TileHexGrid,

    #[bundle]
    text: Text2dBundle,
}

fn setup_view(
    mut commands: Commands,
    config: Res<Config>,
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
    let tile_edge_mesh =
        shape::RegularPolygon::new(config.tile_size * (2.0 - config.tile_gap_scale), 6);
    let tile_color_material = ColorMaterial::from(config.tile_color);
    let tile_edge_color_material = ColorMaterial::from(config.tile_edge_color);

    let tile_text_font = asset_server.load(&config.tile_text_font_path);
    let tile_text_style = TextStyle {
        font: tile_text_font,
        font_size: config.tile_text_size,
        color: config.tile_text_color,
    };

    let mut tilemap_entity_commands = commands.spawn(TilemapBundle {
        transform: Transform::from_translation(tilemap_translation),
        ..Default::default()
    });

    let mut material_mesh_ids = HashMap::<PointyHexGrid, Entity>::new();
    let mut text_ids = HashMap::<PointyHexGrid, Entity>::new();

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
                        " ".to_string()
                    };

                    parent.spawn(TileMaterialMeshBundle {
                        grid: TileHexGrid { grid },
                        material_mesh: MaterialMesh2dBundle {
                            transform: Transform::from_translation(Vec3::from((
                                tile_position,
                                config.tile_layer,
                            ))),
                            mesh: meshes.add(tile_edge_mesh.into()).into(),
                            material: materials.add(tile_edge_color_material.clone()),
                            ..Default::default()
                        },
                    });
                    let material_mesh_id = parent
                        .spawn(TileMaterialMeshBundle {
                            grid: TileHexGrid { grid },
                            material_mesh: MaterialMesh2dBundle {
                                transform: Transform::from_translation(Vec3::from((
                                    tile_position,
                                    config.tile_layer,
                                ))),
                                mesh: meshes.add(tile_mesh.into()).into(),
                                material: materials.add(tile_color_material.clone()),
                                ..Default::default()
                            },
                        })
                        .id();
                    let text_id = parent
                        .spawn(TileTextBundle {
                            grid: TileHexGrid { grid },
                            text: Text2dBundle {
                                transform: Transform::from_translation(Vec3::from((
                                    tile_position,
                                    config.tile_text_layer,
                                ))),
                                text: Text::from_section(tile_text, tile_text_style.clone())
                                    .with_alignment(TextAlignment::CENTER),
                                ..Default::default()
                            },
                        })
                        .id();
                    material_mesh_ids.insert(grid, material_mesh_id);
                    text_ids.insert(grid, text_id);
                }
            }
        }
    });

    commands.insert_resource(TileIds {
        material_mesh_ids,
        text_ids,
    });
}
