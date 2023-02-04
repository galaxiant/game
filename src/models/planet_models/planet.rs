use crate::models::planet_models::planet_climate::PlanetClimate;
use crate::models::planet_models::planet_type::PlanetType;
use strum::IntoEnumIterator;

pub struct Planet {
	pub r#type: PlanetType,
	pub climate: PlanetClimate,
	pub is_habitable: bool,
}

impl Planet {
	fn get_climate(r#type: PlanetType) -> PlanetClimate {
		let type_isize = r#type as isize;

		for climate in PlanetClimate::iter() {
			if (climate as isize & type_isize) == type_isize {
				return climate;
			}
		}

		panic!("The planet type {:#?} does not have an assigned climate.", r#type);
	}

	pub fn new(r#type: PlanetType) -> Planet {
		let climate = Self::get_climate(r#type);
		let is_habitable = climate != PlanetClimate::Uninhabitable;

		Planet {
			r#type,
			climate,
			is_habitable,
		}
	}
}