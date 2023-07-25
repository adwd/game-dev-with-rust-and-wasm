use web_sys::HtmlImageElement;

use crate::engine::*;

use super::{red_hat_boy_state_machine::*, red_hat_boy_states::*};

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    pub fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet,
            image,
        }
    }

    pub fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }

    pub fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    pub fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    pub fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        );

        let sprite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &Rect {
                x: self.state_machine.context().position.x.into(),
                y: self.state_machine.context().position.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
        );
    }
}
