//! This module provides a collection of preset components
//! for building interactive command-line interfaces.
//! It includes a variety of common UI elements
//! such as checkboxes, text input fields, selection lists, and trees.
//!
//! Each component is designed to be easy to use
//! and integrate into CLI applications, providing a quick way
//! to add interactivity and collect input from users.
//!
//! ## Components
//!
//! - `Checkbox`: For creating and managing a list of selectable options.
//! - `Readline`: For reading a line of text from the user.
//! Includes variations like `Confirm` for yes/no questions,
//! and `Password` for masked input.
//! - `Listbox`: For allowing the user to select from a list of options.
//! - `QuerySelector`: Similar to `Listbox`,
//! but with support for filtering options through user input.
//! - `Tree`: For displaying and navigating hierarchical data.
//!
//! These components can be used individually
//! or combined to create complex user interfaces in terminal applications.

mod checkbox;
pub use checkbox::Checkbox;
mod readline;
pub use readline::{Confirm, Password, Readline};
mod listbox;
pub use listbox::Listbox;
mod query_selector;
pub use query_selector::QuerySelector;
mod tree;
pub use tree::Tree;
mod json;
pub use json::Json;
