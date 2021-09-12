use std::time::Duration;

use crate::{
    common::timer::{DeltaCounter, Elapsed},
    io::input_params::{InputParamType, InputParameter},
};

use super::{
    bounced::BouncedGenerator, constant::ConstantGenerator, generic::Generator,
    sequential::SequentialGenerator,
};

struct TypedGenerator {
    ip_type: InputParamType,
    generator: Box<dyn Generator>,
}

pub struct USBParamGenerator {
    generators: Vec<TypedGenerator>,
    timer: DeltaCounter,
}

impl USBParamGenerator {
    pub fn new(delay: Duration) -> Self {
        Self {
            generators: Default::default(),
            timer: DeltaCounter::deferred(delay),
        }
    }

    fn generate_params(&mut self, diff: Duration) -> Vec<InputParameter> {
        let mut params = Vec::with_capacity(self.generators.len());
        for t_gen in self.generators.iter_mut() {
            let delta = self.timer.delay() + diff;
            params.push(InputParameter {
                ip_type: t_gen.ip_type,
                value: t_gen.generator.generate(delta),
            });
        }
        params
    }
}

impl USBParamGenerator {
    pub fn add(&mut self, ip_type: InputParamType, generator: Box<dyn Generator>) {
        self.generators.push(TypedGenerator { ip_type, generator });
    }

    pub fn add_const(&mut self, ip_type: InputParamType, default: i16) {
        let generator = ConstantGenerator::new(default);
        self.add(ip_type, Box::new(generator));
    }

    pub fn add_sequential(
        &mut self,
        ip_type: InputParamType,
        default: i16,
        step: i16,
        delay: Duration,
    ) {
        let generator = SequentialGenerator::new(default, step, delay);
        self.add(ip_type, Box::new(generator));
    }

    pub fn add_bounced(
        &mut self,
        ip_type: InputParamType,
        default: i16,
        step: i16,
        bounce_on: usize,
        delay: Duration,
    ) {
        let sequential = SequentialGenerator::new(default, step, delay);
        let bounced = BouncedGenerator::new(Box::new(sequential), bounce_on);
        self.add(ip_type, Box::new(bounced));
    }

    fn generate(&mut self, delta: Duration) -> Option<Vec<InputParameter>> {
        match self.timer.count(delta) {
            Elapsed::Yes(diff) => {
                self.timer.count(diff);
                Some(self.generate_params(diff))
            }
            Elapsed::No => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::generator::{constant::ConstantGenerator, sequential::SequentialGenerator};

    use super::*;

    #[test]
    fn should_generate_params() {
        let const_default = 136;
        let seq_default = 25;
        let seq_step = 25;
        let delay = Duration::from_secs(1);
        let mut generator = USBParamGenerator::new(delay.clone());
        let const_generator = ConstantGenerator::new(const_default);
        let seq_generator = SequentialGenerator::new(seq_default, seq_step, delay.clone());

        generator.add(InputParamType::Ailerons, Box::new(const_generator));
        generator.add(InputParamType::Altitude, Box::new(seq_generator));

        let params = generator.generate(Duration::ZERO);
        assert!(params.is_none());
        let params = generator.generate(delay).unwrap();
        assert_eq!(
            params,
            vec![
                InputParameter {
                    ip_type: InputParamType::Ailerons,
                    value: const_default
                },
                InputParameter {
                    ip_type: InputParamType::Altitude,
                    value: seq_default + seq_step
                }
            ]
        );
    }
}
