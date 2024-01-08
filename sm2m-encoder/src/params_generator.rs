use crate::params::PARAMS_COUNT;

pub enum GeneratorState {
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
pub struct GeneratorPeriod {
    limit: u8,
    count: u8,
}

#[derive(Default)]
pub struct GeneratorProps {
    state: GeneratorState,
    period: GeneratorPeriod,
    step: u16,
}

#[derive(Default)]
pub struct ParamsGenerator {
    props: [GeneratorProps; PARAMS_COUNT],
}

impl ParamsGenerator {
    pub fn enable(&mut self, index: usize, period: u8, step: u16) -> bool {
        if index < PARAMS_COUNT {
            let props = &mut self.props[index];
            props.state = GeneratorState::Increment;
            props.period.limit = period;
            props.step = step;
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, index: usize) -> bool {
        if index < PARAMS_COUNT {
            self.props[index].state = GeneratorState::Disabled;
            true
        } else {
            false
        }
    }
}
