use iced::widget::text_editor::Action;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    /// Text editing stuff
    EditorAction(Action),
    /// Filesystem stuff
    FileClicked(PathBuf),
    FileOpened(PathBuf, String),
    FolderToggled(PathBuf),
    FileTreeRefresh,
    ToggleSidebar,
    OpenFolderDialog,
    FolderOpened(PathBuf),
    SaveFile,
    FileSaved(Result<(), String>),
    /// Tab stuff
    TabSelected(usize),
    TabClosed(usize),
    CloseActiveTab,
    ///Sidebar stuff
    SidebarResizeStart,
    SidebarResizing(f32),
    SidebarResizeEnd,
    /// Fullscreen and window management stuff
    ToggleFullscreen(iced::window::Mode),
}
