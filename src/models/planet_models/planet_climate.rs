use crate::models::planet_models::planet_climate::PlanetClimate::*;
use crate::models::planet_models::planet_type::PlanetType;
use crate::models::planet_models::planet_type::PlanetType::*;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, PartialEq)]
pub enum PlanetClimate {
	Dry = Arid as isize | Desert as isize | Savanna as isize,
	Frozen = Alpine as isize | Arctic as isize | Tundra as isize,
	Wet = Continental as isize | Ocean as isize | Tropical as isize,
	Uninhabitable = !(Dry as isize | Frozen as isize | Wet as isize),
	Superhabitable = PlanetType::Superhabitable as isize,
}