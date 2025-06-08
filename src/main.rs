/*
This file is from https://git.nia.land/nia/lyri
Copyright (C) 2025 Nia

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Lyri is an application to display the current lyrics of whatever song you are listening to and its translation
//! Its main use is for those interested in language learning, especially if you have a particular favorite song in your target language
//!
//! In terms of the program flow, it starts by using mpris to get the current song url+name (the same method that your desktop environment will use)
// TODO: write out rest of program flow as it gets added

pub mod media_interface;

use iced::widget::{container, text};
use iced::{Length, alignment};

fn main() -> iced::Result {
	let Some(mut media) = media_interface::MediaInterface::new() else {
		panic!("Error while creating media interface")
	};
	media.find_media();
	iced::run("Lyri", LyriApp::update, LyriApp::view)
}

type Message = ();

#[derive(Default)]
struct LyriApp;

impl LyriApp {
	fn update(&mut self, _message: Message) {}

	fn view(&self) -> iced::Element<Message> {
		container(text("Hello World"))
			.height(Length::Fill)
			.width(Length::Fill)
			.align_x(alignment::Horizontal::Center)
			.align_y(alignment::Vertical::Center)
			.into()
	}
}
