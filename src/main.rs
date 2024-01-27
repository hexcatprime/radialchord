use enigo::{Key::*, *};
use gilrs::{ Gamepad, Gilrs};
use std::f32::consts::PI;

// enum Combo
// {
//     Key(Key),
//     MouseButton(MouseButton),
// }
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
        print!("\rAxes: ({:+05.3},{:+05.3})\tAngle: ({:+08.3})\tZone: ({:2}, \tActive?: ({})", self.axis_x,self.axis_y,self.angle, self.zone, self.active);
    }
}

fn main()
{
    let chord_map = build_chord_map();
    // let key_map = build_key_map();
    let mut gilrs = Gilrs::new().unwrap();
    // let mut _enigo = Enigo::new();
    let mut stick_chord: Joystick = Joystick::new(45.0, 45.0 , 0.25);
    let mut stick_note: Joystick = Joystick::new(45.0, 45.0 , 0.25);
    let mut active_gamepad: Gamepad;
    let mut cached_key: Key = Layout('0');
    loop 
    {
        while let Some(ev) = gilrs.next_event()
        {
            active_gamepad = gilrs.gamepad(ev.id);
            stick_chord.set(active_gamepad.value(gilrs::Axis::LeftStickX), active_gamepad.value(gilrs::Axis::LeftStickY));
            stick_note.set(active_gamepad.value(gilrs::Axis::RightStickX), active_gamepad.value(gilrs::Axis::RightStickY));
            // stick_chord.print();
            if stick_note.active
            {
                cached_key = chord_map[stick_chord.zone as usize][stick_note.zone as usize];
                println!("zone:{} zone:{} letter prepared: {:?}", stick_chord.zone, stick_note.zone, cached_key);
            }
            else
            {
                println!("{:?}", cached_key);
            }
        }
    }
}

// fn build_key_map() -> HashMap<Button, Combo>
// {
//     let temp: HashMap<Button, Combo> = HashMap::new();
//     temp.insert(Button::South, Enter);
//     temp.insert(Button::East, Space);
//     temp.insert(Button::North, PageDown);
//     temp.insert(Button::West, PageUp);
//     temp.insert(Button::LeftTrigger, Right);
//     temp.insert(Button::LeftTrigger2, Control);
//     temp.insert(Button::RightTrigger, Left);
//     temp.insert(Button::RightTrigger2, Alt);
//     temp.insert(Button::Select, Tab);
//     temp.insert(Button::Start, Escape);
//     temp.insert(Button::DPadUp, UpArrow);
//     temp.insert(Button::DPadDown, DownArrow);
//     temp.insert(Button::DPadLeft, LeftArrow);
//     temp.insert(Button::DPadRight, RightArrow);
    
//     temp
// }

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
        vec![Layout('0'), Layout('1'), Layout('2'), Layout('3'), Layout('4'), Layout('5'), Layout('6'), Layout('7'), Layout('8'), Layout('9') ],
        vec![Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o'), Layout('o')]
    ];
    temp
}