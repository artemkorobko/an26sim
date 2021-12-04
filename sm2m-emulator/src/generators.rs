pub const MAX_PARAMS_COUNT: usize = u8::BITS as usize;

enum Operation {
    Increment,
    Decrement,
}

struct Period {
    limit: u8,
    count: u8,
}

impl Period {
    pub fn elapsed(&mut self) -> bool {
        if self.count >= self.limit {
            self.count = 0;
            true
        } else {
            self.limit += 1;
            false
        }
    }
}

pub struct Generator {
    operation: Operation,
    period: Period,
    step: u16,
    value: u16,
}

impl Generator {
    pub fn generate(&mut self) -> u16 {
        if self.period.elapsed() {
            match self.operation {
                Operation::Increment => self.increment(),
                Operation::Decrement => self.decrement(),
            }
        } else {
            self.value
        }
    }

    fn increment(&mut self) -> u16 {
        match self.value.overflowing_add(self.step) {
            (_, true) => self.operation = Operation::Decrement,
            (value, false) => self.value = value,
        }

        self.value
    }

    fn decrement(&mut self) -> u16 {
        match self.value.overflowing_sub(self.step) {
            (_, true) => self.operation = Operation::Increment,
            (value, false) => self.value = value,
        }

        self.value
    }
}

#[derive(Default)]
pub struct Generators {
    generators: [Option<Generator>; MAX_PARAMS_COUNT],
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

    pub fn enable(&mut self, fps: u8) -> bool {
        self.fps = fps;
        let state = self.enabled;
        self.enabled = true;
        !state
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable_generator(&mut self, index: usize, value: u16, period: u8, step: u16) -> bool {
        if index < self.generators.len() {
            self.generators[index] = Some(Generator {
                operation: Operation::Increment,
                period: Period {
                    limit: period,
                    count: 0,
                },
                step,
                value,
            });
            true
        } else {
            false
        }
    }

    pub fn disable_generator(&mut self, index: usize) -> bool {
        if index < self.generators.len() {
            self.generators[index] = None;
            true
        } else {
            false
        }
    }

    pub fn generate(&mut self) -> GeneratorsIterator {
        for generator in &mut self.generators {
            if let Some(generator) = generator {
                generator.generate();
            }
        }

        GeneratorsIterator {
            generators: &self.generators,
            index: 0,
        }
    }
}

pub struct GeneratorsIterator<'a> {
    generators: &'a [Option<Generator>; MAX_PARAMS_COUNT],
    index: usize,
}

impl<'a> Iterator for GeneratorsIterator<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let generator = &self.generators[self.index];
        while generator.is_none() && self.index < self.generators.len() {
            self.index += 1;
        }

        generator.as_ref().map(|generator| generator.value)
    }
}
