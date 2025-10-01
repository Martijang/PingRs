#[path = "Editor.rs"]
mod editor;

use editor::Editor;
fn main() -> iced::Result{
    iced::application("Super simple Text Editor", Editor::update, editor::view).theme(|_s| iced::Theme::Dark).run()?;
    Ok(())
}