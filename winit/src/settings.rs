//! Configure your application.
use crate::core::window;

use std::borrow::Cow;

/// The settings of an application.
#[derive(Debug, Clone, Default)]
pub struct Settings<Flags> {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The [`window::Settings`].
    pub window: window::Settings,

    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: crate::Application
    pub flags: Flags,

    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,
}

/// The settings of a tray.
#[derive(Debug, Clone, Default)]
pub struct TraySettings {
    /// The icon of the tray.
    pub icon: Option<window::Icon>,
    /// The tooltip of the tray.
    pub title: String,
    /// The width of the tray.
    pub width: f64,
    /// the height of the tray.
    pub height: f64,
}

/// the options for the tray live settings.
#[derive(Debug)]
pub enum LiveTrayOptions<Message> {
    /// close all windows.
    CloseAll,
    /// open a window.
    OpenWindow(window::Id),
    /// send a message on event.
    Message(Message),
    /// open the tray window.
    OpenTrayWindow,
    /// do nothing.
    None,
}

/// The settings that can be changed live.
#[derive(Debug)]
pub struct LiveTraySettings<Message> {
    /// what to do on left click.
    pub left_click: LiveTrayOptions<Message>,
    /// what to do on right click.
    pub right_click: LiveTrayOptions<Message>,
    /// what to do on middle click.
    pub middle_click: LiveTrayOptions<Message>,
}
