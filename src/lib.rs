use crate::error::TrayResult;
use crate::platform::NativeTrayIcon;

mod platform;
mod error;
mod icon;
mod utils;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TrayIconBuilder<T = ()> {
    menu: Option<Menu<T>>,
    tooltip: Option<String>
}

impl<T> TrayIconBuilder<T> {

    pub fn new() -> Self {
        Self {
            menu: None,
            tooltip: None,
        }
    }

    pub fn with_menu(mut self, menu: Menu<T>) -> Self {
        self.menu = Some(menu);
        self
    }

    pub fn with_tooltip<S: ToString>(mut self, tooltip: S) -> Self {
        self.tooltip = Some(tooltip.to_string());
        self
    }

}

impl<T: Clone + 'static> TrayIconBuilder<T> {

    pub fn build<F>(self, callback: F) -> TrayResult<TrayIcon>
        where F: FnMut(TrayEvent<T>) + Send + 'static
    {
        Ok(TrayIcon(NativeTrayIcon::new(self, callback)?))
    }

}

pub struct TrayIcon(NativeTrayIcon);

impl TrayIcon {
    pub fn set_tooltip<S: ToString>(&self, tooltip: impl Into<Option<S>>) {
        self.0.set_tooltip(tooltip.into().map(|s| s.to_string()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClickType {
    Left,
    Right,
    Double
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrayEvent<T> {
    Tray(ClickType),
    Menu(T)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Menu<T> {
    items: Vec<MenuItem<T>>
}

impl<T> Menu<T> {
    pub fn new<I>(items: I) -> Self
        where I: IntoIterator<Item=MenuItem<T>>
    {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MenuItem<T> {
    Separator,
    Button {
        name: String,
        signal: T
    },
    Menu {
        name: String,
        children: Vec<MenuItem<T>>
    }
}

impl<T> MenuItem<T> {

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn button<S>(name: S, signal: T) -> Self
        where S: ToString
    {
        Self::Button {
            name: name.to_string(),
            signal,
        }
    }

    pub fn menu<S, I>(name: S, children: I) -> Self
        where S: ToString, I: IntoIterator<Item=MenuItem<T>>
    {
        Self::Menu {
            name: name.to_string(),
            children: children.into_iter().collect(),
        }
    }

}