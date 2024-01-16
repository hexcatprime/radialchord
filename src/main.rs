use gilrs::{Gilrs, Event, EventType};
use libm::atan2f;
fn main()
{

    let mut gilrs = Gilrs::new().unwrap();
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut stick_left: [f32; 2] = [0.0,0.0];
    let mut stick_right: [f32; 2] =[0.0,0.0];
    let mut angle_left: f32 = atan2f(stick_left[1],stick_left[0]);
    let mut angle_right: f32 = atan2f(stick_right[1],stick_right[0]);
    
    loop 
    {
        while let Some(event) = gilrs.next_event() 
        {

            match event 
            {
                Event { event: EventType::AxisChanged(gilrs::Axis::LeftStickX, event_value, _), ..} => 
                {
                    stick_left[0] = event_value;
                }
                Event { event: EventType::AxisChanged(gilrs::Axis::LeftStickY, event_value, _), ..} => 
                {
                    stick_left[1] = event_value;
                }
                Event { event: EventType::AxisChanged(gilrs::Axis::RightStickX, event_value, _), ..} => 
                {
                    stick_right[0] = event_value;
                }
                Event { event: EventType::AxisChanged(gilrs::Axis::RightStickY, event_value, _), ..} => 
                {
                    stick_right[1] = event_value;
                }
                _ => (),
            };

            angle_left = atan2f(stick_left[1],stick_left[0]);
            angle_right = atan2f(stick_right[1],stick_right[0]);
            
            println!("left angle:{} - right angle:{}", angle_left, angle_right);
            // println!("left:{},{} - right:{},{}", stick_left[0], stick_left[1], stick_right[0], stick_right[1]);
        }
    }
}