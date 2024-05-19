pub mod state;
pub mod letter;

use std::borrow::Cow;

pub enum Gender {
	Male,
	Female
}

pub enum Tanween {
	Fatha,
	Kusra,
	Damma
}

pub trait Noun {
	/// If something is definite then we are referencing something specific that both interpreters know of.
	/// Indefinite things are generic. An apple, A car.
	///
	/// Indefinite is non-specific. Definite is specific.
	fn get_state(&mut self) -> bool;
	fn get_gender(&mut self) -> Gender;
	fn get_number(&mut self);
	fn get_tanween(&mut self) -> Tanween;
}

pub trait Annotated<'a> {
	fn annotate(&mut self) -> Cow<'a, str>;
}

pub enum WordGroups {
	Noun,       // اسم (isym)
	Verb,       // فعل (feighl, Fey Eighl)
	Preposition // حرف (harrf, hurrf)
}