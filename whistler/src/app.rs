use iced::keyboard::Key;
use iced::window;
use iced::widget::{text, text_input, column, row, button, container, mouse_area, scrollable, markdown, Space, stack};
use iced::widget::text_editor::{Content, Action};
use iced::{Background, Color, Element, Event, Length, Subscription};
use std::path::PathBuf;

use crate::message::Message;
use crate::file_tree::FileTree;
use crate::theme::*;
use crate::ui::{
    create_editor, editor_container_style, empty_editor, search_input_style, search_panel_style,
    status_bar_style, tab_bar_style, tab_button_style, tab_close_button_style, tree_button_style,
    view_sidebar,
};

#[derive(Debug)]
pub enum TabKind {
    Editor {
        content: Content,
        modified: bool,
    },
    Preview {
        md_items: Vec<markdown::Item>,
    },
}

#[derive(Debug)]
pub struct Tab {
    pub path: PathBuf,
    pub name: String,
    pub kind: TabKind,
}

pub struct App {
    tabs: Vec<Tab>,
    active_tab: Option<usize>,
    cursor_line: usize,
    cursor_col: usize,
    file_tree: Option<FileTree>,
    sidebar_visible: bool,
    sidebar_width: f32,
    resizing_sidebar: bool,
    resize_start_x: Option<f32>,
    resize_start_width: f32,
    search_visible: bool,
    search_query: String,
    search_results: Vec<crate::search::SearchResult>,
    search_input_id: iced::widget::Id,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            cursor_line: 1,
            cursor_col: 1,
            file_tree: None,
            sidebar_visible: true,
            sidebar_width: SIDEBAR_DEFAULT_WIDTH,
            resizing_sidebar: false,
            resize_start_x: None,
            resize_start_width: SIDEBAR_DEFAULT_WIDTH,
            search_visible: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_input_id: iced::widget::Id::unique(),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // The below section basically just creates "instances" for each message,
            // declaring the actual action that each of them does.
            Message::EditorAction(action) => { // This one records a keystroke in the editor
                if let Some(idx) = self.active_tab {
                    if let Some(tab) = self.tabs.get_mut(idx) {
                        if let TabKind::Editor { ref mut content, ref mut modified } = tab.kind {
                            let action = match action {
                            Action::Scroll { lines } => Action::Scroll { lines: lines / 5},
                            other => other,
                        };
                        let _ = content.perform(action);
                        *modified = true;
                        let cursor = content.cursor();
                        self.cursor_line = cursor.position.line + 1;
                        self.cursor_col = cursor.position.column + 1;
                        }
                    }
                }
                iced::Task::none()
            }
            Message::FolderToggled(path) => { // Checks if a folder was clicked
                if let Some(ref mut tree) = self.file_tree {
                    tree.toggle_folder(&path);
                }
                iced::Task::none()
            }
            Message::FileClicked(path) => { // Checks if a file was clicked
                if let Some(ref mut tree) = self.file_tree {
                    tree.select(path.clone()); // Opens the file
                }
                if let Some(idx) = self.tabs.iter().position(|t| t.path == path) {
                    self.active_tab = Some(idx);
                    return iced::Task::none();
                }
                iced::Task::perform(
                    async move {
                        let content = std::fs::read_to_string(&path)
                            .unwrap_or_else(|_| String::from("Could not read file"));
                        (path, content) // Error handling if it is a file that the editor cannot read,
                                        // e.g. image or .pkl (for now)
                    },
                    |(path, content)| Message::FileOpened(path, content)
                )
            }
            Message::TabClosed(idx) => {  // To close a tab using the "x" button
                if idx < self.tabs.len() {
                    self.tabs.remove(idx); // Just removes a tab at that index
                    if self.tabs.is_empty() {
                        self.active_tab = None; // Avoid errors by setting active tab to none if none exist
                    } else if let Some(active) = self.active_tab {
                        if active >= self.tabs.len() {
                            self.active_tab = Some(self.tabs.len() - 1);
                        } else if active > idx {
                            self.active_tab = Some(active - 1);
                        }
                    }
                }
                iced::Task::none()
            }
            Message::CloseActiveTab => { // Closes only the active tab (this is only used once in the code for the keyboard shortcut)
                if let Some(idx) = self.active_tab {
                    self.tabs.remove(idx);
                    if self.tabs.is_empty() {
                        self.active_tab = None; // If there are no tabs, set active tab to none to avoid errors
                    } else if idx >= self.tabs.len() {
                        self.active_tab = Some(self.tabs.len() - 1);
                    }
                }
                iced::Task::none()
            }
            Message::FileOpened(path, content) => {
                let name = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                self.tabs.push(Tab {
                    path,
                    name,
                    kind: TabKind::Editor {
                        content: Content::with_text(&content),
                        modified: false,
                    },
                });
                self.active_tab = Some(self.tabs.len() - 1);
                iced::Task::none()
            }
            Message::TabSelected(idx) => {
                if idx < self.tabs.len() {
                    self.active_tab = Some(idx);
                }
                iced::Task::none()
            }
            Message::FileTreeRefresh => {
                if let Some(ref mut tree) = self.file_tree {
                    tree.refresh();
                }
                iced::Task::none()
            }
            Message::OpenFolderDialog => {
                iced::Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("Open Folder")
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    |result| {
                        match result {
                            Some(path) => Message::FolderOpened(path),
                            None => Message::FileTreeRefresh,
                        }
                    }
                )
            }
            Message::FolderOpened(path) => {
                self.file_tree = Some(FileTree::new(path));
                iced::Task::none()
            }
            Message::SaveFile => {
                if let Some(idx) = self.active_tab {
                    if let Some(tab) = self.tabs.get(idx) {
                        if let TabKind::Editor { ref content, .. } = tab.kind {
                            let path = tab.path.clone();
                        let content = content.text();
                        return iced::Task::perform(
                            async move {
                                std::fs::write(&path, content)
                                    .map_err(|e| e.to_string())
                            },
                            Message::FileSaved,
                        );
                        }
                    }
                }
                iced::Task::none()
            }

            Message::FileSaved(result) => {
                if let Err(e) = result {
                    eprintln!("Failed to save file: {}", e);
                } else if let Some(idx) = self.active_tab {
                    if let Some(tab) = self.tabs.get_mut(idx){
                        if let TabKind::Editor { ref mut modified, .. } = tab.kind {
                            *modified = false;
                        }
                    }
                }
                iced::Task::none()
            }

            Message::SidebarResizeStart => {
                self.resizing_sidebar = true;
                self.resize_start_x = None;
                self.resize_start_width = self.sidebar_width;
                iced::Task::none()
            }
            Message::SidebarResizing(x) => {
                if self.resizing_sidebar {
                    if let Some(start_x) = self.resize_start_x {
                        let delta = start_x - x;
                        self.sidebar_width = (self.resize_start_width + delta)
                            .clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH);
                    } else {
                        self.resize_start_x = Some(x);
                    }
                }
                iced::Task::none()
            }
            Message::SidebarResizeEnd => {
                self.resizing_sidebar = false;
                self.resize_start_x = None;
                iced::Task::none()
            }

            Message::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                iced::Task::none()
            }

            Message::ToggleFullscreen(_mode) => {
                window::oldest().and_then(move |id|{
                    window::maximize(id, true)
                })
            }

            Message::PreviewMarkdown => {
                if let Some(idx) = self.active_tab {
                    if let Some(tab) = self.tabs.get(idx) {
                        if let TabKind::Editor { ref content, .. } = tab.kind {
                            let text = content.text();
                            let md_items: Vec<markdown::Item> = markdown::parse(&text).collect();
                            let preview_name = format!("Preview: {}", tab.name);
                            let path = tab.path.clone();
                            self.tabs.push(Tab {
                                path,
                                name: preview_name,
                                kind: TabKind::Preview { md_items },
                            });
                            self.active_tab = Some(self.tabs.len() - 1);
                        }
                    }
                }
                iced::Task::none()
            }

            Message::MarkdownLinkClicked(_uri) => {
                iced::Task::none()
            }

            Message::ToggleSearch => {
                if self.search_visible {
                    self.search_visible = false;
                    self.search_query.clear();
                    self.search_results.clear();
                } else {
                    self.search_visible = true;
                    return iced::widget::operation::focus(self.search_input_id.clone());
                }
                iced::Task::none()
            }

            Message::SearchQueryChanged(query) => {
                self.search_query = query.clone();

                if query.len() < 2 {
                    self.search_results.clear();
                    return iced::Task::none();
                }

                if let Some(ref tree) = self.file_tree {
                    let root = tree.root.clone();
                    iced::Task::perform(
                        async move {
                            crate::search::search_workspace(&root, &query)
                        },
                        Message::SearchCompleted,
                    )
                } else {
                    iced::Task::none()
                }
            }

            Message::SearchCompleted(results) => {
                self.search_results = results;
                iced::Task::none()
            }

            Message::SearchResultClicked(path, _line_number) => {
                self.search_visible = false;
                self.search_query.clear();
                self.search_results.clear();

                if let Some(ref mut tree) = self.file_tree {
                    tree.select(path.clone());
                }
                if let Some(idx) = self.tabs.iter().position(|t| t.path == path) {
                    self.active_tab = Some(idx);
                    return iced::Task::none();
                }
                iced::Task::perform(
                    async move {
                        let content = std::fs::read_to_string(&path)
                            .unwrap_or_else(|_| String::from("Could not read file"));
                        (path, content)
                    },
                    |(path, content)| Message::FileOpened(path, content),
                )
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        use iced::widget::stack;

        let tab_bar = self.view_tab_bar();
        let editor_widget = self.view_editor();
        let status_bar = self.view_status_bar();

        let editor_container = if self.active_tab.is_some() {
            container(column![tab_bar, editor_widget, status_bar])
        } else {
            self.view_welcome_screen()
        }
        .width(Length::Fill)
        .height(Length::Fill)
        .style(editor_container_style);

        let editor_area = container(editor_container)
            .padding(10)
            .width(Length::Fill);

        let base_content: Element<'_, Message> = if self.sidebar_visible {
            let sidebar = view_sidebar(self.file_tree.as_ref(), self.sidebar_width);

            let resize_zone = mouse_area(
                container(text(""))
                    .width(Length::Fixed(RESIZE_HIT_WIDTH))
                    .height(Length::Fill)
            )
            .on_press(Message::SidebarResizeStart)
            .interaction(iced::mouse::Interaction::ResizingHorizontally);

            row![editor_area, resize_zone, sidebar].into()
        } else {
            editor_area.into()
        };

        let wrapped = container(base_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(THEME.bg_editor)),
                ..Default::default()
            });

        if self.search_visible {
            let search_panel = container(self.view_search_panel())
                .padding(iced::Padding { top: 20.0, right: 0.0, bottom: 0.0, left: 20.0 })
                .width(Length::Fill)
                .height(Length::Fill);

            stack![wrapped, search_panel].into()
        } else {
            wrapped.into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            match event {
                Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                    Some(Message::SidebarResizing(position.x))
                }
                Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                    Some(Message::SidebarResizeEnd)
                }
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: Key::Named(iced::keyboard::key::Named::Escape),
                ..
            }) => {
                return Some(Message::ToggleSearch);
            }
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: Key::Character(c),
                modifiers,
                ..
            }) => {
                if modifiers.command() && modifiers.control() {
                    match c.as_str() {
                        "f" => return Some(Message::ToggleFullscreen(window::Mode::Fullscreen)),
                        _ => {}
                    }
                } else if modifiers.command() && modifiers.shift() {
                    match c.as_str() {
                        "v" | "V" => return Some(Message::PreviewMarkdown),
                        "f" | "F" => return Some(Message::ToggleSearch),
                        _ => {}
                    }
                } else if modifiers.command() {
                    match c.as_str() {
                        "r" => return Some(Message::ToggleSidebar),
                        "o" => return Some(Message::OpenFolderDialog),
                        "w" => return Some(Message::CloseActiveTab),
                        "s" => return Some(Message::SaveFile),
                        _ => {}
                    }
                }
                None
            }
            _ => None,
            }
        })
    }

    fn view_tab_bar(&self) -> Element<'_, Message> {
        if self.tabs.is_empty() {
            return container(text("")).into();
        }

        let tabs: Vec<Element<'_, Message>> = self.tabs
            .iter()
            .enumerate()
            .map(|(idx, tab)| {
                let is_active = self.active_tab == Some(idx);
                let is_modified = matches!(&tab.kind, TabKind::Editor { modified: true, .. });
                let close_icon = if is_modified {
                    text("●").size(10).color(THEME.text_muted)
                } else {
                    text("x").size(10).color(THEME.text_dim)
                };

                button(
                    row![
                        text(&tab.name).size(12).color(THEME.text_muted),
                        button(close_icon)
                            .style(tab_close_button_style)
                            .on_press(Message::TabClosed(idx))
                            .padding(2),
                    ]
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                )
                .style(tab_button_style(is_active))
                .on_press(Message::TabSelected(idx))
                .padding(iced::Padding { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 })
                .into()
            })
            .collect();

        container(row(tabs).spacing(6))
            .padding(iced::Padding { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .width(Length::Fill)
            .style(tab_bar_style)
            .into()
    }

    fn view_search_panel(&self) -> Element<'_, Message> {
        let input = text_input("Search across workspace...", &self.search_query)
            .id(self.search_input_id.clone())
            .on_input(Message::SearchQueryChanged)
            .style(search_input_style)
            .size(13)
            .padding(10)
            .width(Length::Fill);

        let mut content_col = column![input].spacing(6);

        if !self.search_results.is_empty() {
            let mut result_items: Vec<Element<'_, Message>> = Vec::new();

            for result in &self.search_results {
                result_items.push(
                    container(
                        text(&result.file_name)
                            .size(11)
                            .color(THEME.text_secondary)
                    )
                    .padding(iced::Padding { top: 6.0, right: 6.0, bottom: 2.0, left: 6.0 })
                    .into()
                );

                for m in result.matches.iter().take(3) {
                    let line_text = format!("  {}:  {}", m.line_number, m.line_content.trim());
                    let path = result.path.clone();
                    let line_num = m.line_number;

                    result_items.push(
                        button(
                            text(line_text)
                                .size(11)
                                .color(THEME.text_muted)
                        )
                        .style(tree_button_style)
                        .on_press(Message::SearchResultClicked(path, line_num))
                        .padding(iced::Padding { top: 3.0, right: 6.0, bottom: 3.0, left: 12.0 })
                        .width(Length::Fill)
                        .into()
                    );
                }

                if result.matches.len() > 3 {
                    result_items.push(
                        container(
                            text(format!("  ... and {} more", result.matches.len() - 3))
                                .size(10)
                                .color(THEME.text_dim)
                        )
                        .padding(iced::Padding { top: 1.0, right: 6.0, bottom: 2.0, left: 12.0 })
                        .into()
                    );
                }
            }

            let results_scroll = scrollable(
                column(result_items).spacing(1)
            )
            .height(Length::Shrink);

            content_col = content_col.push(
                container(results_scroll).max_height(400.0)
            );
        }

        container(content_col)
            .width(Length::Fixed(320.0))
            .padding(10)
            .style(search_panel_style)
            .into()
    }

    fn view_editor(&self) -> Element<'_, Message> {
        if let Some(idx) = self.active_tab {
            if let Some(tab) = self.tabs.get(idx) {
                match &tab.kind {
                    TabKind::Editor { content, .. } => {
                        let ext = tab.path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("");
                        return create_editor(content, ext);
                    }
                    TabKind::Preview { md_items } => {
                        return scrollable(
                            markdown::view(
                                md_items,
                                markdown::Settings::with_style(markdown::Style::from_palette(
                                    iced::theme::Palette::CATPPUCCIN_MOCHA,
                                )),
                            )
                            .map(Message::MarkdownLinkClicked)
                        )
                        .height(Length::Fill)
                        .into();
                    }
                }
            }
        }
        empty_editor()
    }

    fn view_status_bar(&self) -> Element<'_, Message> {
        container(
            text(format!("Ln {}, Col {}", self.cursor_line, self.cursor_col))
                .size(10)
                .color(THEME.text_placeholder)
        )
        .padding(iced::Padding { top: 4.0, right: 12.0, bottom: 6.0, left: 12.0 })
        .width(Length::Fill)
        .style(status_bar_style)
        .into()
    }

    fn view_welcome_screen(&self) -> iced::widget::Container<'_, Message> {
        let folder_name = self.file_tree
            .as_ref()
            .map(|t| t.root.file_name().unwrap_or_default().to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("No folder open"));

        container(
            column![
                text(folder_name).size(24).color(THEME.text_muted),
                text("Select a file from the sidebar to begin editing")
                    .size(13)
                    .color(THEME.text_placeholder),
            ]
            .spacing(12)
            .align_x(iced::Alignment::Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
    }
}
