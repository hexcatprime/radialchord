use serde_json::Value;
use enigo::Key;
use gilrs::{Button, Gamepad, Gilrs};
use std::{collections::HashMap, f32::consts::PI};

const ZONE_ANGLE: f32 = 45.0;
const ZONE_OFFSET: f32 = 45.0;

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
        self.angle = (self.axis_y.atan2(self.axis_x)*(180.0/PI) + 360.0) % 360.0;
        self.zone = ((self.angle + ZONE_OFFSET) / ZONE_ANGLE) as i32;
        self.active = 0.75 <= (self.axis_x.powi(2) + self.axis_y.powi(2)) && (self.axis_x.powi(2) + self.axis_y.powi(2)) <= 1.0;
    }
    pub fn _print(&self)
    {
        print!("\rAxes: ({:+05.3},{:+05.3})\tAngle: ({:+08.3})\tZone: ({:2})", self.axis_x,self.axis_y,self.angle, self.zone);
    }
}

fn main()
{
    let mut gilrs = Gilrs::new().unwrap();
    // let mut _enigo = Enigo::new();
    let mut stick_chord: Joystick = Joystick::new();
    let mut stick_note: Joystick = Joystick::new();
    let mut active_gamepad: Gamepad;

    let button_map: Value  = load_config("/home/dubsbol/Downloads/radialchord/src/buttonmap.json");
    println!("{:?}", button_map);

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

fn load_config(x: &str) -> Value
{
    let path = std::path::Path::new(x);
    let file: String = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let v: Value = serde_json::from_str(&file).unwrap()
    let hm: HashMap<Button, Key> = HashMap::new();
    for 

}