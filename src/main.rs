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

use media_interface::MediaInterface;

use std::fs::File;

use iced::{
	Length, Point, Rectangle, Size, Subscription, Theme, mouse,
	widget::{Button, button, center, column, horizontal_space, image, row, vertical_space},
	window::Icon,
};

fn main() -> iced::Result {
	let mut media = MediaInterface::new();

	let mut icon_file = File::open("assets/icon.png").expect("Failed to open icon file");
	let mut icon_data = Vec::new();
	icon_file.read_to_end(&mut icon_data).expect("Failed to read icon file");
	let icon = Some(Icon::from_rgba(icon_data, 32, 32).expect("Invalid icon data"));

	iced::application("Lyri", LyriApp::update, LyriApp::view)
		.subscription(LyriApp::subscription)
		.theme(LyriApp::theme)
		.settings(iced::Settings {
			antialiasing: true,
			default_text_size: 16.0,
			..Default::default()
		})
		.window(iced::window::Settings {
			size: Size::new(400, 200),
			position: iced::window::Position::Default,
			min_size: Some(Size::new(300, 150)),
			level: window::Level::AlwaysOnTop,
			icon,
			decorations: false,
			..iced::window::Settings::default()
		})
		.run()
}

#[derive(Debug, Clone)]
enum Message {
	NextSong,
	PreviousSong,
	TogglePlayback,
	OpenSettings,
	CloseSettings,
	SetTranslationLanguage(String),
	UpdateMedia,
	MouseEntered,
	MouseLeft,
}

struct LyriApp {
	media: MediaInterface,
	translation_language: String,
	settings_open: bool,
	show_controls: bool,
	available_languages: Vec<String>,
}

impl LyriApp {
	/// Creates new LyriApp
	fn new(media: MediaInterface) -> Self {
		Self {
			media,
			translation_language: "English".to_string(),
			settings_open: false,
			show_controls: false,
			available_languages: vec![
				"English".to_string(),
				"Spanish".to_string(),
				"French".to_string(),
				// TODO: when i hook up with the translation part i can add the rest of the languages
			],
		}
	}

	/// Handles messages
	fn update(&mut self, message: Message) {
		match message {
			Message::NextSong => {
				self.media.next();
			}
			Message::PreviousSong => {
				self.media.prev();
			}
			Message::TogglePlayback => {
				if self.media.is_playing() {
					self.media.pause();
				} else {
					self.media.play();
				}
			}
			Message::OpenSettings => {
				self.settings_open = true;
			}
			Message::CloseSettings => {
				self.settings_open = false;
			}
			Message::SetTranslationLanguage(language) => {
				self.translation_language = language;
			}
			Message::UpdateMedia => {
				self.media.update_media();
			}
			Message::MouseEntered => {
				self.show_controls = true;
			}
			Message::MouseLeft => {
				self.show_controls = false;
			}
		}
	}

	/// Displays either main view or settings view if settings is open
	fn view(&self) -> iced::Element<Message> {
		if self.settings_open { self.settings_view() } else { self.main_view() }
	}

	/// Main view with song name+image+lyrics, and buttons if u hover over
	fn main_view(&self) -> iced::Element<Message> {
		// get media info
		self.media.update_media();
		let (song_name, image_url) = self.media.current_song_info();
		let song_name = song_name.unwrap_or("Unknown Song");
		let lyrics = self.media.get_lyrics();
		let (original_lyrics, translated_lyrics) = lyrics
			.map(|(orig, trans)| (orig, trans))
			.unwrap_or(("No lyrics available".to_string(), "".to_string()));

		// song image + name
		let img_path = image_url.map(String::from).unwrap_or_else(|| "assets/music.png".to_string());
		let song_info = column![
			container(
				image::Image::new(image::Handle::from_path(img_path))
					.width(Length::Fixed(80.0))
					.height(Length::Fixed(80.0))
			)
			.width(Length::Fixed(80.0))
			.height(Length::Fixed(80.0)),
			container(text(song_name).size(16))
				.width(Length::Fixed(100.0))
				.align_x(iced::alignment::Horizontal::Center)
		]
		.spacing(5)
		.width(Length::Fixed(100.0));

		// lyrics
		let lyrics_display = column![text(original_lyrics).size(16), text(translated_lyrics).size(14)]
			.spacing(10)
			.width(Length::Fill);

		// control buttons (only shown on hover)
		let controls = if self.show_controls {
			row![
				button(image::Image::new(image::Handle::from_path("assets/prev.png")).width(Length::Fixed(24.0)))
					.on_press(Message::PreviousSong)
					.padding(5),
				button(
					image::Image::new(image::Handle::from_path(if self.media.is_playing() {
						"assets/pause.png"
					} else {
						"assets/play.png"
					}))
					.width(Length::Fixed(24.0))
				)
				.on_press(Message::TogglePlayback)
				.padding(5),
				button(image::Image::new(image::Handle::from_path("assets/next.png")).width(Length::Fixed(24.0)))
					.on_press(Message::NextSong)
					.padding(5),
				horizontal_space(),
				button(image::Image::new(image::Handle::from_path("assets/settings.png")).width(Length::Fixed(24.0)))
					.on_press(Message::OpenSettings)
					.padding(5)
			]
			.spacing(5)
			.width(Length::Fill)
			.padding(5)
		} else {
			row![].height(Length::Fixed(34.0))
		};

		// main layout
		let content = column![row![song_info, lyrics_display].spacing(20).padding(10).width(Length::Fill), controls]
			.width(Length::Fill)
			.height(Length::Fill);

		container(content)
			.width(Length::Fill)
			.height(Length::Fill)
			.padding(10)
			.style(container::rounded_box)
			.on_mouse_enter(Message::MouseEntered)
			.on_mouse_leave(Message::MouseLeft)
			.into()
	}

	/// View the settings
	fn settings_view(&self) -> iced::Element<Message> {
		// simple settings to choose language
		let languages = self
			.available_languages
			.iter()
			.map(|lang| {
				let is_selected = &self.translation_language == lang;

				let row_content = row![
					text(lang).size(16),
					horizontal_space(),
					if is_selected { text("✓").size(16) } else { text("").size(16) }
				]
				.spacing(10)
				.padding(5);

				button(row_content)
					.width(Length::Fill)
					.on_press(Message::SetTranslationLanguage(lang.clone()))
					.into()
			})
			.collect::<Vec<_>>();

		let language_list = column(languages).spacing(5).width(Length::Fill);

		let content = column![
			text("Settings").size(24),
			vertical_space(),
			text("Translation Language:").size(16),
			vertical_space(),
			language_list,
			vertical_space(),
			button(
				text("Close")
					.horizontal_alignment(iced::alignment::Horizontal::Center)
					.width(Length::Fill)
			)
			.on_press(Message::CloseSettings)
			.width(Length::Fill)
			.padding(10)
		]
		.spacing(5)
		.padding(20)
		.max_width(350);

		center(container(content).width(Length::Fill).style(container::rounded_box))
			.width(Length::Fill)
			.height(Length::Fill)
			.into()
	}

	/// Send periodic update media requests
	fn subscription(&self) -> Subscription<Message> {
		Subscription::batch([
			iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::UpdateMedia),
			mouse::area(Rectangle::new(Point::ORIGIN, Size::new(400.0, 200.0))).map(|event| match event {
				mouse::Event::Enter => Message::MouseEnter,
				mouse::Event::Leave => Message::MouseLeave,
				_ => Message::UpdateMedia, // dont need other mouse events
			}),
		])
	}

	/// Always dark theme cause i like having eyes
	fn theme(&self) -> Theme {
		Theme::Dark
	}
}
