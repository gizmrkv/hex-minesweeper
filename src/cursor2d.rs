use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

/// Plugin to get cursor position.
pub struct Cursor2dPlugin;

impl Plugin for Cursor2dPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor2d::default())
            .add_system(update_cursor);
    }
}

/// Cursor position.
#[derive(Resource, Default)]
pub struct Cursor2d {
    pub screen_position: Vec2,
    pub world_position: Vec2,
}

/// Update screen and world coordinates of the cursor.
fn update_cursor(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor2d: ResMut<Cursor2d>,
) {
    let (camera, camera_transform) = camera_query.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_position) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor2d.screen_position = screen_position;
        cursor2d.world_position = world_position.truncate();
    }
}
