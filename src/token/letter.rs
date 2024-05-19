pub enum Letter {
	Alif,
	Ba,
	Ta,
	Tha,
	Jeem,
	Hha,
	Kha,

	Dal,
	Thal,

	Roh,
	Zah,

	Sheen,
	Seen,

	Sod,
	Dod,

	Toh,
	Dhoh,

	Ein,
	Ghein,

	Fa,
	Qof,
	Kaf,

	Lam,
	Meem,
	Nun,

	Ha,
	Waw,
	Yah
}

impl Letter {
	/// If the letter is a sun letter, then Al will blend with the first letter of the world. Naar (fire) with The
	/// would become An-naar.
	/// If the letter is a moon letter, then Baab (door) with The would become Al-Baab.
	pub fn is_assimilating(&self) -> Consonant {
		match self {
			Self::Alif
			| Self::Baa
			| Self::Jeem
			| Self::Hha
			| Self::Kha
			| Self::Ein
			| Self::Ghein
			| Self::Fa
			| Self::Qof
			| Self::Kaf
			| Self::Meem
			| Self::Ha
			| Self::Waw
			| Self::Yah => false,
			_ => true
		}
	}
}