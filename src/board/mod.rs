mod gen;
mod tile;

pub use tile::Tile;

pub struct Board<T: gen::GenTile, const SIZE: usize> {
	tiles: [[T; SIZE]; SIZE],
}

impl<T: gen::GenTile, const SIZE: usize> Board<T, SIZE> {
	pub fn generate() -> Self {
		gen::Gen::new().fully_collapse()
	}

	pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
		self.tiles.iter().map(|row| row.as_slice())
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}
