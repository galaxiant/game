use crate::models::resource_models::resource_type::ResourceType;

pub struct Resource {
	pub r#type: ResourceType,
	pub name: &'static str,
	pub plural: &'static str,
	pub description: &'static str,
}

impl Resource {
	pub const WOLT: Resource = Resource {
		r#type: ResourceType::Basic,
		name: "Wolt",
		plural: "Wolts",
		description: "\"Wolts\" are a measure of energy that can also be used as currency. The \"W\" instead of a \"V\" is intentional, as it would be futile to measure the real amount of energy produced by your empire.",
	};

	pub const ORE: Resource = Resource {
		r#type: ResourceType::Basic,
		name: "Ore",
		plural: "Ores",
		description: "Ores are a collective term for the basic resources we need to construct various structures and refine into alloys.",
	};

	pub const FOOD: Resource = Resource {
		r#type: ResourceType::Basic,
		name: "Food",
		plural: "Food",
		description: "Food represents the various nutrients required to sustain and grow Pops.",
	};

	pub const ALLOY: Resource = Resource {
		r#type: ResourceType::Refined,
		name: "Alloy",
		plural: "Alloys",
		description: "Alloys are advanced resources which we need for things in space, like the navy and stations.",
	};

	pub const LUXURIES: Resource = Resource {
		r#type: ResourceType::Refined,
		name: "Luxury",
		plural: "Luxuries",
		description: "Luxuries are resources that we need to satisfy Pops. Pops of a higher stratum will need more luxuries.",
	};

	pub const PHYSICS: Resource = Resource {
		r#type: ResourceType::Abstract,
		name: "Physics",
		plural: "Physics",
		description: "Physics is a natural science that involves the study of matter and its motion through spacetime, along with related concepts such as energy and force.",
	};

	pub const ENGINEERING: Resource = Resource {
		r#type: ResourceType::Abstract,
		name: "Engineering",
		plural: "Engineering",
		description: "Engineering is the use of scientific principles to design and build machines, structures, and other items.",
	};

	pub const SOCIETY: Resource = Resource {
		r#type: ResourceType::Abstract,
		name: "Society",
		plural: "Society",
		description: "Society research is the study of societies and the relationships among individuals within those societies.",
	};

	pub const HARMONY: Resource = Resource {
		r#type: ResourceType::Abstract,
		name: "Harmony",
		plural: "Harmony",
		description: "Harmony is used to advance our society and government.",
	};

	pub const TENACIUM: Resource = Resource {
		r#type: ResourceType::Strategic,
		name: "Tenacium",
		plural: "Tenacium",
		description: "Tenacium is an extremely durable crystal used notably for enhancing laser weapons and hull/shield plating.",
	};

	pub const NEGATIUM: Resource = Resource {
		r#type: ResourceType::Strategic,
		name: "Negatium",
		plural: "Negatium",
		description: "Negatium is the only known material in the universe that has negative mass. It is essential for warp travel.",
	};

	pub const DARK_MATTER: Resource = Resource {
		r#type: ResourceType::Exotic,
		name: "Dark Matter",
		plural: "Dark Matter",
		description: "Dark Matter is notoriously hard to harvest, doing so is only possible in black holes, but it has exotic natural properties that we could exploit. At the very least, it is possible to sell it for a high price."
	};

	pub const NANOBOT: Resource = Resource {
		r#type: ResourceType::Exotic,
		name: "Nanobot",
		plural: "Nanobots",
		description: "Nanobots are machines built on the nanoscale, with potential applications in a wide range of fields."
	};
}