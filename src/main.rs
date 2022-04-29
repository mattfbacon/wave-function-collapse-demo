use crate::board::{Board, Tile};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL as OpenGlVersion};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderEvent};
use piston::window::WindowSettings;
use std::sync::{Arc, Mutex};

const OPEN_GL_VERSION: OpenGlVersion = OpenGlVersion::V4_5;

mod board;

const SIZE: usize = 10;

fn main() {
	let mut window: GlutinWindow =
		WindowSettings::new("wave-function-collapse-demo", [(SIZE * 50 + 40) as f64; 2])
			.graphics_api(OPEN_GL_VERSION)
			.exit_on_esc(true)
			.build()
			.unwrap();

	let mut gl = GlGraphics::new(OPEN_GL_VERSION);
	let board: Arc<Mutex<Board<Tile, SIZE>>> = Arc::new(Mutex::new(Board::generate()));

	let mut events = Events::new(EventSettings::new());
	while let Some(event) = events.next(&mut window) {
		event.press(|button| {
			if let Button::Keyboard(Key::N) = button {
				let board = Arc::clone(&board);
				std::thread::spawn(move || {
					let new_board = Board::generate();
					*board.lock().unwrap() = new_board;
				});
			};
		});
		event.render(|args| {
			gl.draw(args.viewport(), |context, gl| {
				draw(&board.lock().unwrap(), context, gl)
			})
		});
	}
}

fn draw(board: &Board<Tile, SIZE>, context: graphics::Context, gl: &mut GlGraphics) {
	use graphics::color as colors;
	use graphics::Transformed as _;

	graphics::clear(colors::WHITE, gl);
	for (row_num, row) in board.iter_rows().enumerate() {
		for (col_num, &tile) in row.iter().enumerate() {
			let transform = context
				.transform
				.trans(col_num as f64 * 50.0 + 20.0, row_num as f64 * 50.0 + 20.0);
			let color = match tile {
				board::Tile::Dirt => [0.395, 0.301, 0.160, 1.0],
				board::Tile::Sky => [0.645, 0.758, 0.828, 1.0],
				board::Tile::Grass => [0.359, 0.555, 0.3635, 1.0],
			};
			graphics::rectangle_from_to(color, [0.0, 0.0], [50.0, 50.0], transform, gl);
		}
	}
}
