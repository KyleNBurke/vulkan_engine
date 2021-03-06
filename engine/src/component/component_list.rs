use crate::entity_manager::MAX_ENTITY_COUNT;

pub struct ComponentList<T> {
	components: Vec<(usize, T)>,
	entity_to_index_map: [Option<usize>; MAX_ENTITY_COUNT]
}

impl<T> ComponentList<T> {
	pub fn new() -> Self {
		Self {
			components: Vec::new(),
			entity_to_index_map: [None; MAX_ENTITY_COUNT]
		}
	}

	pub fn add(&mut self, entity: usize, component: T) {
		assert!(self.entity_to_index_map[entity].is_none(), "Cannot add component to entity {} because it already has this component type", entity);
		self.components.push((entity, component));
		let index = self.components.len() - 1;
		self.entity_to_index_map[entity] = Some(index);
	}

	pub fn remove(&mut self, entity: usize) {
		let index_option = self.entity_to_index_map[entity];
		assert!(index_option.is_some(), "Cannot remove component from entity {} because it does not have this component type", entity);
		let index = index_option.unwrap();
		self.entity_to_index_map[entity] = None;
		self.components.swap_remove(index);
		let (swapped_entity, _) = self.components[index];
		self.entity_to_index_map[swapped_entity] = Some(index);
	}

	pub fn borrow(&self, entity: usize) -> &T {
		let index = self.entity_to_index_map[entity];
		assert!(index.is_some(), "Cannot borrow component from entity {} because it does not have this component type", entity);
		&self.components[index.unwrap()].1
	}

	pub fn borrow_mut(&mut self, entity: usize) -> &mut T {
		let index = self.entity_to_index_map[entity];
		assert!(index.is_some(), "Cannot mutably borrow component from entity {} because it does not have this component type", entity);
		&mut self.components[index.unwrap()].1
	}

	pub fn try_borrow(&self, entity: usize) -> Option<&T> {
		let index = self.entity_to_index_map[entity]?;
		Some(&self.components[index].1)
	}

	pub fn try_borrow_mut(&mut self, entity: usize) -> Option<&mut T> {
		let index = self.entity_to_index_map[entity]?;
		Some(&mut self.components[index].1)
	}

	pub fn iter(&self) -> impl Iterator<Item = &(usize, T)> {
		self.components.iter()
	}
}