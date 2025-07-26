mod buttons;

use std::sync::Arc;

use adw::prelude::*;
use backend::platform_song_interface;
use gtk::{gio, glib};

use buttons::{next_button_action, play_button_action, prev_button_action};

const APP_ID: &str = "land.nia.lyri";

fn main() -> glib::ExitCode {
	gio::resources_register_include!("lyri.gresource").expect("Failed to register resources.");

	let app = adw::Application::builder().application_id(APP_ID).build();

	app.connect_activate(build_ui);

	app.run()
}

fn build_ui(app: &adw::Application) {
	let song_retriever = Arc::new(platform_song_interface::get_platform_retriever());

	let builder = gtk::Builder::from_resource("/land/nia/lyri/window.ui");

	let window: gtk::Window = builder.object("main_window").expect("Failed to get `main_window` from UI file");

	window.set_application(Some(app));
	window.set_title(Some("Lyri"));

	let prev_button: gtk::Button = builder.object("prev_button").expect("Missing `prev_button`");
	let play_button: gtk::Button = builder.object("play_button").expect("Missing `play_button`");
	let next_button: gtk::Button = builder.object("next_button").expect("Missing `next_button`");
	// let settings_button: gtk::Button = builder.object("stetings_button").expect("Missing `settings_button`");

	let play_button_icon: gtk::Image = builder.object("play_button_icon").expect("Missing `play_button_icon`");

	let song_retriever1 = song_retriever.clone();
	let song_retriever2 = song_retriever.clone();
	let song_retriever3 = song_retriever.clone();

	prev_button.connect_clicked(move |_| {
		let song_retriever = song_retriever1.clone();
		glib::MainContext::default().spawn_local(async move {
			prev_button_action(song_retriever).await;
		});
	});

	let icon_weak = play_button_icon.downgrade();

	play_button.connect_clicked(move |_| {
		let song_retriever = song_retriever2.clone();
		let icon_weak = icon_weak.clone();
		glib::MainContext::default().spawn_local(async move {
			if let Some(icon) = icon_weak.upgrade() {
				let is_playing = play_button_action(song_retriever).await;

				let icon_path = if is_playing {
					"/land/nia/lyri/assets/pause.png"
				} else {
					"/land/nia/lyri/assets/play.png"
				};

				icon.set_resource(Some(icon_path));
			}
		});
	});

	next_button.connect_clicked(move |_| {
		let song_retriever = song_retriever3.clone();
		glib::MainContext::default().spawn_local(async move {
			next_button_action(song_retriever).await;
		});
	});

	window.present();
}
