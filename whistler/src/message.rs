use iced::widget::text_editor::Action;
use std::path::PathBuf;
use crate::search::SearchResult;

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
    /// Markdown preview and other md stuff
    PreviewMarkdown,
    MarkdownLinkClicked(iced::widget::markdown::Uri),
    /// Searching stuff
    ToggleSearch,
    SearchQueryChanged(String),
    SearchCompleted(Vec<SearchResult>),
    SearchResultClicked(PathBuf, usize), // (filepath, line number)
    /// File finding
    ToggleFileFinder,
    FileFinderQueryChanged(String),
    FileFinderSelect, // Enter to open file
    FileFinderNavigate(i32), // This is to go up/down using arrow keys
    /// Fullscreen and window management stuff
    ToggleFullscreen(iced::window::Mode),
    EscapePressed,
}
