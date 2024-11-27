use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use tokio::time::{sleep, Duration};

use catalyst::Error;

use std::collections::HashMap;
use std::time::Instant;

type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Mode {
    NoWindow,
    Window,
}

#[derive(Clone, Debug)]
enum Action {
    Nop,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

    fn add_keybinding(&mut self, mode: Mode, key_sequence: KeySequence, action: Action) {
        self.key_sequences
            .entry(mode)
            .or_insert_with(HashMap::new)
            .insert(key_sequence, action);
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
            Mode::Window => self.handle_window_mode(self.current_sequence.clone()),
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

    fn handle_window_mode(&self, sequence: KeySequence) -> Option<Action> {
        if let Some(bindings) = self.key_sequences.get(&self.mode) {
            bindings.get(&sequence).cloned()
        } else {
            None
        }
    }
}

struct Desktop {
    conn: x11rb::rust_connection::RustConnection,
    num: usize,
    root: Window,
}

impl Desktop {
    fn new() -> Result<Self> {
        let (conn, num) = x11rb::connect(None)?;
        let screen = &conn.setup().roots[num];
        let root = screen.root;

        Ok(Desktop {
            conn,
            num,
            root,
        })
    }

    fn check_for_open_windows(&self) -> Result<bool> {
        let cookie = self.conn.query_tree(self.root)?;
        let reply = cookie.reply()?;

        Ok(!reply.children.is_empty())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let desktop = Desktop::new()?;
    let mut keybinding_store = KeybindingDaemon::new();

    if desktop.check_for_open_windows()? {
        keybinding_store.mode = Mode::Window;
    } else {
        keybinding_store.mode = Mode::NoWindow;
    }

    desktop.conn.change_window_attributes(desktop.root, &ChangeWindowAttributesAux::new().event_mask(EventMask::KEY_PRESS))?;
    desktop.conn.flush()?;

    keybinding_store.add_keybinding(Mode::Window, KeySequence { keys: vec![ 20, 73 ] }, Action::Nop );

    println!("{:?}", keybinding_store.handle_key_press(20));
    println!("{:?}", keybinding_store.handle_key_press(73));

    Ok(())
}
