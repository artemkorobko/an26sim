use std::ffi::{c_void, CString};

use num_traits::FromPrimitive;
use xplm_sys::*;

use super::{error::MenuError, instance::PluginMenu, item::MenuItem};

pub type ApiResult<T> = Result<T, MenuError>;

pub fn find_plugins_menu() -> ApiResult<XPLMMenuID> {
    let menu = unsafe { XPLMFindPluginsMenu() };
    if menu != std::ptr::null_mut() {
        Ok(menu)
    } else {
        Err(MenuError::PluginsMenu)
    }
}

pub fn append_menu_item(parent: XPLMMenuID, text: &str) -> ApiResult<i32> {
    append_action_item(parent, text, MenuItem::Undefined)
}

pub fn append_action_item(parent: XPLMMenuID, text: &str, id: MenuItem) -> ApiResult<i32> {
    let text_c = cstring(text)?;
    let id_int = id as i32;
    let index = unsafe { XPLMAppendMenuItem(parent, text_c.as_ptr(), id_int as *mut _, 0) };
    if index >= 0 {
        Ok(index)
    } else {
        Err(MenuError::Append(String::from(text)))
    }
}

pub fn append_checked_item(parent: XPLMMenuID, text: &str, id: MenuItem) -> ApiResult<i32> {
    let index = append_action_item(parent, text, id)?;
    check_item(parent, index);
    Ok(index)
}

pub fn append_unchecked_item(parent: XPLMMenuID, text: &str, id: MenuItem) -> ApiResult<i32> {
    let index = append_action_item(parent, text, id)?;
    uncheck_item(parent, index);
    Ok(index)
}

pub fn append_separator(parent: XPLMMenuID) {
    unsafe { XPLMAppendMenuSeparator(parent) };
}

pub fn check_item(parent: XPLMMenuID, index: i32) {
    let state = xplm_Menu_Checked as XPLMMenuCheck;
    unsafe { XPLMCheckMenuItem(parent, index, state) };
}

pub fn uncheck_item(parent: XPLMMenuID, index: i32) {
    let state = xplm_Menu_NoCheck as XPLMMenuCheck;
    unsafe { XPLMCheckMenuItem(parent, index, state) };
}

pub fn toggle_item(parent: XPLMMenuID, index: i32) -> bool {
    if is_item_checked(parent, index) {
        uncheck_item(parent, index);
        false
    } else {
        check_item(parent, index);
        true
    }
}

pub fn is_item_checked(parent: XPLMMenuID, index: i32) -> bool {
    let mut state = xplm_Menu_NoCheck as XPLMMenuCheck;
    unsafe { XPLMCheckMenuItemState(parent, index, &mut state) };
    state == xplm_Menu_Checked as XPLMMenuCheck
}

pub fn create_menu(
    parent: XPLMMenuID,
    index: i32,
    text: &str,
    menu: &PluginMenu,
) -> ApiResult<XPLMMenuID> {
    let text_c = cstring(text)?;
    let menu_ref = menu as *const PluginMenu;
    let id = unsafe {
        XPLMCreateMenu(
            text_c.as_ptr(),
            parent,
            index,
            Some(menu_handler),
            menu_ref as *mut _,
        )
    };

    if id != std::ptr::null_mut() {
        Ok(id)
    } else {
        Err(MenuError::Create(String::from(text)))
    }
}

pub fn cstring(val: &str) -> ApiResult<CString> {
    CString::new(val).map_err(|_| MenuError::InvalidText(String::from(val)))
}

unsafe extern "C" fn menu_handler(menu: *mut c_void, item: *mut c_void) {
    let menu = menu as *mut PluginMenu;
    let item_id = item as i32;
    if let Some(id) = FromPrimitive::from_i32(item_id) {
        (*menu).handle_click(id);
    }
}
