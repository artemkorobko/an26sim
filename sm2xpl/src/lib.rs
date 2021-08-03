mod common;
mod controller;
mod io;
mod plugin;
mod plugin_error;
mod plugin_event;
mod xplane;

pub const PLUGIN_NAME: &str = "SM2M Communication";

xplm::xplane_plugin!(plugin::SM2MPlugin);
