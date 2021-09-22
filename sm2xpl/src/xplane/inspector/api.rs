use std::{ffi::CString, os::raw::c_int};

use xplm::geometry::Rect;
use xplm_sys::*;

use super::error::WidgetError;

pub type ApiResult<T> = Result<T, WidgetError>;

pub fn create_label(parent: XPWidgetID, rect: &Rect<i32>, text: &str) -> ApiResult<XPWidgetID> {
    const VISIBLE: c_int = 1;
    const IS_CHILD: c_int = 0;
    const LABEL_CLASS: XPWidgetClass = xpWidgetClass_Caption as XPWidgetClass;
    create_widget(parent, rect, text, VISIBLE, IS_CHILD, LABEL_CLASS)
}

pub fn update_widget(id: XPWidgetID, text: &str) -> ApiResult<()> {
    let text_c = cstring(text)?;
    unsafe { XPSetWidgetDescriptor(id, text_c.as_ptr()) };
    Ok(())
}

pub fn create_window(rect: &Rect<i32>, title: &str) -> ApiResult<XPWidgetID> {
    const NO_PARENT: XPWidgetID = std::ptr::null_mut();
    const INVISIBLE: c_int = 0;
    const IS_ROOT: c_int = 1;
    const WINDOW_CLASS: XPWidgetClass = xpWidgetClass_MainWindow as XPWidgetClass;
    create_widget(NO_PARENT, rect, title, INVISIBLE, IS_ROOT, WINDOW_CLASS)
}

fn create_widget(
    parent: XPWidgetID,
    rect: &Rect<i32>,
    text: &str,
    visible: c_int,
    is_root: c_int,
    class: XPWidgetClass,
) -> ApiResult<XPWidgetID> {
    let text_c = cstring(text)?;
    let id = unsafe {
        XPCreateWidget(
            rect.left(),
            rect.top(),
            rect.right(),
            rect.bottom(),
            visible,
            text_c.as_ptr(),
            is_root,
            parent,
            class,
        )
    };

    if id.is_null() {
        Err(WidgetError::Widget(String::from(text)))
    } else {
        Ok(id)
    }
}

pub fn make_widget_translucent(id: XPWidgetID) {
    unsafe {
        XPSetWidgetProperty(
            id,
            xpProperty_MainWindowType as XPWidgetPropertyID,
            xpMainWindowStyle_Translucent as isize, // xpMainWindowStyle_MainWindow
        )
    }
}

pub fn make_widget_closeable(id: XPWidgetID) {
    const TRUE: isize = 1;
    unsafe {
        XPSetWidgetProperty(
            id,
            xpProperty_MainWindowHasCloseBoxes as XPWidgetPropertyID,
            TRUE,
        )
    }
}

pub fn make_widget_lit(id: XPWidgetID) {
    const LIT_VALUE: isize = 1600;
    unsafe { XPSetWidgetProperty(id, xpProperty_CaptionLit as XPWidgetPropertyID, LIT_VALUE) }
}

fn cstring(val: &str) -> ApiResult<CString> {
    CString::new(val).map_err(|_| WidgetError::InvalidText(String::from(val)))
}
