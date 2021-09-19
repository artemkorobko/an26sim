use std::{cell::RefCell, rc::Rc};

use crate::{common::pipeline::Supplier, xplane::output_params::XPlaneOutputParams};

use super::collection::DataRefs;

pub struct XPlaneOutputSupplier {
    datarefs: Rc<RefCell<DataRefs>>,
}

impl XPlaneOutputSupplier {
    pub fn new(datarefs: Rc<RefCell<DataRefs>>) -> Self {
        Self { datarefs }
    }
}

impl Supplier<XPlaneOutputParams> for XPlaneOutputSupplier {
    fn supply(&mut self) -> XPlaneOutputParams {
        XPlaneOutputParams::from(self.datarefs.borrow())
    }
}
