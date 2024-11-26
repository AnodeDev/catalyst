use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

use catalyst::Error;

use std::collections::{VecDeque, HashMap};
use std::time::{Instant, Duration};

type Result<T> = std::result::Result<T, Error>;

enum Mode {
    NoWindow,
    Window,
}

enum Action {
    Nop,
}

type KeySequence = Vec<(Mode, Vec<u32>)>;

struct KeybindingDaemon {
    key_sequence: HashMap<Mode, HashMap<(KeySequence, Instant), Action>>,
    chord_timeout: Duration,
    mode: Mode,
    current_sequence: Option<KeySequence>,
}

impl KeybindingDaemon {
    fn new() -> Self {
        KeybindingDaemon {
            key_sequence: HashMap::new(),
            chord_timeout: Duration::from_millis(500),
            mode: Mode::NoWindow,
            current_sequence: None,
        }
    }

    fn handle_key_press(&mut self, keycode: u32) {
        let sequence: &KeySequence = if let Some(sequence) = &self.current_sequence {
            sequence
        } else {
            &Vec::new()
        };
        let now = Instant::now();

        todo!()
    }
}

fn main() -> Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    conn.change_window_attributes(root, &ChangeWindowAttributesAux::new().event_mask(EventMask::KEY_PRESS))?;
    conn.flush()?;

    Ok(())
}
