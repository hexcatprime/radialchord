pub mod buttonmap
{
    use enigo::{Key, MouseButton};
    use gilrs::Button;
    struct buttonmap
    {
        South: Key,
        East: Key,
        North: Key,
        West: Key,
        C: Key,
        Z: Key,
        LeftTrigger : MouseButton,
        LeftTrigger2: Key,
        RightTrigger: MouseButton,
        RightTrigger2: Key,
        Select: Key,
        Start: Key,
        Mode: Key,
        LeftThumb: Key,
        RightThumb: Key,
        DPadUp: Key,
        DPadDown: Key,
        DPadLeft: Key,
        DPadRight: Key,
        Unknown: Key,
        LeftZ: Key,
        RightZ: Key,
    }
}