use super::{Board, Direction};
use enumset::{EnumSet, EnumSetType};
use graphics::types::Vec2d;
use rand::seq::{IteratorRandom as _, SliceRandom as _};
use std::collections::VecDeque;

pub trait GenTile: EnumSetType + Clone + Copy + Sized {
	fn character_repr(self) -> char;
	/// direction is from `self` to `other`. In other words, `other` is in `direction` relative to `self`.
	fn is_valid_neighbor(self, other: Self, direction: Direction) -> bool;
}

pub struct Gen<T: GenTile, const SIZE: usize>([[EnumSet<T>; SIZE]; SIZE]);

type Position = Vec2d<usize>;

impl<T: GenTile, const SIZE: usize> Gen<T, SIZE> {
	pub fn new() -> Self {
		Self([[EnumSet::<T>::all(); SIZE]; SIZE])
	}

	fn get(&self, pos: Position) -> EnumSet<T> {
		self.0[pos[1]][pos[0]]
	}
	fn get_mut(&mut self, pos: Position) -> &mut EnumSet<T> {
		&mut self.0[pos[1]][pos[0]]
	}

	fn find_to_collapse(&self) -> Option<Position> {
		let mut min_arity = usize::MAX;
		let mut positions_with_min_arity: Vec<Position> = Vec::new();
		let mut max_arity = usize::MIN;

		for (y, row) in self.0.iter().enumerate() {
			for (x, item) in row.iter().enumerate() {
				assert!(!item.is_empty());
				let item_arity = item.len();
				max_arity = max_arity.max(item_arity);

				// for the minimum arity, don't include already-collapsed tiles
				if item_arity > 1 {
					if item_arity < min_arity {
						positions_with_min_arity.clear();
					}
					if item_arity <= min_arity {
						positions_with_min_arity.push([x, y]);
					}

					min_arity = min_arity.min(item_arity);
				}
			}
		}

		if max_arity == 1 {
			// at this point, all of the tiles have collapsed, so we can't find one to collapse
			return None;
		}

		Some(
			*positions_with_min_arity
				.choose(&mut rand::thread_rng())
				.unwrap(),
		)
	}

	fn neighbors(pos: Position) -> impl IntoIterator<Item = (Direction, Option<Position>)> {
		let left = pos[0].checked_sub(1).map(|x| [x, pos[1]]);
		let above = pos[1].checked_sub(1).map(|y| [pos[0], y]);
		let right = if pos[0] + 1 >= SIZE {
			None
		} else {
			Some([pos[0] + 1, pos[1]])
		};
		let below = if pos[1] + 1 >= SIZE {
			None
		} else {
			Some([pos[0], pos[1] + 1])
		};

		[
			(Direction::Left, left),
			(Direction::Up, above),
			(Direction::Right, right),
			(Direction::Down, below),
		]
	}

	fn collapse_single(&mut self, pos: Position) -> T {
		let to_collapse = self.get_mut(pos);
		let tile = to_collapse.iter().choose(&mut rand::thread_rng()).unwrap();
		*to_collapse = EnumSet::only(tile);
		tile
	}

	// General idea of what's going on here:
	// After choosing an initial position to collapse at random,
	// we want to propagate the collapses to all the other tiles.
	// To do this, we use a slightly modified version of BFS.
	// The key difference from BFS is that, rather than keeping track of visited nodes using a set,
	// we use whether a neighbor collapsed as the check for whether to continue processing *its* neighbors.
	fn collapse(&mut self, initial_pos: Position) {
		let mut queue: VecDeque<Position> = [initial_pos].into_iter().collect();

		if cfg!(debug_assertions) {
			eprintln!("Before collapse_single");
			self.debug_repr();
		}
		self.collapse_single(initial_pos);
		if cfg!(debug_assertions) {
			eprintln!("After collapse_single");
			self.debug_repr();
		}

		while let Some(pos) = queue.pop_front() {
			let tile = self.get(pos).iter().next().unwrap();
			if cfg!(debug_assertions) {
				eprintln!("Processing {pos:?} from queue");
				self.debug_repr();
			}
			for (neighbor_direction, neighbor_pos) in Self::neighbors(pos) {
				if let Some(neighbor_pos) = neighbor_pos {
					if cfg!(debug_assertions) {
						eprintln!("Processing {neighbor_pos:?} as neighbor of {pos:?}");
						self.debug_repr();
					}
					let neighbor = self.get_mut(neighbor_pos);
					let old_neighbor_len = neighbor.len();
					*neighbor = neighbor
						.iter()
						.filter(move |&neighbor_possibility| {
							tile.is_valid_neighbor(neighbor_possibility, neighbor_direction)
						})
						.collect();
					if cfg!(debug_assertions) {
						eprintln!("After processing {neighbor_pos:?} as neighbor of {pos:?}");
						self.debug_repr();
					}
					let neighbor = self.get_mut(neighbor_pos);
					assert!(!neighbor.is_empty());
					// if neighbor collapsed (and was not collapsed before)
					if neighbor.len() == 1 && neighbor.len() != old_neighbor_len {
						// ...process it (and its neighbors) again
						queue.push_back(neighbor_pos);
					}
				}
			}
		}
	}

	pub fn fully_collapse(mut self) -> Board<T, SIZE> {
		while let Some(to_collapse_pos) = self.find_to_collapse() {
			self.collapse(to_collapse_pos);
		}
		Board {
			tiles: self
				.0
				.map(|row| row.map(|tile| tile.iter().next().unwrap())),
		}
	}

	fn debug_repr(&self) {
		for row in self.0 {
			for tile in row {
				for possibility in EnumSet::<T>::all().iter() {
					let ch = if tile.contains(possibility) {
						possibility.character_repr()
					} else {
						' '
					};
					eprint!("{ch}");
				}
				eprint!(" ");
			}
			eprintln!();
		}
	}
}
