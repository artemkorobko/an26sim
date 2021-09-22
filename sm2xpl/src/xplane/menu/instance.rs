use crate::plugin_event::PluginEvent;
use crate::xplane::menu::api;
use crate::xplane::menu::api::{check_item, toggle_item, uncheck_item};
use crate::xplane::menu::item::MenuItem;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use xplm_sys::XPLMMenuID;

use super::api::ApiResult;

pub struct PluginMenu {
    tx: Sender<PluginEvent>,
    root: XPLMMenuID,
    items: HashMap<MenuItem, i32>,
}

impl PluginMenu {
    pub fn new(tx: Sender<PluginEvent>) -> Self {
        Self {
            tx,
            root: std::ptr::null_mut(),
            items: HashMap::new(),
        }
    }

    pub fn check_item(&mut self, id: MenuItem) {
        if let Some(index) = self.items.get(&id) {
            check_item(self.root, *index);
        }
    }

    pub fn uncheck_item(&mut self, id: MenuItem) {
        if let Some(index) = self.items.get(&id) {
            uncheck_item(self.root, *index);
        }
    }

    pub fn handle_click(&mut self, id: MenuItem) {
        if let Some(index) = self.items.get(&id) {
            match id {
                MenuItem::Undefined => {}
                MenuItem::Physics => match toggle_item(self.root, *index) {
                    true => self.send_event(PluginEvent::EnablePhysics),
                    false => self.send_event(PluginEvent::DisablePhysics),
                },
                MenuItem::Inspector => match toggle_item(self.root, *index) {
                    true => self.send_event(PluginEvent::ShowDebugWindow),
                    false => self.send_event(PluginEvent::HideDebugWindow),
                },
                MenuItem::Test => match toggle_item(self.root, *index) {
                    true => self.send_event(PluginEvent::StartTest),
                    false => self.send_event(PluginEvent::StopTest),
                },
            }
        }
    }

    fn send_event(&self, event: PluginEvent) {
        self.tx.send(event).expect("Unable to send menu event");
    }
}

pub fn create(menu: &mut Box<PluginMenu>) -> ApiResult<()> {
    const MENU_TEXT: &str = "SM2M";
    let plugins_menu = api::find_plugins_menu()?;
    let root_index = api::append_menu_item(plugins_menu, MENU_TEXT)?;
    let root_menu = api::create_menu(plugins_menu, root_index, MENU_TEXT, &*menu)?;
    let item = api::append_checked_item(root_menu, "X-Plane physics", MenuItem::Physics)?;
    menu.items.insert(MenuItem::Physics, item);
    let item = api::append_unchecked_item(root_menu, "Inspector window", MenuItem::Inspector)?;
    menu.items.insert(MenuItem::Inspector, item);
    api::append_separator(root_menu);
    let item = api::append_unchecked_item(root_menu, "Parameters test", MenuItem::Test)?;
    menu.items.insert(MenuItem::Test, item);
    menu.root = root_menu;
    Ok(())
}
