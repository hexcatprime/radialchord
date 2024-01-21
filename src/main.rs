use gilrs::{Gilrs, Gamepad};
use serde::{Serialize, Deserialize};
use toml::from_str;
use enigo::*;
use std::collections::HashMap;
use libm::atan2f;
use std::f64::consts::PI;

const ZONE_ANGLE: f32 = 45.0;
const ZONE_OFFSET: f32 = 45.0;

#[derive(Serialize, Deserialize)]
struct Chord
{
    notes: Vec<enigo::keycodes::Key>
}
struct Joystick
{
    axis_x: f32,
    axis_y: f32,
    angle: f32,
    zone: i32,
    active: bool,
}

impl Joystick
{
    pub fn new() -> Self
    {
        let axis_x: f32 = 0.0;
        let axis_y: f32 = 0.0;
        let angle: f32 = 0.0;
        let zone: i32 = 0;
        let active: bool = false;
        Joystick {axis_x, axis_y, angle, zone, active}
    }
    pub fn set(&mut self, axis_x_unclamped: f32, axis_y_unclamped: f32)
    {
        self.axis_x = axis_x_unclamped.clamp(-1.0,1.0);
        self.axis_y = axis_y_unclamped.clamp(-1.0,1.0);
        self.angle = (atan2f(self.axis_y, self.axis_x)*(180.0/PI as f32) + 360.0) % 360.0;
        self.zone = ((self.angle + ZONE_OFFSET) / ZONE_ANGLE) as i32;
        self.active = (false);
    }
    pub fn _print(&self)
    {
        print!("\rAxes: ({:+05.3},{:+05.3})\tAngle: ({:+08.3})\tZone: ({:2})", self.axis_x,self.axis_y,self.angle, self.zone);
    }
}

fn main()
{
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let mut stick_chord: Joystick = Joystick::new();
    let mut stick_note: Joystick = Joystick::new();
    let mut active_gamepad: Gamepad;
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}\n", gamepad.name(), gamepad.power_info());
    }
    
    loop 
    {
        while let Some(ev) = gilrs.next_event()
        {
            active_gamepad = gilrs.gamepad(ev.id);
            stick_chord.set(active_gamepad.value(gilrs::Axis::LeftStickX), active_gamepad.value(gilrs::Axis::LeftStickY));
            stick_note.set(active_gamepad.value(gilrs::Axis::RightStickX), active_gamepad.value(gilrs::Axis::RightStickY));
        }
    }
}

fn load_keybinds() -> Table
{
    let path = std::path::Path::new("./keybinds.toml");
    let file: &str = match std::fs::read_to_string(path) {
        Ok(f) => &f,
        Err(e) => panic!("{}", e),
    };
    let chord_map: HashMap<String, Chord> = from_str(file).unwrap();
    
}