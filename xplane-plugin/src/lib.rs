mod controller;
mod io;
mod plugin;
mod plugin_error;
mod plugin_event;
mod shared;
mod usb;
mod xplane;

pub const PLUGIN_NAME: &str = "SM2M Communication";

xplm::xplane_plugin!(plugin::SM2MPlugin);

// use thread for USB and MPSC for communication
