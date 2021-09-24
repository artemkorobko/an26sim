use std::{ffi, ops::DerefMut, os::raw};

use super::commands::Command;

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("String contains null byte: {0}")]
    InvalidString(String),
}

pub type ApiResult<T> = Result<T, CommandError>;

const EXEC_BEFORE_XPLANE: raw::c_int = 1;

pub fn create_command<T: AsRef<str>>(name: T, description: T) -> ApiResult<()> {
    let c_name = cstring(name.as_ref())?;
    let c_description = cstring(description.as_ref())?;
    unsafe { xplm_sys::XPLMCreateCommand(c_name.as_ptr(), c_description.as_ptr()) };
    Ok(())
}

pub fn send_command(command: Command) {
    let command_ref = xplm_sys::XPLMCommandRef::from(command);
    unsafe { xplm_sys::XPLMCommandOnce(command_ref) };
}

pub trait CommandHandler {
    fn handle(&mut self, command: Command);
}

pub struct OwnedCommand {
    data: Box<OwnedCommandData>,
    native_handler: xplm_sys::XPLMCommandCallback_f,
}

impl Drop for OwnedCommand {
    fn drop(&mut self) {
        let data_ref = self.data.deref_mut() as *mut OwnedCommandData;
        unsafe {
            xplm_sys::XPLMUnregisterCommandHandler(
                self.data.command.into(),
                self.native_handler,
                EXEC_BEFORE_XPLANE,
                data_ref as *mut ffi::c_void,
            );
        }
    }
}

pub struct OwnedCommandData {
    command: Command,
    rust_handler: Box<dyn CommandHandler>,
}

pub fn register_command_handler<T: CommandHandler + 'static>(
    command: Command,
    handler: T,
) -> ApiResult<OwnedCommand> {
    let mut owned_data = Box::new(OwnedCommandData {
        rust_handler: Box::new(handler),
        command,
    });
    let owned_data_ptr = owned_data.deref_mut() as *mut OwnedCommandData;

    unsafe {
        xplm_sys::XPLMRegisterCommandHandler(
            xplm_sys::XPLMCommandRef::from(command),
            Some(command_handler),
            EXEC_BEFORE_XPLANE,
            owned_data_ptr as *mut ffi::c_void,
        )
    };

    Ok(OwnedCommand {
        data: owned_data,
        native_handler: Some(command_handler),
    })
}

unsafe extern "C" fn command_handler(
    command_ref: xplm_sys::XPLMCommandRef,
    phase: xplm_sys::XPLMCommandPhase,
    refcon: *mut ffi::c_void,
) -> raw::c_int {
    const PREVENT_FURTHER_PROCESSING: raw::c_int = 0;
    let owned_handler = refcon as *mut OwnedCommandData;
    if !owned_handler.is_null() && phase == xplm_sys::xplm_CommandEnd as xplm_sys::XPLMCommandPhase
    {
        let handler = &mut (*owned_handler).rust_handler;
        handler.handle(command_ref.into());
    }
    PREVENT_FURTHER_PROCESSING
}

pub fn cstring(value: &str) -> ApiResult<ffi::CString> {
    ffi::CString::new(value).map_err(|_| CommandError::InvalidString(String::from(value)))
}
