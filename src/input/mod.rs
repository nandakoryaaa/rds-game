use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive (Copy, Clone, PartialEq)]
pub enum InputEvent {
	Empty,
	MoveLeft,
	MoveRight,
	Stop,
	Shoot,
	StopShoot,
	ItemPrev,
	ItemNext,
	ItemSelect,
	Continue,
	Quit
}

type EventTransform = fn(evt: &Event, input: &mut Input);

fn evt_transform_any_key(evt: &Event, input: &mut Input)
{
	input.evt = InputEvent::Empty;

	match evt {
		Event::KeyDown { .. } => {
			input.evt = InputEvent::Continue;
		},
		_ => {}
	}
}

fn evt_transform_game(evt: &Event, input: &mut Input)
{
	input.evt = InputEvent::Empty;

	match *evt {
		Event::KeyDown { keycode: Some(k), .. } => {
			if k == Keycode::Left {
				input.evt = InputEvent::MoveLeft;
			} else if k == Keycode::Right {
				input.evt = InputEvent::MoveRight;
			} else if k == Keycode::Space {
				input.evt = InputEvent::Shoot;
			} else if k == Keycode::Escape {
				input.evt = InputEvent::Quit;
			}
		},
		Event::KeyUp { keycode: Some(k), .. } => {
			if k == Keycode::Left || k == Keycode::Right {
				input.evt = InputEvent::Stop;
			} else if k == Keycode::Space {
				input.evt = InputEvent::StopShoot;
			}
		},
		_ => {}
	}
}

pub struct Input {
	evt: InputEvent,
	evt_transform: EventTransform
}

pub struct InputBuilder;

impl InputBuilder
{
	pub fn any_key() -> Input
	{
		Input::new(evt_transform_any_key)
	}

	pub fn game() -> Input
	{
		Input::new(evt_transform_game)
	}
}

impl Input
{
	pub fn new(evt_transform: EventTransform) -> Self
	{
		Self {
			evt: InputEvent::Empty,
			evt_transform: evt_transform
		}
	}

	pub fn get_event(&self) -> InputEvent
	{
		self.evt
	}

	pub fn set_event(&mut self, evt: &Event) -> bool
	{
		(self.evt_transform)(evt, self);
		return self.evt != InputEvent::Empty;
	}
}
