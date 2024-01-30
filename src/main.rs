use enigo::{keycodes::Key::*, *};
use gilrs::{ev::filter::Repeat, Button, EventType, Gamepad, Gilrs};
use std::{char::MAX, collections::HashMap, f32::consts::PI};

const MOUSE_MULTIPLIER: f32 = 5.0;
struct Joystick {
    zone_angle: f32,
    zone_offset: f32,
    deadzone: f32,
    axis_x: f32,
    axis_y: f32,
    angle: f32,
    zone: i32,
    zone_cached: i32,
    zone_virtual_layer: i32,
    active: bool,
}

impl Joystick {
    pub fn new(zone_angle: f32, zone_offset: f32, deadzone: f32) -> Self {
        let axis_x: f32 = 0.0;
        let axis_y: f32 = 0.0;
        let angle: f32 = 0.0;
        let zone: i32 = 0;
        let zone_cached: i32 = 0;
        let zone_virtual_layer: i32 = 0;
        let active: bool = false;
        Joystick {
            zone_angle,
            zone_offset,
            deadzone,
            axis_x,
            axis_y,
            angle,
            zone,
            zone_cached,
            zone_virtual_layer,
            active,
        }
    }
    pub fn set(&mut self, axis_x_unclamped: f32, axis_y_unclamped: f32) {
        self.axis_x = axis_x_unclamped.clamp(-1.0, 1.0);
        self.axis_y = axis_y_unclamped.clamp(-1.0, 1.0);
        self.angle = (self.axis_y.atan2(self.axis_x) * (180.0 / PI) + 360.0) % 360.0;
        if self.zone != ((self.angle) / self.zone_angle) as i32 {
            self.zone_cached = self.zone;
        }
        self.zone = ((self.angle) / self.zone_angle) as i32;
        self.active = self.deadzone <= (self.axis_x.powi(2) + self.axis_y.powi(2));
    }
    pub fn print(&self) {
        print!(
            "\rAxes: ({:+05.3},{:+05.3})\tAngle: ({:+08.3})\tZone: ({:2})\tCached Zone: ({:2})\tActive?: ({})",
            self.axis_x, self.axis_y, self.angle, self.zone,self.zone_cached, self.active
        );
    }
    pub fn virtual_selection(&mut self, input_vector_length: usize) -> usize
    {
        let last_zone: i32 = (360/self.zone_angle as i32)-1;
        let max_layer: i32 = (input_vector_length as i32- 1)/last_zone;
        if self.zone == 0 && self.zone_cached == last_zone && self.zone_virtual_layer != max_layer
        {
            self.zone_virtual_layer += 1;
        } else if self.zone == last_zone && self.zone_cached == 0 && self.zone_virtual_layer != 0
        {
            self.zone_virtual_layer -= 1;
        }
        (self.zone + (self.zone_virtual_layer*last_zone)) as usize
    }
}          
fn main() {
    let key_map = build_key_map();
    let chord_map = build_chord_map();
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let mut stick_chord: Joystick = Joystick::new(45.0, 45.0, 0.25);
    let mut stick_note: Joystick = Joystick::new(45.0, 45.0, 0.25);

    let mut cached_key: Key = Layout('0');

    let mut zone_lock: bool = false;
    let mut context_lock: bool = false;

    loop {
        while let Some(ev) = gilrs.next_event() {
            match ev.event {
                EventType::ButtonPressed(button, _) => match key_map.get(&button) {
                    Some(&F33) => enigo.mouse_down(MouseButton::Left),
                    Some(&F34) => enigo.mouse_down(MouseButton::Right),
                    Some(&value) => enigo.key_down(Key::try_from(value).unwrap()),
                    _ => (),
                },
                EventType::ButtonReleased(button, _) => match key_map.get(&button) {
                    Some(&F33) => enigo.mouse_up(MouseButton::Left),
                    Some(&F34) => enigo.mouse_up(MouseButton::Right),
                    Some(&F35) => context_lock = !context_lock,
                    Some(&value) => enigo.key_up(Key::try_from(value).unwrap()),
                    _ => (),
                },
                EventType::AxisChanged(..) => {
                    stick_chord.set(
                        gilrs.gamepad(ev.id).value(gilrs::Axis::LeftStickX),
                        gilrs.gamepad(ev.id).value(gilrs::Axis::LeftStickY),
                    );
                    stick_note.set(
                        gilrs.gamepad(ev.id).value(gilrs::Axis::RightStickX),
                        gilrs.gamepad(ev.id).value(gilrs::Axis::RightStickY),
                    );
                    if context_lock {
                        enigo.mouse_move_relative(
                            (stick_chord.axis_x * MOUSE_MULTIPLIER) as i32,
                            -(stick_chord.axis_y * MOUSE_MULTIPLIER) as i32,
                        );
                        enigo.mouse_scroll_x((-stick_note.axis_x * MOUSE_MULTIPLIER) as i32);
                        enigo.mouse_scroll_y((-stick_note.axis_y * MOUSE_MULTIPLIER) as i32);
                    } else if stick_note.active {
                        cached_key = chord_map[stick_chord.virtual_selection(chord_map.len())][stick_note.virtual_selection(chord_map[stick_chord.virtual_selection(chord_map.len())].len())];
                        zone_lock = true;
                    } else if zone_lock {
                        enigo.key_click(cached_key);
                        zone_lock = false;
                    }
                }
                _ => (),
            }
        }
        stick_chord.print();
    }
}

fn build_key_map() -> HashMap<Button, Key> {
    let mut builder: HashMap<Button, Key> = HashMap::new();
    builder.insert(Button::South, Return);
    builder.insert(Button::East, Space);
    builder.insert(Button::North, PageDown);
    builder.insert(Button::West, PageUp);
    builder.insert(Button::LeftTrigger, Control);
    builder.insert(Button::LeftTrigger2, F34);
    builder.insert(Button::RightTrigger, Alt);
    builder.insert(Button::RightTrigger2, F33);
    builder.insert(Button::Select, Tab);
    builder.insert(Button::Start, Escape);
    builder.insert(Button::DPadUp, UpArrow);
    builder.insert(Button::DPadDown, DownArrow);
    builder.insert(Button::DPadLeft, LeftArrow);
    builder.insert(Button::DPadRight, RightArrow);
    builder.insert(Button::LeftThumb, F35);
    builder.insert(Button::RightThumb, Shift);

    builder
}

fn build_chord_map() -> Vec<Vec<Key>> {
    let builder: Vec<Vec<Key>> = vec![
        vec![
            Layout('0'),
            Layout('1'),
            Layout('2'),
            Layout('3'),
            Layout('4'),
            Layout('5'),
            Layout('6'),
            Layout('7'),
            Layout('8'),
            Layout('9'),
        ],
        vec![Layout('o'), Layout('a'), Layout('t'), Layout('e')],
        vec![Layout('i'), Layout('n'), Layout('s'), Layout('h')],
        vec![Layout('r'), Layout('d'), Layout('l'), Layout('v')],
        vec![Layout('c'), Layout('w'), Layout('m'), Layout('f')],
        vec![Layout('y'), Layout('g'), Layout('b'), Layout('p')],
        vec![Layout('v'), Layout('k'), Layout('x'), Layout('j')],
        vec![Layout('q'), Layout('z'), Layout(','), Layout('.')],
        vec![
            Layout('['),
            Layout(']'),
            Layout('\\'),
            Layout('/'),
            Layout(';'),
            Layout('\''),
            Layout('`'),
        ],
    ];
    builder
}
