use winit::{
    error::EventLoopError,
    event_loop::{EventLoop, EventLoopBuilder},
};

#[derive(Default)]
pub struct Game<T> {
    pub game_state: T,
}
