pub struct ListEntry<T> {
	pub prev: usize,
	pub next: usize,
	pub payload: T
}

pub struct Pantry<T> {
	pub entries: Vec<ListEntry<T>>,
	pub capacity: usize,
	pub used_cnt: usize,
	pub free_cnt: usize,
	pub used_first: usize,
	pub used_last: usize,
	pub free_first: usize,
	pub free_last: usize
}

impl<T> Pantry<T> {
	pub fn create(capacity: usize) -> Self {
		Pantry {
			entries: Vec::with_capacity(capacity),
			capacity: capacity,
			used_cnt: 0,
			free_cnt: 0,
			used_first: 0,
			used_last: 0,
			free_first: 0,
			free_last: 0,
		}
	}

	pub fn get_mut(&mut self, index: usize) -> &mut T {
		&mut self.entries[index].payload
	}

	pub fn first_index(&self) -> usize {
		return self.used_first;
	}

	pub fn last_index(&self) -> usize {
		return self.used_last;
	}

	pub fn next_index(&self, index: usize) -> usize {
		return self.entries[index].next;
	}

	pub fn is_last_index(&self, index: usize) -> bool {
		return index == self.used_last;
	}

	pub fn len(&self) -> usize {
		return self.used_cnt;
	}

	pub fn alloc(&mut self, p: T) -> usize {
		let mut index: usize = self.entries.len();

		let entry = ListEntry {
			prev: self.used_last,
			next: index,
			payload: p
		};

		if self.free_cnt == 0 {
			if index == self.capacity {
				return index;
			}
			self.entries.push(entry);
		} else {
			index = self.free_first;
			self.free_cnt -= 1;
			self.free_first = self.entries[index].next;
			self.entries[index] = entry;
		}
		if self.used_cnt == 0 {
			self.used_first = index;
		} else {
			self.entries[self.used_last].next = index;
		}
		self.used_last = index;
		self.used_cnt += 1;

		index
	}

	pub fn update(&mut self, index: usize, p: T) {
		self.entries[index].payload = p;
	}

	pub fn get(&self, index: usize) -> &T {
		&self.entries[index].payload
	}
		
	pub fn free(&mut self, index: usize) {
		if index >= self.entries.len() || self.used_cnt == 0 {
			panic!(
				"free({}): invalid index {} len {} used_cnt {}",
				std::any::type_name::<T>(), index, self.entries.len(), self.used_cnt
			);
		}
		let prev = self.entries[index].prev;
		let next = self.entries[index].next;

		if index == self.used_first {
			self.used_first = next;
		} else {
			self.entries[prev].next = next;
		}
		if index == self.used_last {
			self.used_last = prev;
		} else {
			self.entries[next].prev = prev;
		}
		if self.free_cnt == 0 {
			self.free_first = index;
			self.free_last = index;
		} else {
			self.entries[self.free_last].next = index;
			self.free_last = index;
		}
		self.used_cnt -= 1;
		self.free_cnt += 1;
	}
}
