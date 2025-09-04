pub enum GameEvent<U> {
    AccessKit(accesskit_winit::Event),
    UserEvent(U),
}

pub struct EventHandler;
