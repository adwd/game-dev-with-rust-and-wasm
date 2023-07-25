use crate::engine::Point;

const FLOOR: i16 = 475;
const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const JUMPING_FRAMES: u8 = 35;
const SLIDING_FRAMES: u8 = 14;
const RUNNING_SPEED: i16 = 3;
const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";
const SLIDING_FRAME_NAME: &str = "Slide";
const JUMPING_FRAME_NAME: &str = "Jump";
const JUMP_SPEED: i16 = -25;
const GRAVITY: i16 = 1;

#[derive(Copy, Clone)]
pub(crate) struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    pub(crate) fn context(&self) -> &RedHatBoyContext {
        &self.context
    }

    fn update_context(&mut self, frames: u8) {
        self.context = self.context.update(frames);
    }
}

#[derive(Copy, Clone)]
pub struct Idle;

impl RedHatBoyState<Idle> {
    pub(crate) fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub(crate) fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub(crate) fn update(mut self) -> RedHatBoyState<Idle> {
        self.update_context(IDLE_FRAMES);
        self
    }

    pub(crate) fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }
}

#[derive(Copy, Clone)]
pub struct Running;

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub fn update(mut self) -> RedHatBoyState<Running> {
        self.update_context(RUNNING_FRAMES);
        self
    }

    pub fn jump(self) -> RedHatBoyState<Jumping> {
        RedHatBoyState {
            context: self.context.reset_frame().set_vertical_velocity(JUMP_SPEED),
            _state: Jumping {},
        }
    }

    pub fn slide(self) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Sliding {},
        }
    }
}

#[derive(Copy, Clone)]
pub struct Jumping;

pub(crate) enum JumpingEndState {
    Jumping(RedHatBoyState<Jumping>),
    Landing(RedHatBoyState<Running>),
}

impl RedHatBoyState<Jumping> {
    pub fn frame_name(&self) -> &str {
        JUMPING_FRAME_NAME
    }

    pub fn update(mut self) -> JumpingEndState {
        self.update_context(JUMPING_FRAMES);

        if self.context.position.y >= FLOOR {
            JumpingEndState::Landing(self.land())
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    pub fn land(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running {},
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sliding;

pub(crate) enum SlidingEndState {
    Sliding(RedHatBoyState<Sliding>),
    Running(RedHatBoyState<Running>),
}

impl RedHatBoyState<Sliding> {
    pub(crate) fn frame_name(&self) -> &str {
        SLIDING_FRAME_NAME
    }

    pub(crate) fn update(mut self) -> SlidingEndState {
        self.update_context(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Running(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    pub(crate) fn stand(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running {},
        }
    }
}

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    velocity: Point,
}

impl RedHatBoyContext {
    pub fn update(mut self, frame_count: u8) -> Self {
        self.velocity.y += GRAVITY;

        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > FLOOR {
            self.position.y = FLOOR;
        }

        self
    }

    fn reset_frame(mut self) -> Self {
        self.frame = 0;
        self
    }

    fn set_vertical_velocity(mut self, y: i16) -> Self {
        self.velocity.y = y;
        self
    }

    fn run_right(mut self) -> Self {
        self.velocity.x += RUNNING_SPEED;
        self
    }
}
