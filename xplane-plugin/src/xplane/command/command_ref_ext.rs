use xplm_sys::XPLMCommandRef;

use super::commands::Command;

const ENABLE_XPLANE_PHYSICS_ID: u64 = 1;
const DISABLE_XPLANE_PHYSICS_ID: u64 = 2;
const UNKNOWN_ID: u64 = 100;

impl From<Command> for XPLMCommandRef {
    fn from(command: Command) -> Self {
        let command_id = match command {
            Command::EnableXPlanePhysics => ENABLE_XPLANE_PHYSICS_ID,
            Command::DisableXPlanePhysics => DISABLE_XPLANE_PHYSICS_ID,
            _ => UNKNOWN_ID,
        };
        command_id as XPLMCommandRef
    }
}

impl From<XPLMCommandRef> for Command {
    fn from(command: XPLMCommandRef) -> Self {
        let command_id = command as u64;
        match command_id {
            ENABLE_XPLANE_PHYSICS_ID => Self::EnableXPlanePhysics,
            DISABLE_XPLANE_PHYSICS_ID => Self::DisableXPlanePhysics,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disable_xplane_physics_command_to_ref() {
        let command = Command::DisableXPlanePhysics;

        let command_ref = XPLMCommandRef::from(command);

        assert_eq!(command_ref, DISABLE_XPLANE_PHYSICS_ID as XPLMCommandRef);
    }

    #[test]
    fn disable_xplane_physics_ref_to_command() {
        let command_ref = DISABLE_XPLANE_PHYSICS_ID as XPLMCommandRef;

        let command = Command::from(command_ref);

        assert_eq!(command, Command::DisableXPlanePhysics);
    }

    #[test]
    fn enable_xplane_physics_command_to_ref() {
        let command = Command::EnableXPlanePhysics;

        let command_ref = XPLMCommandRef::from(command);

        assert_eq!(command_ref, ENABLE_XPLANE_PHYSICS_ID as XPLMCommandRef);
    }

    #[test]
    fn enable_xplane_physics_ref_to_command() {
        let command_ref = ENABLE_XPLANE_PHYSICS_ID as XPLMCommandRef;

        let command = Command::from(command_ref);

        assert_eq!(command, Command::EnableXPlanePhysics);
    }

    #[test]
    fn unknown_command_to_ref() {
        let command = Command::Unknown;

        let command_ref = XPLMCommandRef::from(command);

        assert_eq!(command_ref, UNKNOWN_ID as XPLMCommandRef);
    }

    #[test]
    fn unknown_ref_to_command() {
        let command_ref = UNKNOWN_ID as XPLMCommandRef;

        let command = Command::from(command_ref);

        assert_eq!(command, Command::Unknown);
    }
}
