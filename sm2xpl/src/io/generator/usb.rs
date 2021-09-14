use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::common::{
    chain::Supplier,
    delta::DeltaTimeSupplier,
    timer::{DeltaCounter, Elapsed},
};

use super::{
    bounced::BouncedGenerator, constant::ConstGenerator, generator::Generator,
    parameter::Parameter, sequential::SequentialGenerator,
};

pub struct USBParamGenerator {
    generators: Vec<Box<dyn Generator>>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: DeltaCounter,
}

impl From<Rc<RefCell<DeltaTimeSupplier>>> for USBParamGenerator {
    fn from(delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        Self {
            generators: Default::default(),
            delta,
            timer: DeltaCounter::default(),
        }
    }
}

impl USBParamGenerator {
    pub fn delay(mut self, delay: Duration) -> Self {
        self.timer = DeltaCounter::deferred(delay);
        self
    }

    pub fn with_boxed_generator(mut self, generator: Box<dyn Generator>) -> Self {
        self.generators.push(generator);
        self
    }

    pub fn with_const<T: 'static + Parameter>(mut self, generator: ConstGenerator<T>) -> Self {
        self.with_boxed_generator(Box::new(generator))
    }

    pub fn with_sequential<T: 'static + Parameter + Copy>(
        mut self,
        generator: SequentialGenerator<T>,
    ) -> Self {
        self.with_boxed_generator(Box::new(generator))
    }

    pub fn with_bounced<T: 'static + Parameter + Copy>(
        mut self,
        generator: BouncedGenerator<T>,
    ) -> Self {
        self.with_boxed_generator(Box::new(generator))
    }

    fn try_generate(&mut self, delta: Duration) -> Option<Vec<u8>> {
        match self.timer.count(delta) {
            Elapsed::Yes(diff) => {
                self.timer.count(diff);
                Some(self.generate_params(diff))
            }
            Elapsed::No => None,
        }
    }

    fn generate_params(&mut self, diff: Duration) -> Vec<u8> {
        let size_bytes = self.generators_size_bytes();
        let mut params = Vec::with_capacity(size_bytes);
        for generator in self.generators.iter_mut() {
            let delta = self.timer.delay() + diff;
            let bytes = generator.generate(delta);
            params.extend(bytes);
        }
        params
    }

    fn generators_size_bytes(&self) -> usize {
        self.generators
            .iter()
            .map(|generator| generator.size_bytes())
            .fold(0, |acc, val| acc + val)
    }
}

impl Supplier<Option<Vec<u8>>> for USBParamGenerator {
    fn supply(&mut self) -> Option<Vec<u8>> {
        let delta = self.delta.borrow_mut().supply();
        println!("delta: {:?}", delta);
        self.try_generate(delta)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::generator::helper::{ToBounced, ToGenerator};

    use super::*;

    #[test]
    fn should_generate_params() {
        let delay = Duration::from_secs(1);
        let delta = Rc::new(RefCell::new(DeltaTimeSupplier::default()));
        let mut generator = USBParamGenerator::from(delta.clone())
            .with_const(123u16.to_const_generator())
            .with_const(456i16.to_const_generator())
            .with_const(789u32.to_const_generator())
            .with_sequential(5u16.to_sequential_generator().with_step(5))
            .with_sequential(10i16.to_sequential_generator().with_step(10))
            .with_sequential(25u32.to_sequential_generator().with_step(25))
            .with_bounced(5u16.to_const_generator().to_bounced_generator())
            .with_bounced(10i16.to_const_generator().to_bounced_generator())
            .with_bounced(25u32.to_const_generator().to_bounced_generator())
            .delay(delay);

        let params = generator.supply();
        assert!(params.is_none());
        delta.borrow_mut().update(delay);
        let params = generator.supply().unwrap();
        assert_eq!(
            params,
            vec![0, 123, 1, 200, 0, 0, 3, 21, 0, 10, 0, 20, 0, 0, 0, 50, 0, 5, 0, 10, 0, 0, 0, 25]
        );
    }
}
