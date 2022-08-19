use sfml::window::{Event, Key};

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Down,
    RotateLeft,
    RotateRight,
    Quit,
    LostFocus,
    GainedFocus,
}

pub struct Actions {
    left: bool,
    right: bool,
    down: bool,
    rotate_left: bool,
    rotate_right: bool,
    quit: bool,
    lost_focus: bool,
    gained_focus: bool,
}
impl Actions {
    pub fn new() -> Actions {
        Actions{
            left: false,
            right: false,
            down: false,
            rotate_left: false,
            rotate_right: false,
            quit: false,
            lost_focus: false,
            gained_focus: false,
        }
    }

    pub fn set(&mut self, a: Action, b: bool) {
        match a {
            Action::Left => self.left = b,
            Action::Right => self.right = b,
            Action::Down => self.down = b,
            Action::RotateLeft => self.rotate_left = b,
            Action::RotateRight => self.rotate_right = b,
            Action::Quit => self.quit = b,
            Action::LostFocus => self.lost_focus = b,
            Action::GainedFocus => self.gained_focus = b,
        }
    }
    pub fn get(&self, a: Action) -> &bool {
        match a {
            Action::Left => &self.left,
            Action::Right => &self.right,
            Action::Down => &self.down,
            Action::RotateLeft => &self.rotate_left,
            Action::RotateRight => &self.rotate_right,
            Action::Quit => &self.quit,
            Action::LostFocus => &self.lost_focus,
            Action::GainedFocus => &self.gained_focus,
        }
    }
}

pub struct Input {
    window: Option<Event>,
    actions: Actions,
}
impl Input {
    pub fn new() -> Input {
        Input {
            window: None,
            actions: Actions::new(),
        }
    }

    pub fn process(&mut self, events: Option<Event>) {
        self.window = events;
        self.match_actions();
    }


    fn match_actions(&mut self) {
        let x = match self.window {
            Some(Event::KeyPressed {code: c, alt: _, ctrl: _, shift: _, system: _}) => Some((c, true)),
            Some(Event::KeyReleased {code: c, alt: _, ctrl: _, shift: _, system: _}) => Some((c, false)),
            Some(Event::Closed) => {self.set_action(Action::Quit, true); None},
            Some(Event::LostFocus) => {self.set_action(Action::LostFocus, true); self.set_action(Action::GainedFocus, false); None},
            Some(Event::GainedFocus) => {self.set_action(Action::GainedFocus, true); self.set_action(Action::LostFocus, false); None},
            _ => None
        };
        match x {
            Some((Key::A, n)) => self.set_action(Action::Left, n),
            Some((Key::S, n)) => self.set_action(Action::Down, n),
            Some((Key::D, n)) => self.set_action(Action::Right, n),
            Some((Key::Q, n)) => self.set_action(Action::RotateLeft, n),
            Some((Key::E, n)) => self.set_action(Action::RotateRight, n),
            Some((Key::Escape, n)) => self.set_action(Action::Quit, n),
            _ => (),
        }
    }

    pub fn set_action(&mut self, a: Action, b: bool) {
        self.actions.set(a, b);
    }

    pub fn get_action(&self, a: Action) -> &bool {
        self.actions.get(a)

    }

    pub fn get_iter(&self) -> Vec<Action> {
        let mut vec = vec![];
        if self.get_action(Action::Left) == &true {vec.push(Action::Left)}
        if self.get_action(Action::Down) == &true {vec.push(Action::Down)}
        if self.get_action(Action::Right) == &true {vec.push(Action::Right)}
        if self.get_action(Action::RotateLeft) == &true {vec.push(Action::RotateLeft)}
        if self.get_action(Action::RotateRight) == &true {vec.push(Action::RotateRight)}
        if self.get_action(Action::Quit) == &true {vec.push(Action::Quit)}
        if self.get_action(Action::LostFocus) == &true {vec.push(Action::LostFocus)}
        if self.get_action(Action::GainedFocus) == &true {vec.push(Action::GainedFocus)}
        return vec
    }
}