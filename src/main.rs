use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

use catalyst::Error;

use std::collections::HashMap;
use std::time::{Instant, Duration};

type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Mode {
    NoWindow,
    Window,
}

#[derive(Clone)]
enum Action {
    Nop,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct  KeySequence {
    keys: Vec<u32>,
}

struct KeybindingDaemon {
    key_sequences: HashMap<Mode, HashMap<KeySequence, Action>>,
    chord_timeout: Duration,
    mode: Mode,
    current_sequence: KeySequence,
    timestamp: Option<Instant>,
}

impl KeybindingDaemon {
    fn new() -> Self {
        KeybindingDaemon {
            key_sequences: HashMap::new(),
            chord_timeout: Duration::from_millis(500),
            mode: Mode::NoWindow,
            current_sequence: KeySequence { keys: Vec::new() },
            timestamp: None,
        }
    }

    fn handle_key_press(&mut self, keycode: u32) -> Option<Action> {
        let now = Instant::now();

        self.current_sequence.keys.push(keycode);

        if let Some(timestamp) = self.timestamp {
            if now.duration_since(timestamp) > self.chord_timeout {
                return None;
            }
        }

        let action = match self.mode {
            Mode::NoWindow => self.handle_no_window_mode(self.current_sequence.clone()),
            Mode::Window => self.handle_window_mode(),
        };

        if action.is_some() {
            self.current_sequence.keys.clear();
            action
        } else {
            if let Some(bindings) = self.key_sequences.get(&self.mode) {
                if bindings
                    .keys()
                    .any(|seq| seq.keys.starts_with(&self.current_sequence.keys)) {
                    if !self.timestamp.is_some() {
                        self.timestamp = Some(Instant::now());
                    }
                } else {
                    self.current_sequence.keys.clear();
                }
            }

            None
        }
    }

    fn handle_no_window_mode(&self, sequence: KeySequence) -> Option<Action> {
        if let Some(bindings) = self.key_sequences.get(&self.mode) {
            bindings.get(&sequence).cloned()
        } else {
            None
        }
    }

    fn handle_window_mode(&self) -> Option<Action> {
        todo!()
    }
}

fn main() -> Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    let mut keybinding_store = KeybindingDaemon::new();

    conn.change_window_attributes(root, &ChangeWindowAttributesAux::new().event_mask(EventMask::KEY_PRESS))?;
    conn.flush()?;

    keybinding_store.handle_key_press(0);

    Ok(())
}
