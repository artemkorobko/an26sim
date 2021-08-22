use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    common::{chain::Supplier, timer::DeltaCounter},
    io::delta::DeltaTimeSupplier,
};

use super::{bounced::BouncedGenerator, generic::Generator};

const GEN_TIMEOUT: Duration = Duration::from_millis(50);

pub struct USBParamGenerator {
    params: Vec<GeneratorType>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: DeltaCounter,
}

impl USBParamGenerator {
    pub fn constant(delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        Self {
            params: vec![GeneratorType::ConstU16(0); 18],
            delta,
            timer: DeltaCounter::immediate(GEN_TIMEOUT),
        }
    }

    pub fn dynamic(delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        let params = vec![
            GeneratorType::ConstU16(0), // Latitude HI
            GeneratorType::ConstU16(0), // Latitude LO
            GeneratorType::ConstU16(0), // Longitude HI
            GeneratorType::ConstU16(0), // Longitude LO
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(1, 0, 8000, GEN_TIMEOUT * 2)), // Altitude
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(35, 0, 32767, GEN_TIMEOUT)), // Heading
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeI16(Generator::new(350, -32767, 32767, GEN_TIMEOUT)), // Pitch
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeI16(Generator::new(300, -32767, 32767, GEN_TIMEOUT)), // Roll
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeI16(Generator::new(2000, -32767, 32767, GEN_TIMEOUT)), // Ailerons
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeI16(Generator::new(2000, -32767, 32767, GEN_TIMEOUT)), // Elevator
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeI16(Generator::new(2000, -32767, 32767, GEN_TIMEOUT)), // Rudder
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(300, 0, 32767, GEN_TIMEOUT)), // Flaps
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(300, 0, 32767, GEN_TIMEOUT)), // Left engine
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(300, 0, 32767, GEN_TIMEOUT)), // Right engine
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(400, 0, 32767, GEN_TIMEOUT)), // Front gear
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(400, 0, 32767, GEN_TIMEOUT)), // Left gear
            GeneratorType::ConstU16(0),
            // GeneratorType::RangeU16(Generator::new(400, 0, 32767, GEN_TIMEOUT)), // Right gear
            // Lights
            GeneratorType::ConstU16(0),
            // GeneratorType::Range({
            //     let mut generator = Generator::full_range(1, GEN_TIMEOUT);
            //     generator.max = 7;
            //     generator.timer = TimeCounter::new(Duration::from_secs(3));
            //     generator
            // }),
            // Init
            GeneratorType::ConstU16(0),
            // GeneratorType::Range({
            //     let mut generator = Generator::full_range(1, GEN_TIMEOUT);
            //     generator.max = 1;
            //     generator.timer = TimeCounter::new(Duration::from_secs(10));
            //     generator
            // }),
        ];

        Self {
            params,
            delta,
            timer: DeltaCounter::immediate(GEN_TIMEOUT),
        }
    }

    pub fn update_params(&mut self, params: &Vec<u16>) {
        for (idx, param) in params.iter().enumerate() {
            self.update_param(idx, *param);
        }
    }

    fn update_param(&mut self, idx: usize, value: u16) {
        let param = &mut self.params[idx];
        match param {
            GeneratorType::ConstU16(param) => *param = value,
            GeneratorType::RangeU16(generator) => generator.reset(value),
            GeneratorType::RangeI16(generator) => generator.reset(value as i16),
        }
    }

    fn can_supply(&mut self, delta: &Duration) -> bool {
        self.timer.count(delta).is_elapsed()
    }

    fn update(&mut self, delta: &Duration) {
        for generator in self.params.iter_mut() {
            generator.generate(delta);
        }
    }

    fn generate(&mut self, delta: &Duration) -> Vec<u16> {
        self.params
            .iter_mut()
            .map(|generator| generator.generate(delta).reverse_bits())
            .collect()
    }
}

impl Supplier<Option<Vec<u16>>> for USBParamGenerator {
    fn supply(&mut self) -> Option<Vec<u16>> {
        let delta = self.delta.borrow_mut().supply();
        if self.can_supply(&delta) {
            Some(self.generate(&delta))
        } else {
            self.update(&delta);
            None
        }
    }
}

#[derive(Copy, Clone)]
enum GeneratorType {
    ConstU16(u16),
    RangeU16(BouncedGenerator<u16>),
    RangeI16(BouncedGenerator<i16>),
}

impl GeneratorType {
    fn generate(&mut self, delta: &Duration) -> u16 {
        match self {
            GeneratorType::ConstU16(param) => *param,
            GeneratorType::RangeU16(generator) => generator.generate(delta),
            GeneratorType::RangeI16(generator) => generator.generate(delta) as u16,
        }
    }
}
