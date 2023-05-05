use embedded_graphics::primitives::Rectangle;
use embedded_layout::{object_chain::ChainElement, prelude::*};

use crate::{MenuEvent, MenuItemTrait};

/// Menu-related extensions for object chain elements
pub trait MenuExt<R: Copy>: ChainElement {
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> MenuEvent<R>;
}

impl<I, R: Copy> MenuExt<R> for Chain<I>
where
    R: Copy,
    I: MenuItemTrait<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        debug_assert!(nth == 0);
        self.object.bounds()
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        debug_assert!(nth == 0);
        self.object.interact()
    }

    fn title_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.title()
    }

    fn details_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.details()
    }
}

impl<I, LE, R> MenuExt<R> for Link<I, LE>
where
    R: Copy,
    I: MenuItemTrait<R>,
    LE: MenuExt<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        if nth == 0 {
            self.object.bounds()
        } else {
            self.parent.bounds_of(nth - 1)
        }
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        if nth == 0 {
            self.object.interact()
        } else {
            self.parent.interact_with(nth - 1)
        }
    }

    fn title_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.title()
        } else {
            self.parent.title_of(nth - 1)
        }
    }

    fn details_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.details()
        } else {
            self.parent.details_of(nth - 1)
        }
    }
}