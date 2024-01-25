use enigo::{Key, MouseButton};
use gilrs::{Button, Gamepad, Gilrs};
use std::{collections::HashMap, f32::consts::PI};

enum Combo
{
    Key(Key),
    MouseButton(MouseButton),
}
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
        self.zone = ((self.angle + self.zone_offset) / self.zone_angle) as i32;
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
    let mut stick_chord: Joystick = Joystick::new(45.0, 45.0 , 50.0);
    let mut stick_note: Joystick = Joystick::new(45.0, 45.0 , 50.0);
    let mut active_gamepad: Gamepad;
    
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
