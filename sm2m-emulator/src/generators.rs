pub const MAX_PARAMS_COUNT: usize = 12;

enum GeneratorState {
    Disabled,
    Increment,
    Decrement,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Default)]
struct GeneratorPeriod {
    limit: u8,
    count: u8,
}

#[derive(Default)]
struct Generator {
    state: GeneratorState,
    period: GeneratorPeriod,
    step: u16,
    value: u16,
}

#[derive(Default)]
pub struct Generators {
    generators: [Generator; MAX_PARAMS_COUNT],
    fps: u8,
    enabled: bool,
}

impl Generators {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn fps(&self) -> u8 {
        self.fps
    }

    pub fn enable(&mut self, fps: u8) {
        self.fps = fps;
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn update_param(&mut self, index: usize, value: u16) -> bool {
        if index < self.generators.len() {
            self.generators[index].value = value;
            true
        } else {
            false
        }
    }

    pub fn enable_generator(&mut self, index: usize, period: u8, step: u16) -> bool {
        if index < self.generators.len() {
            let props = &mut self.generators[index];
            props.state = GeneratorState::Increment;
            props.period.limit = period;
            props.step = step;
            true
        } else {
            false
        }
    }

    pub fn disable_generator(&mut self, index: usize) -> bool {
        if index < self.generators.len() {
            self.generators[index].state = GeneratorState::Disabled;
            true
        } else {
            false
        }
    }
}
