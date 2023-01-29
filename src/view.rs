use crate::events::*;
use crate::hexgrid;
use crate::hexgrid::PointyHexGrid;
use crate::{model, Config, CursorWorldPosition};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::collections::HashMap;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedTile::default())
            .add_startup_system(setup_view)
            .add_startup_system(setup_game_over)
            .add_system(recolor_tile_selected_system)
            .add_system(on_move_tile_system)
            .add_system(on_game_over)
            .add_system(on_retry);
    }
}

#[derive(Component, Default)]
pub struct Tilemap;

#[derive(Component, Default)]
struct TileHexGrid {
    pub grid: PointyHexGrid,
}

#[derive(Component)]
struct TileEdge;

#[derive(Resource)]
struct TileIds {
    pub material_mesh_ids: HashMap<PointyHexGrid, Entity>,
    pub text_ids: HashMap<PointyHexGrid, Entity>,
}

#[derive(Resource, Default)]
struct SelectedTile {
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
    let tilemap_translation = -Vec3::from((board_center_grid, 0.0)) * config.tile_size;

    let tile_mesh = shape::RegularPolygon::new(config.tile_size * config.tile_gap_scale, 6);
    let tile_edge_mesh =
        shape::RegularPolygon::new(config.tile_size * (2.0 - config.tile_gap_scale), 6);
    let tile_color_material = ColorMaterial::from(config.tile_color);
    let tile_edge_color_material = ColorMaterial::from(config.tile_edge_color);

    let tile_text_font = asset_server.load(&config.tile_text_font_path);

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
                if let Some(_) = game_board.get(grid) {
                    let tile_position =
                        hexgrid::pointy_hex_grid_to_cartesian(grid) * config.tile_size;
                    parent.spawn((
                        TileEdge,
                        TileHexGrid { grid },
                        MaterialMesh2dBundle {
                            transform: Transform::from_translation(Vec3::from((
                                tile_position,
                                config.tile_edge_layer,
                            ))),
                            mesh: meshes.add(tile_edge_mesh.into()).into(),
                            material: materials.add(tile_edge_color_material.clone()),
                            ..Default::default()
                        },
                    ));
                    let material_mesh_id = parent
                        .spawn((
                            TileHexGrid { grid },
                            MaterialMesh2dBundle {
                                transform: Transform::from_translation(Vec3::from((
                                    tile_position,
                                    config.tile_layer,
                                ))),
                                mesh: meshes.add(tile_mesh.into()).into(),
                                material: materials.add(tile_color_material.clone()),
                                ..Default::default()
                            },
                        ))
                        .id();

                    let (value, color) = get_tile_text_and_color(&game_board, grid, &config);
                    let tile_text_style = TextStyle {
                        font: tile_text_font.clone(),
                        font_size: config.tile_text_size,
                        color,
                    };
                    let text_id = parent
                        .spawn((
                            TileHexGrid { grid },
                            Text2dBundle {
                                transform: Transform::from_translation(Vec3::from((
                                    tile_position,
                                    config.tile_text_layer,
                                ))),
                                text: Text::from_section(value, tile_text_style)
                                    .with_alignment(TextAlignment::CENTER),
                                ..Default::default()
                            },
                        ))
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

fn recolor_tile_selected_system(
    tile_color_query: Query<(&TileHexGrid, &mut Handle<ColorMaterial>), Without<TileEdge>>,
    tilemap_query: Query<&Transform, With<Tilemap>>,
    cursor_world_position: Res<CursorWorldPosition>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
    tile_ids: Res<TileIds>,
    mut selexted_tile: ResMut<SelectedTile>,
) {
    let tilemap_transform = tilemap_query.single();
    let cursor_grid = hexgrid::cartesian_point_to_nearest_pointy_hex_grid(
        (cursor_world_position.position - tilemap_transform.translation.truncate())
            / config.tile_size,
    );

    // 色を戻す
    if let Some(grid_entity) = tile_ids.material_mesh_ids.get(&selexted_tile.grid) {
        if let Ok((_, color_handle)) = tile_color_query.get(*grid_entity) {
            if let Some(color_material) = materials.get_mut(&color_handle) {
                color_material.color = config.tile_color;
            }
        }
    }

    selexted_tile.grid = cursor_grid;

    //色を付ける
    if let Some(grid_entity) = tile_ids.material_mesh_ids.get(&selexted_tile.grid) {
        if let Ok((_, color_handle)) = tile_color_query.get(*grid_entity) {
            if let Some(color_material) = materials.get_mut(&color_handle) {
                color_material.color = config.tile_selected_color;
            }
        }
    }
}

fn on_move_tile_system(
    mut reader: EventReader<OnMoveTile>,
    mut tile_text_query: Query<&mut Text>,
    tile_ids: Res<TileIds>,
    game_board: Res<model::GameBoard>,
    config: Res<Config>,
) {
    for event in reader.iter() {
        let target = match event {
            OnMoveTile::Open { target } => target,
            OnMoveTile::Flag { target } => target,
        };
        if let Some(tile_text_entity) = tile_ids.text_ids.get(target) {
            if let Ok(mut tile_text) = tile_text_query.get_mut(*tile_text_entity) {
                let (value, color) = get_tile_text_and_color(&game_board, *target, &config);
                tile_text.sections[0].value = value;
                tile_text.sections[0].style.color = color;
            }
        }
    }
}

fn get_tile_text_and_color(
    game_board: &model::GameBoard,
    grid: PointyHexGrid,
    config: &Config,
) -> (String, Color) {
    if let Some(tile_state) = game_board.get(grid) {
        if tile_state.is_open() {
            if tile_state.is_mine() {
                ("M".to_string(), config.tile_text_mine_color)
            } else {
                (
                    format!("{}", game_board.count_adjacent_mines(grid).unwrap()),
                    config.tile_text_hint_color,
                )
            }
        } else {
            if tile_state.is_flag() {
                ("F".to_string(), config.tile_text_flag_color)
            } else {
                (" ".to_string(), Color::PINK)
            }
        }
    } else {
        ("ERROR".to_string(), Color::PINK)
    }
}

#[derive(Component)]
struct GameOverParent;

fn setup_game_over(mut commands: Commands, config: Res<Config>, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOverParent,
            Transform::from_translation(Vec3::ZERO),
            GlobalTransform::default(),
            Visibility::INVISIBLE,
            ComputedVisibility::INVISIBLE,
        ))
        .add_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: config.game_over_background_color,
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3 {
                    x: 10000.0,
                    y: 10000.0,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: config.game_over_background_layer,
                }),
                ..Default::default()
            });
            let game_over_text_font = asset_server.load(&config.game_over_text_font_path);
            let game_over_text_style = TextStyle {
                font: game_over_text_font.clone(),
                font_size: config.game_over_text_size,
                color: config.game_over_text_color,
            };
            parent.spawn(Text2dBundle {
                text: Text::from_section(config.game_over_text.clone(), game_over_text_style)
                    .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_translation(Vec3::from((
                    config.game_over_text_position,
                    config.game_over_text_layer,
                ))),
                ..Default::default()
            });
            let game_over_text_below_style = TextStyle {
                font: game_over_text_font.clone(),
                font_size: config.game_over_text_below_size,
                color: config.game_over_text_below_color,
            };
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    config.game_over_text_below.clone(),
                    game_over_text_below_style,
                )
                .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_translation(Vec3::from((
                    config.game_over_text_below_position,
                    config.game_over_text_layer,
                ))),
                ..Default::default()
            });
        });
}

fn on_game_over(
    mut commands: Commands,
    mut reader: EventReader<OnGameOver>,
    mut game_over_query: Query<&mut Visibility, With<GameOverParent>>,
) {
    for event in reader.iter() {
        let mut game_over = game_over_query.single_mut();
        game_over.is_visible = true;
    }
}

fn on_retry(
    mut reader: EventReader<OnRetry>,
    mut game_over_query: Query<&mut Visibility, With<GameOverParent>>,
) {
    for event in reader.iter() {
        let mut game_over = game_over_query.single_mut();
        game_over.is_visible = false;
    }
}
