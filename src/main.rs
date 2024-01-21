use gilrs::{Gilrs, Gamepad};
// use enigo::*;
use libm::atan2f;
// use serde::de::value;

const ZONE_ANGLE: f32 = 45.0;
struct Joystick
{
    axis_x: f32,
    axis_y: f32,
    angle: f32,
    zone: i32,
}

impl Joystick
{
    pub fn new() -> Self
    {
        let axis_x: f32 = 0.0;
        let axis_y: f32 = 0.0;
        let angle: f32 = 0.0;
        let zone: i32 = 0;
        Joystick {axis_x, axis_y, angle, zone}
    }
    pub fn set(&mut self, axis_x_unclamped: f32, axis_y_unclamped: f32)
    {
        self.axis_x = axis_x_unclamped.clamp(-1.0,1.0);
        self.axis_y = axis_y_unclamped.clamp(-1.0,1.0);
        self.angle = atan2f(self.axis_y, self.axis_y);
        self.zone = zone_check(self.angle);
    }
    pub fn print(&self)
    {
        println!("Axes: ({},{})\tAngle: ({})\tZone: ({})", self.axis_x,self.axis_y,self.angle, self.zone);
    }
}

fn main()
{
    let mut gilrs = Gilrs::new().unwrap();
    // let mut enigo = Enigo::new();
    let mut stick_left: Joystick = Joystick::new();
    let mut stick_right: Joystick = Joystick::new();
    let mut active_gamepad: Gamepad;
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    
    loop 
    {
        while let Some(ev) = gilrs.next_event()
        {
            active_gamepad = gilrs.gamepad(ev.id);
            stick_left.set(active_gamepad.value(gilrs::Axis::LeftStickX), active_gamepad.value(gilrs::Axis::LeftStickY));
            stick_right.set(active_gamepad.value(gilrs::Axis::RightStickX), active_gamepad.value(gilrs::Axis::RightStickY));
        }

        stick_left.print();
        stick_right.print();
    }
}

fn zone_check(x: f32) -> i32
{
(x / ZONE_ANGLE) as i32
}