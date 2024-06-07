use tui_realm_stdlib::Phantom as GlobalListener;
use tuirealm::{event::{Key, KeyEvent}, Component, Event, NoUserEvent};

use crate::model::Msg;

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            // Global hotkeys
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::Quit),
            _ => {}
        };

        None
    }
}
