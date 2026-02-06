use iced::widget::button::{Style as ButtonStyle, Status as ButtonStatus};
use iced::widget::container;
use iced::widget::text_editor;
use iced::{Background, Border, Color, Theme, Vector};

use crate::theme::*;

pub fn tree_button_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(BG_HOVER)),
        ButtonStatus::Pressed => Some(Background::Color(BG_PRESSED)),
        _ => None,
    };

    ButtonStyle {
        background,
        text_color: TEXT_SECONDARY,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn tab_button_style(is_active: bool) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, _status| {
        let bg_color = if is_active { BG_TAB_ACTIVE } else { BG_TAB_INACTIVE };
        ButtonStyle {
            background: Some(Background::Color(bg_color)),
            text_color: TEXT_SECONDARY,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: BORDER_RADIUS_SMALL.into(),
            },
            shadow: Default::default(),
            snap: false,
        }
    }
}

pub fn tab_close_button_style(_theme: &Theme, _status: ButtonStatus) -> ButtonStyle {
    ButtonStyle {
        background: None,
        text_color: TEXT_DIM,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn editor_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_PRIMARY)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: SHADOW_DARK,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    }
}

pub fn sidebar_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: SHADOW_LIGHT,
            offset: Vector::new(-10.0, 0.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}

pub fn status_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_STATUS_BAR)),
        border: Border {
            color: BORDER_VERY_SUBTLE,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn tab_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_TAB_BAR)),
        ..Default::default()
    }
}

pub fn text_editor_style(_theme: &Theme, _status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        background: Background::Color(BG_EDITOR),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: TEXT_PLACEHOLDER,
        value: TEXT_PRIMARY,
        selection: SELECTION,
    }
}

pub fn drag_handle_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(BG_HOVER)),
        ButtonStatus::Pressed => Some(Background::Color(BG_PRESSED)),
        _ => Some(Background::Color(BG_DRAG_HANDLE)),
    };

    ButtonStyle {
        background,
        text_color: Color::TRANSPARENT,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}
