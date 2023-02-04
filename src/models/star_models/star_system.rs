use crate::models::planet_models::planet::Planet;
use super::star::Star;

pub struct StarSystem {
	pub star: Star,
	pub planets: Vec<Planet>,
}