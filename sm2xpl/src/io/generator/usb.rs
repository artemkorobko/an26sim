use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::common::{
    chain::Supplier,
    delta::DeltaTimeSupplier,
    timer::{DeltaCounter, Elapsed},
};

use super::{
    bounced::BouncedGenerator, constant::ConstantGenerator, generic::Generator,
    sequential::SequentialGenerator,
};

enum GeneratorImpl {
    GeneratorU16(Box<dyn Generator<u16>>),
    GeneratorI16(Box<dyn Generator<i16>>),
    GeneratorU32(Box<dyn Generator<u32>>),
}

pub struct USBParamGenerator {
    generators: Vec<GeneratorImpl>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: DeltaCounter,
}

impl USBParamGenerator {
    pub fn new(delta: Rc<RefCell<DeltaTimeSupplier>>, delay: Duration) -> Self {
        Self {
            generators: Default::default(),
            delta,
            timer: DeltaCounter::deferred(delay),
        }
    }

    fn generate_params(&mut self, diff: Duration) -> Vec<u8> {
        let mut params = Vec::with_capacity(self.generators.len() * 2);
        for generator_impl in self.generators.iter_mut() {
            let delta = self.timer.delay() + diff;
            match generator_impl {
                GeneratorImpl::GeneratorU16(generator) => {
                    let value = generator.generate(delta);
                    params.extend(value.to_be_bytes());
                }
                GeneratorImpl::GeneratorI16(generator) => {
                    let value = generator.generate(delta);
                    params.extend(value.to_be_bytes());
                }
                GeneratorImpl::GeneratorU32(generator) => {
                    let value = generator.generate(delta);
                    params.extend(value.to_be_bytes());
                }
            }
        }
        params
    }
}

impl USBParamGenerator {
    pub fn with_const_u16(mut self, default: u16) -> Self {
        let generator = ConstantGenerator::new(default);
        self.generators
            .push(GeneratorImpl::GeneratorU16(Box::new(generator)));
        self
    }

    pub fn with_const_i16(mut self, default: i16) -> Self {
        let generator = ConstantGenerator::new(default);
        self.generators
            .push(GeneratorImpl::GeneratorI16(Box::new(generator)));
        self
    }

    pub fn with_const_u32(mut self, default: u32) -> Self {
        let generator = ConstantGenerator::new(default);
        self.generators
            .push(GeneratorImpl::GeneratorU32(Box::new(generator)));
        self
    }

    pub fn with_sequential_u16(mut self, default: u16, step: u16, delay: Duration) -> Self {
        let generator = SequentialGenerator::new(default, step, delay);
        self.generators
            .push(GeneratorImpl::GeneratorU16(Box::new(generator)));
        self
    }

    pub fn with_sequential_i16(mut self, default: i16, step: i16, delay: Duration) -> Self {
        let generator = SequentialGenerator::new(default, step, delay);
        self.generators
            .push(GeneratorImpl::GeneratorI16(Box::new(generator)));
        self
    }

    pub fn with_sequential_u32(mut self, default: u32, step: u32, delay: Duration) -> Self {
        let generator = SequentialGenerator::new(default, step, delay);
        self.generators
            .push(GeneratorImpl::GeneratorU32(Box::new(generator)));
        self
    }

    pub fn with_bounced_u16(
        mut self,
        default: u16,
        step: u16,
        bounce_on: usize,
        delay: Duration,
    ) -> Self {
        let sequential = SequentialGenerator::new(default, step, delay);
        let bounced = BouncedGenerator::new(Box::new(sequential), bounce_on);
        self.generators
            .push(GeneratorImpl::GeneratorU16(Box::new(bounced)));
        self
    }

    pub fn with_bounced_i16(
        mut self,
        default: i16,
        step: i16,
        bounce_on: usize,
        delay: Duration,
    ) -> Self {
        let sequential = SequentialGenerator::new(default, step, delay);
        let bounced = BouncedGenerator::new(Box::new(sequential), bounce_on);
        self.generators
            .push(GeneratorImpl::GeneratorI16(Box::new(bounced)));
        self
    }

    pub fn with_bounced_u32(
        mut self,
        default: u32,
        step: u32,
        bounce_on: usize,
        delay: Duration,
    ) -> Self {
        let sequential = SequentialGenerator::new(default, step, delay);
        let bounced = BouncedGenerator::new(Box::new(sequential), bounce_on);
        self.generators
            .push(GeneratorImpl::GeneratorU32(Box::new(bounced)));
        self
    }

    fn generate(&mut self, delta: Duration) -> Option<Vec<u8>> {
        match self.timer.count(delta) {
            Elapsed::Yes(diff) => {
                self.timer.count(diff);
                Some(self.generate_params(diff))
            }
            Elapsed::No => None,
        }
    }
}

impl Supplier<Option<Vec<u8>>> for USBParamGenerator {
    fn supply(&mut self) -> Option<Vec<u8>> {
        let delta = self.delta.borrow_mut().supply();
        self.generate(delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_params() {
        let delay = Duration::from_secs(1);
        let mut generator = USBParamGenerator::new(delay.clone())
            .with_const_u16(123)
            .with_const_i16(456)
            .with_const_u32(789)
            .with_sequential_u16(5, 5, delay)
            .with_sequential_i16(10, 10, delay)
            .with_sequential_u32(25, 25, delay)
            .with_bounced_u16(5, 5, 10, delay)
            .with_bounced_i16(10, 10, 10, delay)
            .with_bounced_u32(25, 25, 10, delay);

        let params = generator.generate(Duration::ZERO);
        assert!(params.is_none());
        let params = generator.generate(delay).unwrap();
        assert_eq!(
            params,
            vec![0, 123, 1, 200, 0, 0, 3, 21, 0, 10, 0, 20, 0, 0, 0, 50, 0, 10, 0, 20, 0, 0, 0, 50]
        );
    }
}
