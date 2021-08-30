use std::{cell::RefCell, rc::Rc};

use crate::{common::chain::Mapper, io::delta::DeltaTimeSupplier};

use super::input_params::XPlaneInputParams;

pub struct XPlaneParamInterpolator {
    params: XPlaneInputParams,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
}

impl XPlaneParamInterpolator {
    pub fn new(default: XPlaneInputParams, delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        Self {
            params: default,
            delta,
        }
    }
}

impl Mapper<Option<XPlaneInputParams>, XPlaneInputParams> for XPlaneParamInterpolator {
    fn map(&mut self, input: Option<XPlaneInputParams>) -> XPlaneInputParams {
        if let Some(target) = input {
            self.params = target;
            self.params.clone() // update
        } else {
            self.params.clone() // interpolate
        }
    }
}
