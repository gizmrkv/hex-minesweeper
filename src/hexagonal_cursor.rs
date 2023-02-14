use crate::cursor2d;
use crate::hexagonal_coordinate::*;
use bevy::prelude::*;

/// Plugin to detect when the cursor enters or exits the hexagon.
pub struct HexagonalCursorPlugin;

impl Plugin for HexagonalCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnEnterHexagon>()
            .add_event::<OnExitHexagon>()
            .add_system(check_enter_or_exit_hexagon)
            .add_system(info_enter_or_exit_hexagon_event);
    }
}

/// Hexagonal cursor marker.
#[derive(Default, Component)]
pub struct HexagonalCursor;

/// Hexagonal cursor marker with origin transform.
#[derive(Default, Bundle)]
pub struct HexagonalCursorBundle {
    pub hexagonal_cursor: HexagonalCursor,
    pub transform: Transform,
}

/// Event that the cursor enters the hexagon.
pub struct OnEnterHexagon {
    pub hexagonal_cursor_entity: Entity,
    pub grid_point: (i32, i32),
}

/// Event that the cursor exits the hexagon.
pub struct OnExitHexagon {
    pub hexagonal_cursor_entity: Entity,
    pub grid_point: (i32, i32),
}

/// Check OnEnterEvent and OnExitHexagon.
fn check_enter_or_exit_hexagon(
    hexagonal_cursor_query: Query<(Entity, &Transform), With<HexagonalCursor>>,
    cursor: Res<cursor2d::Cursor2d>,
    mut enter_event_writer: EventWriter<OnEnterHexagon>,
    mut exit_event_writer: EventWriter<OnExitHexagon>,
    mut previous_cursor: Local<cursor2d::Cursor2d>,
) {
    let curr_pos = cursor.world_position;
    let prev_pos = previous_cursor.world_position;
    for (
        entity,
        Transform {
            translation,
            scale,
            rotation: _,
        },
    ) in hexagonal_cursor_query.iter()
    {
        let prev_grid = snap_cartesian_to_hexagonal_grid(
            ((prev_pos + translation.truncate()) / scale.truncate()).into(),
        );
        let curr_grid = snap_cartesian_to_hexagonal_grid(
            ((curr_pos + translation.truncate()) / scale.truncate()).into(),
        );

        if curr_grid != prev_grid {
            exit_event_writer.send(OnExitHexagon {
                hexagonal_cursor_entity: entity,
                grid_point: prev_grid,
            });
            enter_event_writer.send(OnEnterHexagon {
                hexagonal_cursor_entity: entity,
                grid_point: curr_grid,
            });
        }
    }

    *previous_cursor = *cursor;
}

/// Inform enter and exit event.
fn info_enter_or_exit_hexagon_event(
    mut enter_event_reader: EventReader<OnEnterHexagon>,
    mut exit_event_reader: EventReader<OnExitHexagon>,
) {
    for OnEnterHexagon {
        hexagonal_cursor_entity,
        grid_point,
    } in enter_event_reader.iter()
    {
        info!(
            "Cursor enters the Hexagon {:?} in {:?}",
            grid_point, hexagonal_cursor_entity
        );
    }

    for OnExitHexagon {
        hexagonal_cursor_entity,
        grid_point,
    } in exit_event_reader.iter()
    {
        info!(
            "Cursor exits the Hexagon {:?} in {:?}",
            grid_point, hexagonal_cursor_entity
        );
    }
}
