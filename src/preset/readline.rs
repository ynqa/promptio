use anyhow::Result;

use crate::{
    components::{Component, Mode, State, Text, TextBuilder, TextEditor, TextEditorBuilder},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    suggest::Suggest,
    theme::readline::Theme,
    validate::Validator,
    Prompt, PromptBuilder,
};

pub struct Readline {
    title: TextBuilder,
    text_editor: TextEditorBuilder,
    validator: Option<Validator<str>>,
    error_message: TextBuilder,
}

impl Default for Readline {
    fn default() -> Self {
        Self {
            title: Default::default(),
            text_editor: Default::default(),
            validator: Default::default(),
            error_message: Default::default(),
        }
        .theme(Theme::default())
    }
}

impl Readline {
    pub fn theme(mut self, theme: Theme) -> Self {
        self.title = self.title.style(theme.title_style);
        self.text_editor = self
            .text_editor
            .label(theme.label)
            .label_style(theme.label_style)
            .style(theme.text_style)
            .cursor_style(theme.cursor_style);
        self.error_message = self.error_message.style(theme.error_message_style);
        self
    }

    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title = self.title.text(text);
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.text_editor = self.text_editor.mode(mode);
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.text_editor = self.text_editor.lines(lines);
        self
    }

    pub fn suggest(mut self, suggest: Suggest) -> Self {
        self.text_editor = self.text_editor.suggest(suggest);
        self
    }

    pub fn disable_history(mut self) -> Self {
        self.text_editor = self.text_editor.disable_history();
        self
    }

    pub fn validator<V, F>(mut self, validator: V, error_message_configure: F) -> Self
    where
        V: Fn(&str) -> bool + 'static,
        F: Fn(&str) -> String + 'static,
    {
        self.validator = Some(Validator::new(validator, error_message_configure));
        self
    }

    pub fn prompt(self) -> Result<Prompt> {
        let validator = self.validator;

        PromptBuilder::new(vec![
            self.title.build_state()?,
            self.text_editor.build_state()?,
            self.error_message.build_state()?,
        ])
        .evaluate(
            move |event: &Event, components: &Vec<Box<dyn Component + 'static>>| -> Result<bool> {
                let text: String = components[1]
                    .as_any()
                    .downcast_ref::<State<TextEditor>>()
                    .unwrap()
                    .after
                    .borrow()
                    .textbuffer
                    .content_without_cursor();

                let hint_state = components[2]
                    .as_any()
                    .downcast_ref::<State<Text>>()
                    .unwrap();

                let ret = match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => match &validator {
                        Some(validator) => {
                            let ret = validator.validate(&text);
                            if !validator.validate(&text) {
                                hint_state.after.borrow_mut().text = validator.error_message(&text);
                            }
                            ret
                        }
                        None => true,
                    },
                    _ => true,
                };
                if ret {
                    *hint_state.after.borrow_mut() = hint_state.init.clone();
                }
                Ok(ret)
            },
        )
        .build()
    }
}
