// use std::{cell::RefCell, rc::Rc};

// use crate::common::chain::Supplier;

// use super::{dataref::collection::DataRefs, output_params::XPlaneOutputParams};

// pub struct XPlaneOutputSupplier {
//     datarefs: Rc<RefCell<DataRefs>>,
// }

// impl XPlaneOutputSupplier {
//     pub fn new(datarefs: Rc<RefCell<DataRefs>>) -> Self {
//         Self { datarefs }
//     }
// }

// impl Supplier<XPlaneOutputParams> for XPlaneOutputSupplier {
//     fn supply(&mut self) -> XPlaneOutputParams {
//         self.datarefs.borrow().as_output()
//     }
// }
