

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ButtonState {
    None, Over, Pressed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Horizon, Vertical,
}

