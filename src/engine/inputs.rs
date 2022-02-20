/// A key on the keyboard
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Up,
    Down,
    Left,
    Right,
    Escape,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl
}

impl Key {
    #[inline]
    pub(crate) fn from_glfw_key(glfw_key: glfw::Key) -> Option<Self> {
        match glfw_key {
            glfw::Key::A => Some(Key::A),
            glfw::Key::B => Some(Key::B),
            glfw::Key::C => Some(Key::C),
            glfw::Key::D => Some(Key::D),
            glfw::Key::E => Some(Key::E),
            glfw::Key::F => Some(Key::F),
            glfw::Key::G => Some(Key::G),
            glfw::Key::H => Some(Key::H),
            glfw::Key::I => Some(Key::I),
            glfw::Key::J => Some(Key::J),
            glfw::Key::K => Some(Key::K),
            glfw::Key::L => Some(Key::L),
            glfw::Key::M => Some(Key::M),
            glfw::Key::N => Some(Key::N),
            glfw::Key::O => Some(Key::O),
            glfw::Key::P => Some(Key::P),
            glfw::Key::Q => Some(Key::Q),
            glfw::Key::R => Some(Key::R),
            glfw::Key::S => Some(Key::S),
            glfw::Key::T => Some(Key::T),
            glfw::Key::U => Some(Key::U),
            glfw::Key::V => Some(Key::V),
            glfw::Key::W => Some(Key::W),
            glfw::Key::X => Some(Key::X),
            glfw::Key::Y => Some(Key::Y),
            glfw::Key::Z => Some(Key::Z),
            glfw::Key::Up => Some(Key::Up),
            glfw::Key::Down => Some(Key::Down),
            glfw::Key::Left => Some(Key::Left),
            glfw::Key::Right => Some(Key::Right),
            glfw::Key::Escape => Some(Key::Escape),
            glfw::Key::LeftShift => Some(Key::LeftShift),
            glfw::Key::RightShift => Some(Key::RightShift),
            glfw::Key::LeftControl => Some(Key::LeftCtrl),
            glfw::Key::RightControl => Some(Key::RightCtrl),
            _ => None,
        }
    }
}
