use enigo::{keycodes::Key::*, *};
use gilrs::{Gamepad, Gilrs, Event, Button, EventType};
use std::{collections::HashMap, f32::consts::PI};

const MOUSE_MULTIPLIER: f32 = 10.0;
struct Joystick
{
    zone_angle: f32,
    zone_offset: f32,
    zone_deadzone: f32,
    axis_x: f32,
    axis_y: f32,
    angle: f32,
    zone: i32,
    active: bool,
}

impl Joystick
{
    pub fn new( zone_angle: f32, zone_offset: f32, zone_deadzone: f32) -> Self
    {
        let axis_x: f32 = 0.0;
        let axis_y: f32 = 0.0;
        let angle: f32 = 0.0;
        let zone: i32 = 0;
        let active: bool = false;
        Joystick {zone_angle, zone_offset, zone_deadzone, axis_x, axis_y, angle, zone, active}
    }
    pub fn set(&mut self, axis_x_unclamped: f32, axis_y_unclamped: f32)
    {
        self.axis_x = axis_x_unclamped.clamp(-1.0,1.0);
        self.axis_y = axis_y_unclamped.clamp(-1.0,1.0);
        self.angle = (self.axis_y.atan2(self.axis_x)*(180.0/PI) + 360.0) % 360.0;
        self.zone = ((self.angle) / self.zone_angle) as i32;
        self.active = self.zone_deadzone <= (self.axis_x.powi(2) + self.axis_y.powi(2));
    }
    pub fn print(&self)
    {
        print!("\rAxes: ({:+05.3},{:+05.3})\tAngle: ({:+08.3})\tZone: ({:2}, \tActive?: ({})", self.axis_x as i32,self.axis_y as i32,self.angle, self.zone, self.active);
    }
}

fn main()
{
    let key_map = build_key_map();
    let chord_map = build_chord_map();
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let mut stick_chord: Joystick = Joystick::new(45.0, 45.0 , 0.25);
    let mut stick_note: Joystick = Joystick::new(45.0, 45.0 , 0.25);
    let mut active_gamepad: Gamepad;

    let mut cached_key: Key = Layout('0');

    let mut zone_lock: bool = false;
    let mut context_lock: bool = false;

    loop 
    {
        while let Some(Event {id, event,  ..}) = gilrs.next_event()
        {
            match event 
            {

                EventType::ButtonReleased(button, _) => 
                {
                    match key_map.get(&button)
                    {
                        Some(&F33) => enigo.mouse_click(MouseButton::Left),
                        Some(&F34) => enigo.mouse_click(MouseButton::Right),
                        Some(&F35) => context_lock = !context_lock,
                        Some(&value) => enigo.key_click(Key::try_from(value).unwrap()),
                        _ => (),
                    }
                    // println!("{:?}, {:?}", button, context_lock);
                }

                EventType::AxisChanged(..) =>
                {
                active_gamepad = gilrs.gamepad(id);
                stick_chord.set(active_gamepad.value(gilrs::Axis::LeftStickX), active_gamepad.value(gilrs::Axis::LeftStickY));
                stick_note.set(active_gamepad.value(gilrs::Axis::RightStickX), active_gamepad.value(gilrs::Axis::RightStickY));
                stick_chord.print();
                if context_lock
                {
                    enigo.mouse_move_relative((stick_chord.axis_x * MOUSE_MULTIPLIER) as i32, -(stick_chord.axis_y * MOUSE_MULTIPLIER) as i32);
                }
                else if stick_note.active
                {
                    cached_key = chord_map[stick_chord.zone as usize][stick_note.zone as usize];
                    zone_lock = true;
                } 
                else if zone_lock
                {
                    enigo.key_click(cached_key);
                    zone_lock = false;
                }
                }
                _ => (),
            }
        }
    }
}

fn build_key_map() -> HashMap<Button, Key>
{
    let mut temp: HashMap<Button, Key> = HashMap::new();
    temp.insert(Button::South, Return);
    temp.insert(Button::East, Space);
    temp.insert(Button::North, PageDown);
    temp.insert(Button::West, PageUp);
    temp.insert(Button::LeftTrigger, Control);
    temp.insert(Button::LeftTrigger2, F34);
    temp.insert(Button::RightTrigger, Alt);
    temp.insert(Button::RightTrigger2, F33);
    temp.insert(Button::Select, Tab);
    temp.insert(Button::Start, Escape);
    temp.insert(Button::DPadUp, UpArrow);
    temp.insert(Button::DPadDown, DownArrow);
    temp.insert(Button::DPadLeft, LeftArrow);
    temp.insert(Button::DPadRight, RightArrow);
    temp.insert(Button::LeftThumb, F35);
    temp.insert(Button::RightThumb, Shift);
    
    temp
}

fn build_chord_map() -> Vec<Vec<Key>>
{
    let temp: Vec<Vec<Key>> = vec!
    [
        vec![Layout('o'), Layout('s'), Layout('e'), Layout('n'), Layout('t'), Layout('i'), Layout('a'), Layout('h')],
        vec![Layout('j'), Layout('q'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o')],
        vec![Layout('u'), Layout('y'), Layout('r'), Layout('c'), Layout('l'), Layout('m'), Layout('d'), Layout('w')],
        vec![Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o')],
        vec![Layout('b'), Layout('x'), Layout('f'), Layout('v'), Layout('g'), Layout('k'), Layout('p'), Layout('z')],
        vec![Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o')],
        vec![Space, Space, Space, Space, Space, Space, Space, Space],
        vec![Layout('0'), Layout('1'), Layout('2'), Layout('3'), Layout('4'), Layout('5'), Layout('6'), Layout('7'), Layout('8'), Layout('9') ]
    ];
    temp
}

