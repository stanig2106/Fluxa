pub struct Flux {
    pub memory: Memory,
    // pub config: Config,
    // pub state: State,
}

impl Flux {
    pub fn new() -> Flux {
        Flux {
            memory: Memory::new(),
            // config: Config::new(),
            // state: State::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Memory {
    current_url: Option<String>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            current_url: None,
        }
    }
    pub fn get_current_url(&self) -> Option<String> {
        self.current_url.clone()
    }
    pub fn set_current_url(&mut self, url: String) {
        self.current_url = Some(url);
    }
}

