use iced::widget::{Column, button, column, text};

#[derive(Default)]
struct Counter {
	value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
	Increment,
	Decrement,
	Reset,
}

impl Counter {
	fn update(&mut self, message: Message) {
		match message {
			Message::Increment => {
				self.value += 1;
			}
			Message::Decrement => {
				self.value -= 1;
			}
			Message::Reset => {
				self.value = 0;
			}
		}
	}

	fn view(&self) -> Column<'_, Message> {
		column![
			button("+").on_press(Message::Increment),
			text(self.value),
			button("-").on_press(Message::Decrement),
			button("reset").on_press(Message::Reset),
		]
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	iced::run("Counter", Counter::update, Counter::view)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use crate::Counter;

	#[test]
	fn it_counts_properly() {
		let mut counter = Counter::default();
		counter.update(crate::Message::Increment);
		counter.update(crate::Message::Increment);
		counter.update(crate::Message::Decrement);
		assert_eq!(counter.value, 1);
	}

	#[test]
	fn it_resets() {
		let mut counter = Counter::default();
		counter.update(crate::Message::Increment);
		counter.update(crate::Message::Increment);
		counter.update(crate::Message::Decrement);
		counter.update(crate::Message::Reset);
		assert_eq!(counter.value, 0);
	}
}
