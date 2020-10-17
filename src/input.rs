use bevy::{
    prelude::*,
    input::mouse::{MouseButtonInput, MouseMotion},
};

#[derive(Default)]
pub struct MouseState {
    pub mouse_button_event_reader: EventReader<MouseButtonInput>,
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub cursor_moved_event_reader: EventReader<CursorMoved>,

    pub last_mouse_left_pressed_position: Vec2,
    pub mouse_position: Vec2
}


/// This system prints out all mouse events as they come in
pub fn print_mouse_events_system(
    mut state: ResMut<MouseState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.mouse_position = event.position;
    }
}
