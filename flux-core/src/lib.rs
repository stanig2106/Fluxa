use serde::ser::SerializeStruct;
use serde::Serialize;
use std::sync::mpsc::Sender;

pub struct Flux {
    pub memory: Memory,
    // pub config: Config,
    // pub state: State,
}

impl Flux {
    pub fn new(memory_sender: Sender<()>) -> Flux {
        Flux {
            memory: Memory::new(memory_sender),
            // config: Config::new(),
            // state: State::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Memory {
    sender_callback: Sender<()>,
    current_url: Option<String>,
}

impl Memory {
    pub fn new(sender_callback: Sender<()>) -> Memory {
        Memory {
            sender_callback,
            current_url: None,
        }
    }
    pub fn get_current_url(&self) -> Option<String> {
        self.current_url.clone()
    }
    pub fn set_current_url(&mut self, url: String) {
        self.current_url = Some(url);
        self.sender_callback.send(()).unwrap();
    }
}

impl Serialize for Memory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Memory", 1)?;
        state.serialize_field("current_url", &self.current_url)?;
        state.end()
    }
}

