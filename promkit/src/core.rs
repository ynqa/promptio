pub mod checkbox;
mod cursor;
pub use cursor::Cursor;
pub mod jsonstream;
pub mod listbox;
pub mod snapshot;
pub mod text;
pub mod text_editor;
pub mod tree;

use crate::Pane;

pub trait PaneFactory {
    /// Creates pane with the given width.
    fn create_pane(&self, width: u16, height: u16) -> Pane;
}

/// Determines whether an item at the specified index should be displayed.
/// First fills the screen with items from the cursor position forward.
/// If there's remaining space, fills it by going backward from the cursor position.
///
/// # Arguments
/// * `index` - The index of the item to check
/// * `position` - The current cursor position
/// * `total_items` - The total number of items in the list
/// * `height` - The height of the display area
///
/// # Returns
/// `true` if the item should be displayed, `false` otherwise
pub fn should_display_item(index: usize, position: usize, total_items: usize, height: usize) -> bool {
    // First, display items from position forward
    if index >= position && index <= position + height {
        return true;
    }

    // Calculate how many items we can display from position forward
    let displayed_items = (position + height).min(total_items) - position;

    // If we have remaining space, fill it with items before position
    if displayed_items < height {
        let remaining = height - displayed_items;
        let start_idx = if position > remaining {
            position - remaining
        } else {
            0
        };
        return index >= start_idx && index < position;
    }

    false
}
