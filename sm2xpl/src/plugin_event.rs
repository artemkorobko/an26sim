#[derive(Copy, Clone)]
pub enum PluginEvent {
    EnablePhysics,
    DisablePhysics,
    ShowDebugWindow,
    HideDebugWindow,
    StartTest,
    StopTest,
}
