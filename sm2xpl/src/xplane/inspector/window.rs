use std::{os::raw::c_int, sync::mpsc::Sender, time::Duration};

use xplm::geometry::Rect;
use xplm_sys::*;

use crate::{
    io::metrics::IOMetrics,
    plugin_event::PluginEvent,
    xplane::input_params::{General, View, XPlaneInputParams},
};

use super::{
    api::{self, ApiResult},
    rect_ext::RectExt,
    widgets::Widgets,
};

const SELF_REF_PROP: XPWidgetPropertyID = 1101;

pub struct InspectorWindow {
    tx: Sender<PluginEvent>,
    window: XPWidgetID,
    widgets: Widgets,
}

impl InspectorWindow {
    pub fn new(tx: Sender<PluginEvent>, width: i32, height: i32, title: &str) -> ApiResult<Self> {
        let window_rect = Rect::from_size(width, height);
        let window = api::create_window(&window_rect, title)?;
        api::make_widget_translucent(window);
        api::make_widget_closeable(window);

        Ok(Self {
            tx,
            window,
            widgets: Widgets::new(window, &window_rect)?,
        })
    }

    pub fn show(&self) {
        self.register_close_handler();
        unsafe { XPShowWidget(self.window) };
    }

    pub fn hide(&self) {
        unsafe { XPHideWidget(self.window) };
    }

    pub fn visible(&self) -> bool {
        const VISIBLE: c_int = 1;
        let visible = unsafe { XPIsWidgetVisible(self.window) };
        visible == VISIBLE
    }

    pub fn update(
        &self,
        params: &XPlaneInputParams,
        general: &General,
        view: &View,
        terrain: f32,
        input: &mut IOMetrics,
        output: &mut IOMetrics,
        delta: &Duration,
    ) -> ApiResult<()> {
        self.widgets
            .update(params, general, view, terrain, input, output, delta)
    }

    fn register_close_handler(&self) {
        let window_ref =
            unsafe { XPGetWidgetProperty(self.window, SELF_REF_PROP, std::ptr::null_mut()) };
        if window_ref == 0 {
            let window_ref: *const InspectorWindow = self;
            unsafe { XPSetWidgetProperty(self.window, SELF_REF_PROP, window_ref as isize) }
            unsafe { XPAddWidgetCallback(self.window, Some(widget_handler)) };
        }
    }

    fn handle_close(&self) {
        self.tx
            .send(PluginEvent::HideDebugWindow)
            .expect("Unable to send hide debug window event")
    }
}

pub(super) unsafe extern "C" fn widget_handler(
    message: XPWidgetMessage,
    widget: XPWidgetID,
    _: isize,
    _: isize,
) -> c_int {
    const HANDLE_OK: c_int = 1;
    const HANDLE_DISCARD: c_int = 0;
    if message == xpMessage_CloseButtonPushed as XPWidgetMessage {
        let window_ref = XPGetWidgetProperty(widget, SELF_REF_PROP, std::ptr::null_mut());
        if window_ref > 0 {
            let window = window_ref as *const InspectorWindow;
            (*window).handle_close();
            HANDLE_OK
        } else {
            HANDLE_DISCARD
        }
    } else {
        HANDLE_DISCARD
    }
}
