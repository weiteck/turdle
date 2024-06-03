use tuirealm::props::Color;

pub const CELL_FG: Color = Color::White;
pub const CELL_BG_EMPTY: Color = Color::Indexed(238);
pub const CELL_BG_INCORRECT: Color = Color::Indexed(236);
pub const CELL_BG_CONTAINS: Color = Color::Indexed(214);
pub const CELL_BG_CORRECT: Color = Color::Indexed(2);

pub const LETTER_FG: Color = CELL_FG;
pub const LETTER_BG_UNUSED: Color = CELL_BG_EMPTY;
pub const LETTER_FG_INCORRECT: Color = Color::Indexed(240);
pub const LETTER_BG_INCORRECT: Color = CELL_BG_INCORRECT;
pub const LETTER_BG_CONTAINS: Color = CELL_BG_CONTAINS;
pub const LETTER_BG_CORRECT: Color = CELL_BG_CORRECT;