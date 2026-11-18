use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct GameState {
    pub hour: u8,
    pub elapsed: f32,
    pub monitor_up: bool,
    pub mask_on: bool,
    pub flashlight_on: bool,
    pub flashlight_charge: f32,
    pub music_box: f32,
    pub current_camera: usize,
    pub animatronic_step: f32,
    pub animatronic_position: usize,
    pub message: String,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            hour: 12,
            elapsed: 0.0,
            monitor_up: false,
            mask_on: false,
            flashlight_on: false,
            flashlight_charge: 100.0,
            music_box: 100.0,
            current_camera: 0,
            animatronic_step: 0.0,
            animatronic_position: 0,
            message: "Ночь 1 — добро пожаловать на смену".into(),
        }
    }
}

#[derive(Component)]
struct HudText;
#[derive(Component)]
struct OfficeVisual;
#[derive(Component)]
struct ControlButton(Action);
#[derive(Clone, Copy)]
enum Action { Monitor, Mask, Flashlight, CameraLeft, CameraRight, Wind }

const CAMERAS: [&str; 5] = ["CAM 01 — СЦЕНА", "CAM 02 — СТОЛОВАЯ", "CAM 03 — ЗАПАДНЫЙ КОРИДОР", "CAM 04 — ПРАВАЯ ВЕНТИЛЯЦИЯ", "CAM 05 — ЛЕВАЯ ВЕНТИЛЯЦИЯ"];

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Office background and a few stylized shapes; placeholder art will be replaced later.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.055, 0.060, 0.075), Vec2::new(960.0, 540.0)),
        Transform::from_xyz(0.0, 0.0, -2.0),
        OfficeVisual,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.12, 0.12, 0.14), Vec2::new(680.0, 120.0)),
        Transform::from_xyz(0.0, -210.0, -1.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.025, 0.025, 0.035), Vec2::new(130.0, 300.0)),
        Transform::from_xyz(-350.0, 15.0, -0.5),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.025, 0.025, 0.035), Vec2::new(130.0, 300.0)),
        Transform::from_xyz(350.0, 15.0, -0.5),
    ));

    commands.spawn((
        Text2d::new("FNAF 2: RUST EDITION"),
        TextFont { font_size: 28.0, ..default() },
        TextColor(Color::srgb(0.85, 0.85, 0.80)),
        Transform::from_xyz(0.0, 225.0, 5.0),
    ));

    // HUD
    commands.spawn((
        Text::new(""),
        TextFont { font_size: 19.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(14.0),
            left: Val::Px(16.0),
            ..default()
        },
        HudText,
    ));

    spawn_button(&mut commands, "МОНИТОР", Action::Monitor, 0.0);
    spawn_button(&mut commands, "МАСКА", Action::Mask, 1.0);
    spawn_button(&mut commands, "ФОНАРИК", Action::Flashlight, 2.0);
    spawn_button(&mut commands, "КАМЕРА ◀", Action::CameraLeft, 3.0);
    spawn_button(&mut commands, "КАМЕРА ▶", Action::CameraRight, 4.0);
    spawn_button(&mut commands, "ЗАВЕСТИ", Action::Wind, 5.0);
}

fn spawn_button(commands: &mut Commands, label: &str, action: Action, index: f32) {
    commands.spawn((
        Button,
        ControlButton(action),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(18.0),
            left: Val::Px(14.0 + index * 158.0),
            width: Val::Px(145.0),
            height: Val::Px(58.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.13, 0.16)),
        BorderColor::all(Color::srgb(0.45, 0.45, 0.48)),
        children![(
            Text::new(label),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        )],
    ));
}

pub fn clock_tick(time: Res<Time>, mut state: ResMut<GameState>) {
    state.elapsed += time.delta_secs();
    // Prototype pacing: one in-game hour every 45 real seconds.
    if state.elapsed >= 45.0 {
        state.elapsed = 0.0;
        state.hour = if state.hour >= 5 { 6 } else { state.hour + 1 };
        if state.hour == 6 {
            state.message = "6 AM — смена завершена!".into();
        }
    }

    if state.flashlight_on {
        state.flashlight_charge = (state.flashlight_charge - time.delta_secs() * 5.0).max(0.0);
        if state.flashlight_charge <= 0.0 {
            state.flashlight_on = false;
            state.message = "Заряд фонарика закончился".into();
        }
    }
    state.music_box = (state.music_box - time.delta_secs() * 0.35).max(0.0);
}

pub fn keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
) {
    if keys.just_pressed(KeyCode::Space) { state.monitor_up = !state.monitor_up; }
    if keys.just_pressed(KeyCode::KeyM) { state.mask_on = !state.mask_on; }
    if keys.just_pressed(KeyCode::KeyF) && state.flashlight_charge > 0.0 {
        state.flashlight_on = !state.flashlight_on;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        state.current_camera = (state.current_camera + CAMERAS.len() - 1) % CAMERAS.len();
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        state.current_camera = (state.current_camera + 1) % CAMERAS.len();
    }
    if keys.just_pressed(KeyCode::KeyW) {
        state.music_box = (state.music_box + 15.0).min(100.0);
        state.message = "Музыкальная шкатулка заведена".into();
    }
}

pub fn touch_controls(
    touches: Res<Touches>,
    mut state: ResMut<GameState>,
) {
    // Basic touch fallback: tap screen sides to switch cameras while monitor is up.
    if state.monitor_up {
        for touch in touches.iter_just_pressed() {
            if touch.position().x < 0.0 {
                state.current_camera = (state.current_camera + CAMERAS.len() - 1) % CAMERAS.len();
            } else {
                state.current_camera = (state.current_camera + 1) % CAMERAS.len();
            }
        }
    }
}

pub fn button_interactions(
    mut interactions: Query<(&Interaction, &ControlButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<GameState>,
) {
    for (interaction, control, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.28, 0.29, 0.33));
                match control.0 {
                    Action::Monitor => state.monitor_up = !state.monitor_up,
                    Action::Mask => state.mask_on = !state.mask_on,
                    Action::Flashlight => {
                        if state.flashlight_charge > 0.0 { state.flashlight_on = !state.flashlight_on; }
                    }
                    Action::CameraLeft => state.current_camera = (state.current_camera + CAMERAS.len() - 1) % CAMERAS.len(),
                    Action::CameraRight => state.current_camera = (state.current_camera + 1) % CAMERAS.len(),
                    Action::Wind => {
                        state.music_box = (state.music_box + 20.0).min(100.0);
                        state.message = "Музыкальная шкатулка заведена".into();
                    }
                }
            }
            Interaction::Hovered => *color = BackgroundColor(Color::srgb(0.20, 0.21, 0.25)),
            Interaction::None => *color = BackgroundColor(Color::srgb(0.12, 0.13, 0.16)),
        }
    }
}

pub fn refresh_hud(
    state: Res<GameState>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    if !state.is_changed() { return; }
    let hour = if state.hour == 12 { 12 } else { state.hour };
    let monitor = if state.monitor_up { format!("МОНИТОР: {}", CAMERAS[state.current_camera]) } else { "МОНИТОР: ОПУЩЕН".into() };
    let mask = if state.mask_on { "НАДЕТА" } else { "СНЯТА" };
    let flashlight = if state.flashlight_on { "ВКЛ" } else { "ВЫКЛ" };
    let mut text = hud.single_mut().expect("HUD text exists");
    **text = format!(
        "{hour} AM   |   {monitor}\nМАСКА: {mask}   |   ФОНАРИК: {flashlight} ({:.0}%)\nШКАТУЛКА: {:.0}%\n{}\n\nУправление: Space — монитор, M — маска, F — фонарик, ←/→ — камеры, W — завести",
        state.flashlight_charge, state.music_box, state.message
    );
}

pub fn animatronic_ai(time: Res<Time>, mut state: ResMut<GameState>) {
    state.animatronic_step += time.delta_secs();
    if state.animatronic_step >= 12.0 {
        state.animatronic_step = 0.0;
        state.animatronic_position = (state.animatronic_position + 1) % 4;
        state.message = match state.animatronic_position {
            0 => "Вдалеке слышен металлический шум...".into(),
            1 => "Кажется, кто-то переместился по коридору...".into(),
            2 => "В вентиляции раздался шорох...".into(),
            _ => "На камерах замечено движение...".into(),
        };
    }
}
