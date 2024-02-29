use std::{any::Any, cell::RefCell};

use crate::{crossterm::event::Event, pane::Pane, AsAny, EventAction, Renderer, Result};

pub struct Snapshot<R: Renderer> {
    pub init: R,
    pub before: R,
    /// `after` is a `RefCell` containing the renderer state after an event has been processed.
    /// The use of `RefCell` allows for dynamic, mutable borrowing of the contained value,
    /// enabling modifications to the renderer state at runtime while adhering to Rust's borrowing rules.
    /// This is crucial for allowing safe, mutable access to the renderer state in scenarios
    /// where the borrowing context is not statically known.
    pub after: RefCell<R>,
}

impl<R: Renderer + Clone> Snapshot<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            init: renderer.clone(),
            before: renderer.clone(),
            after: RefCell::new(renderer),
        }
    }
}

impl<R: Clone + Renderer + 'static> Renderer for Snapshot<R> {
    fn make_pane(&self, width: u16) -> Pane {
        self.after.borrow().make_pane(width)
    }

    fn handle_event(&mut self, event: &Event) -> Result<EventAction> {
        self.before = self.after.borrow().clone();
        self.after.borrow_mut().handle_event(event)
    }

    fn postrun(&mut self) {
        self.after.borrow_mut().postrun();
        self.init = self.after.borrow().clone();
        self.before = self.after.borrow().clone();
    }
}

impl<R: Renderer + 'static> AsAny for Snapshot<R> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
