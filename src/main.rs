use qt::Qt;
use qt::Widgets::QApplication;

fn main() {
	let app = QApplication::new();

	let window = Qt::Widgets::QMainWindow::new();

	window.setWindowTitle(":3");

	window.show();

	app.exec();
}
