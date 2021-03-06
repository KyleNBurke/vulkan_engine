use ash::{vk, version::DeviceV1_0};
use crate::vulkan::Context;

pub struct Buffer {
	pub handle: vk::Buffer,
	pub memory: vk::DeviceMemory,
	usage: vk::BufferUsageFlags,
	properties: vk::MemoryPropertyFlags,
	pub capacity: vk::DeviceSize
}

impl Buffer {
	pub fn new(context: &Context, capacity: vk::DeviceSize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Self {
		let (handle, memory) = Self::allocate(context, capacity, usage, properties);

		Self {
			handle,
			memory,
			usage,
			properties,
			capacity
		}
	}

	pub fn null(usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Self {
		Self {
			handle: vk::Buffer::null(),
			memory: vk::DeviceMemory::null(),
			usage,
			properties,
			capacity: 0
		}
	}

	pub fn reallocate(&mut self, context: &Context, capacity: vk::DeviceSize) {
		unsafe {
			context.logical_device.free_memory(self.memory, None);
			context.logical_device.destroy_buffer(self.handle, None);
		}

		let (handle, memory) = Self::allocate(context, capacity, self.usage, self.properties);

		self.handle = handle;
		self.memory = memory;
		self.capacity = capacity;
	}

	fn allocate(
		context: &Context,
		capacity: vk::DeviceSize,
		usage: vk::BufferUsageFlags,
		properties: vk::MemoryPropertyFlags) -> (vk::Buffer, vk::DeviceMemory)
	{
		let create_info = vk::BufferCreateInfo::builder()
			.size(capacity)
			.usage(usage)
			.sharing_mode(vk::SharingMode::EXCLUSIVE);
		
		let handle = unsafe { context.logical_device.create_buffer(&create_info, None).unwrap() };
		let memory_requirements = unsafe { context.logical_device.get_buffer_memory_requirements(handle) };
		let memory_type_index = context.physical_device.find_memory_type_index(memory_requirements.memory_type_bits, properties);

		let allocate_info = vk::MemoryAllocateInfo::builder()
			.allocation_size(memory_requirements.size)
			.memory_type_index(memory_type_index as u32);
	
		let memory = unsafe { context.logical_device.allocate_memory(&allocate_info, None).unwrap() };
		unsafe { context.logical_device.bind_buffer_memory(handle, memory, 0).unwrap() };

		(handle, memory)
	}

	pub fn drop(&self, logical_device: &ash::Device) {
		unsafe {
			logical_device.free_memory(self.memory, None);
			logical_device.destroy_buffer(self.handle, None);
		}
	}
}