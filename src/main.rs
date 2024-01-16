use gilrs::{Gilrs, Button, Event};
fn main()
{

    let mut gilrs = Gilrs::new().unwrap();
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    
    let mut active_gamepad = None;
    let mut stick_lx: f32;
    let mut stick_ly: f32;
    let mut stick_rx: f32;
    let mut stick_ry: f32;
    
    loop {
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            println!("{:?}", event);
            active_gamepad = Some(id);
        }
    
        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }
}