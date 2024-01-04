use std::f32::consts::PI;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use key_bindings::KeyBindings;

/// Структура плагина камеры.
/// Данный плагин отвечает за настройку и перемещение камеры игрока
pub struct CameraPlugin;

/// Маркер для дефолтной камеры игрока
#[derive(Component)]
pub struct PlayerCamera;

/// Хранит состояние для движения мыши
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Настройки перемещения камеры
#[derive(Resource)]
pub struct MovementSettings {
    /// Чувствительность мыши
    pub sensitivity: f32,
    /// Скорость перемещения камеры
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.0002,
            speed: 8.,
        }
    }
}


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_systems(Startup, setup_player_camera)
            .add_systems(Update, (player_move_system, player_look_system, cursor_grab_system));
    }
}

/// Создает и настраивает дефолтную камеру игрока
fn setup_player_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3., 3., 3.)
                .looking_at(vec3(1., 1., 1.), Vec3::Z),
            ..default()
        },
        PlayerCamera,
    ));
}

/// Слушает события нажатия клавиш отвечающих за перемещение камеры
fn player_move_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let window = primary_window.single();
    let mut transform = camera.single_mut();

    let mut velocity = Vec3::ZERO;
    let local_x = transform.local_x();
    let forward = -Vec3::new(local_x.y, -local_x.x, 0.);
    let right = Vec3::new(local_x.x, local_x.y, 0.);

    for key in keys.get_pressed() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => (),
            _ => {
                let key = *key;
                if key == key_bindings.move_forward {
                    velocity += forward;
                } else if key == key_bindings.move_backward {
                    velocity -= forward;
                } else if key == key_bindings.move_left {
                    velocity -= right;
                } else if key == key_bindings.move_right {
                    velocity += right;
                } else if key == key_bindings.move_up {
                    velocity += Vec3::Z;
                } else if key == key_bindings.move_down {
                    velocity -= Vec3::Z;
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_seconds() * settings.speed
    }
}

/// Слушает события движения мыши и поворачивает камеру если курсор захвачен
fn player_look_system(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let window = primary_window.single();
    let mut transform = camera.single_mut();

    // Проверяем захвачен ли курсор
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            // Если курсор не захвачен игнорируем движение мыши
            state.reader_motion.clear(&motion)
        }
        CursorGrabMode::Confined |
        CursorGrabMode::Locked => {
            // Если курсор захвачен поворачиваем камеру
            for ev in state.reader_motion.read(&motion) {
                // Вверх направлена ось Z. Это может быть не типично, чаще вверх направляют ось Y,
                // но такая система позволяет снизить количество путаници в координатах чанков.
                let (mut yaw, _, mut pitch) = transform.rotation.to_euler(EulerRot::ZYX);

                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();

                // Что бы избежать эффекта "Шарнирного замка" мы не можем смотреть полностью вверх
                // или вниз, поэтому ограничиваем максимальный угол близким к критическому.
                pitch = pitch.clamp(0.02 * PI, 0.98 * PI);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Z, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

/// Переключает состояние курсора при нажатии клавиши toggle_grab_cursor
fn cursor_grab_system(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.single_mut();
    if keys.just_pressed(key_bindings.toggle_grab_cursor) {
        toggle_grab_cursor(&mut window);
    }
}

/// Захватывает / отпускает курсор мыши
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}