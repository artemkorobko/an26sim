use std::{cell::RefCell, rc::Rc, time::Duration};

use num_traits::PrimInt;
use rand::{distributions::uniform::SampleUniform, prelude::ThreadRng, Rng};
use xplm::debugln;

use crate::common::{chain::Supplier, timer::TimeCounter};

use super::{delta::DeltaTimeSupplier, params::input::*};

const GEN_TIMEOUT: Duration = Duration::from_millis(50);

pub struct USBParamGenerator {
    params: Vec<GeneratorType>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: TimeCounter,
}

impl USBParamGenerator {
    pub fn constant(delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        Self {
            params: vec![GeneratorType::ConstU16(0); PARAMS_COUNT],
            delta,
            timer: TimeCounter::new(GEN_TIMEOUT),
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
            timer: TimeCounter::new(GEN_TIMEOUT),
        }
    }

    pub fn set_latitude(&mut self, hi: u16, lo: u16) {
        self.update_param(LAT_HI_IDX, hi);
        self.update_param(LAT_LO_IDX, lo);
    }

    pub fn set_longitude(&mut self, hi: u16, lo: u16) {
        self.update_param(LON_HI_IDX, hi);
        self.update_param(LON_LO_IDX, lo);
    }

    pub fn set_altitude(&mut self, value: u16) {
        self.update_param(ALT_IDX, value);
    }

    pub fn set_heading(&mut self, value: u16) {
        self.update_param(HDG_IDX, value);
    }

    pub fn set_pitch(&mut self, value: u16) {
        self.update_param(PITCH_IDX, value);
    }

    pub fn set_roll(&mut self, value: u16) {
        self.update_param(ROLL_IDX, value);
    }

    pub fn set_ailerons(&mut self, value: u16) {
        self.update_param(AIL_IDX, value);
    }

    pub fn set_elevator(&mut self, value: u16) {
        self.update_param(ELEV_IDX, value);
    }

    pub fn set_rudder(&mut self, value: u16) {
        self.update_param(RUD_IDX, value);
    }

    pub fn set_flaps(&mut self, value: u16) {
        self.update_param(FLP_IDX, value);
    }

    pub fn set_engine_left(&mut self, value: u16) {
        self.update_param(ENG_L_IDX, value);
    }

    pub fn set_engine_right(&mut self, value: u16) {
        self.update_param(ENG_R_IDX, value);
    }

    pub fn set_gear_front(&mut self, value: u16) {
        self.update_param(GEAR_F_IDX, value);
    }

    pub fn set_gear_left(&mut self, value: u16) {
        self.update_param(GEAR_L_IDX, value);
    }

    pub fn set_gear_right(&mut self, value: u16) {
        self.update_param(GEAR_R_IDX, value);
    }

    pub fn set_lights(&mut self, value: u16) {
        self.update_param(LIGHTS_IDX, value);
    }

    fn update_param(&mut self, idx: usize, value: u16) {
        let param = &mut self.params[idx];
        match param {
            GeneratorType::ConstU16(param) => *param = value,
            GeneratorType::RangeU16(generator) => generator.param = value,
            GeneratorType::RangeI16(generator) => generator.param = value as i16,
        }
    }

    fn can_supply(&mut self, delta: &Duration) -> bool {
        self.timer.count(delta).is_some()
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

impl Supplier<Option<InputParams>> for USBParamGenerator {
    fn supply(&mut self) -> Option<InputParams> {
        let delta = self.delta.borrow_mut().supply();
        if self.can_supply(&delta) {
            let params_vec = self.generate(&delta);
            let params = InputParams::from_vec(params_vec);
            match params {
                Ok(params) => Some(params),
                Err(error) => {
                    debugln!("{}", error.to_string());
                    None
                }
            }
        } else {
            self.update(&delta);
            None
        }
    }
}

#[derive(Copy, Clone)]
enum GeneratorType {
    ConstU16(u16),
    RangeU16(Generator<u16>),
    RangeI16(Generator<i16>),
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

#[derive(Copy, Clone)]
enum Direction {
    Increase,
    Decrease,
}

#[derive(Copy, Clone)]
struct Generator<T> {
    param: T,
    step: T,
    min: T,
    max: T,
    direction: Direction,
    timer: TimeCounter,
    cnt: usize,
}

impl<T: PrimInt + SampleUniform + std::fmt::Debug> Generator<T> {
    fn new(step: T, min: T, max: T, timeout: Duration) -> Self {
        Self {
            param: T::zero(),
            step,
            min,
            max,
            direction: Direction::Increase,
            timer: TimeCounter::new(timeout),
            cnt: 0,
        }
    }

    fn can_calculate(&mut self, delta: &Duration) -> bool {
        self.timer.count(delta).is_some()
    }

    fn should_bounce(&mut self, random: &mut ThreadRng) -> bool {
        self.cnt += 1;
        if self.cnt == 50 {
            self.cnt = 0;
            random.gen::<bool>()
        } else {
            false
        }
    }

    fn in_range(&self, other: T) -> bool {
        other > self.min && other < self.max
    }

    fn reverse_direction(&mut self) {
        match self.direction {
            Direction::Increase => self.direction = Direction::Decrease,
            Direction::Decrease => self.direction = Direction::Increase,
        }
    }

    fn calculate(&mut self) -> (T, bool) {
        let result = match self.direction {
            Direction::Increase => self.param.checked_add(&self.step),
            Direction::Decrease => self.param.checked_sub(&self.step),
        };

        match result {
            Some(result) => (result, false),
            None => (self.param, true),
        }
    }

    fn bounce(&self, random: &mut ThreadRng) -> T {
        let value = random.gen_range(self.min..self.max);
        println!("BOUNCE: {:?} - {:?}", self.param, value);
        value
    }

    fn generate(&mut self, delta: &Duration) -> T {
        if self.can_calculate(delta) {
            let (calculated, overflow) = self.calculate();
            if !self.in_range(calculated) || overflow {
                self.reverse_direction();
                self.param
            } else {
                self.param = calculated;
                let mut random = rand::thread_rng();
                if self.should_bounce(&mut random) {
                    self.bounce(&mut random)
                } else {
                    self.param
                }
            }
        } else {
            self.param
        }
    }
}
