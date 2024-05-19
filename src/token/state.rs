pub enum Definite {
	/// Names of people and places
	ProperNoun,
	/// When "the" is used
	Specific,
	/// When a non definite and a definite are used together.
	Construct
}

pub enum State {
	Definite(Definite),
	Indefinite
}