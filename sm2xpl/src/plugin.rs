use std::sync::mpsc;

use xplm::{
    flight_loop::FlightLoop,
    plugin::{Plugin, PluginInfo},
};

use crate::{
    controller::Controller,
    plugin_error::PluginError,
    plugin_event::PluginEvent,
    xplane::{dataref::collection::DataRefs, inspector::window::InspectorWindow, menu::instance},
    PLUGIN_NAME,
};

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 300;

pub struct SM2MPlugin {
    #[allow(dead_code)]
    flight_loop: FlightLoop,
}

impl Plugin for SM2MPlugin {
    type Error = PluginError;

    fn start() -> Result<Self, Self::Error> {
        let (tx, rx) = mpsc::channel::<PluginEvent>();
        let mut menu = Box::new(instance::PluginMenu::new(tx.clone()));
        instance::create(&mut menu)?;
        let inspector = InspectorWindow::new(tx.clone(), WINDOW_WIDTH, WINDOW_HEIGHT, PLUGIN_NAME)?;
        let data_refs = DataRefs::new()?;
        let controller = Controller::new(menu, inspector, data_refs, rx);
        let mut flight_loop = FlightLoop::new(controller);
        flight_loop.schedule_immediate();
        Ok(Self { flight_loop })
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn disable(&mut self) {}

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: PLUGIN_NAME.to_owned(),
            signature: "an26.sm2m".to_owned(),
            description: "An26 simulator SM2M communication module".to_owned(),
        }
    }
}
