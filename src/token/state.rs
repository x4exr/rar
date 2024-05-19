pub enum Definite {
	/// Names of people and places
	ProperNoun,
	/// When "the" is used
	Specific
}

pub enum State {
	Definite(Definite),
	Indefinite
}