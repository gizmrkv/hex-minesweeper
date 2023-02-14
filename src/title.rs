use crate::AppState;
use bevy::prelude::*;

/// Plugin that controls the title state.
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Title).with_system(startup))
            .add_system_set(SystemSet::on_update(AppState::Title).with_system(enter_menu));
    }
}

/// Spawn title image.
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("image/title.png");
    commands.spawn((
        TitleSprite,
        SpriteBundle {
            texture: image_handle.clone(),
            ..Default::default()
        },
    ));
    commands.spawn(DisplayTitleTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

/// Enter menu.
fn enter_menu(
    time: Res<Time>,
    title_sprite_query: Query<Entity, With<TitleSprite>>,
    mut display_timer_query: Query<(Entity, &mut DisplayTitleTimer)>,
    mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
) {
    let (entity, mut timer) = display_timer_query.single_mut();
    if timer.tick(time.delta()).just_finished() {
        if let Ok(_) = app_state.set(AppState::Menu) {
            commands.entity(entity).despawn();
            commands.entity(title_sprite_query.single()).despawn();
            info!("enter Menu from Title.");
        } else {
            error!("failed to enter Menu from Title.");
        }
    }
}

#[derive(Component)]
struct TitleSprite;

#[derive(Component, Deref, DerefMut)]
struct DisplayTitleTimer(Timer);
