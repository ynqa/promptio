use crate::{
    crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseEvent,
        MouseEventKind,
    },
    preset, PromptSignal,
};

pub type Keymap = fn(
    event: &Event,
    renderer: &mut preset::text::render::Renderer,
) -> anyhow::Result<PromptSignal>;

/// Default key bindings for the text.
///
/// | Key                    | Action
/// | :--------------------- | :-------------------------------------------
/// | <kbd>Enter</kbd>       | Exit the text
/// | <kbd>Ctrl + C</kbd>    | Interrupt the current operation
/// | <kbd>↑</kbd>           | Move the selection up
/// | <kbd>↓</kbd>           | Move the selection down
pub fn default(
    event: &Event,
    renderer: &mut preset::text::render::Renderer,
) -> anyhow::Result<PromptSignal> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Ok(PromptSignal::Quit),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Err(anyhow::anyhow!("ctrl+c")),

        // Move cursor.
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: _,
            row: _,
            modifiers: KeyModifiers::NONE,
        }) => {
            renderer.text_state.text.backward();
        }

        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: _,
            row: _,
            modifiers: KeyModifiers::NONE,
        }) => {
            renderer.text_state.text.forward();
        }

        _ => (),
    }
    Ok(PromptSignal::Continue)
}
