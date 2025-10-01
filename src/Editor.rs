use iced::{
    widget::{
        button, column, row, text ,text_input, text_editor
    }, Element, Renderer, Task, Theme
};
use std::fs;

#[derive(Debug, Default)]
pub struct Editor{
    path: String,
    file_content: text_editor::Content,
    new_file_content: String,
    statue: String,
}

#[derive(Debug, Clone)]
pub enum Message{
    Save,
    FilePathInput(String),
    SubmitFile,
    Editing(text_editor::Action),
}

impl Editor{
    fn get_file(path: &String) -> Result<text_editor::Content, std::io::Error>{
        let content:String = fs::read_to_string(path)?;

        Ok(text_editor::Content::with_text(&content.to_owned()))
    }

    pub fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Save => Editor::save_file(self, true).into(),

            Message::FilePathInput(path) => (self.path = path).into(),

            Message::SubmitFile => {
                match Editor::get_file(&self.path) {
                    Ok(content) => {
                        self.file_content = content;
                        self.statue = format!("{} loaded", self.path);
                    }
                    Err(_) => {
                        Editor::save_file(self, false);
                        self.file_content = Editor::get_file(&self.path).unwrap();
                        self.statue = format!("Failed to load file. New file created at '{}'", self.path);
                    }
                }
            }.into(),

            Message::Editing(action) => {
                self.file_content.perform(action);
                self.new_file_content = self.file_content.text().to_owned();
            }.into()
        }
    }

    fn save_file(&mut self, condition: bool){
        if condition == false {
            fs::write(&self.path,  "Write something now!").unwrap();
        }else{
            fs::write(&self.path, &self.new_file_content).unwrap();
            self.statue = "Saved!".to_string();
        }
    }
}

pub fn view(state: &Editor) -> Element<'_, Message>{
    column![
        row![
            text_input::<Message, Theme, Renderer>("type the path to load", &state.path)
            .on_input(Message::FilePathInput),
            button(text("Submit")).on_press(Message::SubmitFile)
        ],
        text(&state.statue),
        button(text("Save")).on_press(Message::Save),
        text_editor(&state.file_content)
        .on_action(Message::Editing),
    ].into() 
}
