use super::Direction;

#[derive(enumset::EnumSetType)]
#[enumset(repr = "u8")]
#[repr(u8)]
pub enum Tile {
	Dirt,
	Grass,
	Sky,
}

impl super::gen::GenTile for Tile {
	fn is_valid_neighbor(self, other: Self, direction: Direction) -> bool {
		match (self, other) {
			// dirt can always neighbor dirt
			(Self::Dirt, Self::Dirt) => true,
			// sky can always neighbor sky
			(Self::Sky, Self::Sky) => true,
			// grass can only neighbor grass adjacently
			(Self::Grass, Self::Grass) => direction != Direction::Up && direction != Direction::Down,
			// dirt can only neighbor sky adjacently
			(Self::Dirt, Self::Sky) | (Self::Sky, Self::Dirt) => {
				direction != Direction::Up && direction != Direction::Down
			}
			// sky can't be below grass
			(Self::Grass, Self::Sky) => direction != Direction::Down,
			(Self::Sky, Self::Grass) => direction != Direction::Up,
			// dirt can't be above grass
			(Self::Grass, Self::Dirt) => direction != Direction::Up,
			(Self::Dirt, Self::Grass) => direction != Direction::Down,
		}
	}
	fn character_repr(self) -> char {
		match self {
			Self::Dirt => 'D',
			Self::Sky => 'S',
			Self::Grass => 'G',
		}
	}
}
