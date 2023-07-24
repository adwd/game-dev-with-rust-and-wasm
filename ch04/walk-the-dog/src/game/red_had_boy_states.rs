use crate::engine::Point;

const FLOOR: i16 = 475;
const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;

const RUNNING_SPEED: i16 = 3;

const SLIDING_FRAMES: u8 = 14;
const SLIDING_FRAME_NAME: &str = "Slide";

#[derive(Clone, Copy)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

#[derive(Clone, Copy)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl RedHatBoyContext {
    pub fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self
    }

    fn reset_frame(mut self) -> Self {
        self.frame = 0;
        self
    }

    fn run_right(mut self) -> Self {
        self.velocity.x = RUNNING_SPEED;
        self
    }

    fn run_left(mut self) -> Self {
        self.velocity.x = -RUNNING_SPEED;
        self
    }
}

#[derive(Clone, Copy)]
pub struct Idle;
#[derive(Clone, Copy)]
pub struct Running;
#[derive(Clone, Copy)]
pub struct RunningBack;
#[derive(Clone, Copy)]
pub struct Sliding;
#[derive(Clone, Copy)]
pub struct Update;

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle,
        }
    }

    pub fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running,
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(IDLE_FRAMES);
        self
    }
}

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub fn run_back(self) -> RedHatBoyState<RunningBack> {
        RedHatBoyState {
            context: self.context.reset_frame().run_left(),
            _state: RunningBack,
        }
    }

    pub fn slide(self) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Sliding,
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(RUNNING_FRAMES);
        self
    }
}

impl RedHatBoyState<RunningBack> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running,
        }
    }
}

impl RedHatBoyState<Sliding> {
    pub fn frame_name(&self) -> &str {
        SLIDING_FRAME_NAME
    }

    pub fn update(mut self) -> SlidingEndState {
        self.context = self.context.update(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Complete(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    pub fn stand(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running,
        }
    }
}

pub enum SlidingEndState {
    Complete(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}
