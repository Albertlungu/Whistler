use iced::widget::button::{Style as ButtonStyle, Status as ButtonStatus};
use iced::widget::container;
use iced::widget::text_editor;
use iced::{Background, Border, Color, Theme, Vector};

use crate::theme::*;

pub fn tree_button_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(THEME.bg_hover)),
        ButtonStatus::Pressed => Some(Background::Color(THEME.bg_pressed)),
        _ => None,
    };

    ButtonStyle {
        background,
        text_color: THEME.text_secondary,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn tab_button_style(is_active: bool) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, _status| {
        let bg_color = if is_active { THEME.bg_tab_active } else { THEME.bg_tab_inactive };
        ButtonStyle {
            background: Some(Background::Color(bg_color)),
            text_color: THEME.text_secondary,
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
        text_color: THEME.text_dim,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn editor_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_primary)),
        border: Border {
            color: THEME.border_subtle,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: THEME.shadow_dark,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    }
}

pub fn sidebar_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_secondary)),
        border: Border {
            color: THEME.border_subtle,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: THEME.shadow_light,
            offset: Vector::new(-10.0, 0.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}

pub fn status_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_status_bar)),
        border: Border {
            color: THEME.border_very_subtle,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn tab_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_tab_bar)),
        ..Default::default()
    }
}

pub fn text_editor_style(_theme: &Theme, _status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        background: Background::Color(THEME.bg_editor),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: THEME.text_placeholder,
        value: THEME.text_primary,
        selection: THEME.selection,
    }
}

pub fn drag_handle_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(THEME.bg_hover)),
        ButtonStatus::Pressed => Some(Background::Color(THEME.bg_pressed)),
        _ => Some(Background::Color(THEME.bg_drag_handle)),
    };

    ButtonStyle {
        background,
        text_color: Color::TRANSPARENT,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}
