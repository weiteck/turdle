use tuirealm::props::Color;

pub const CELL_FG: Color = Color::White;
pub const CELL_BG_EMPTY: Color = Color::Indexed(237);
pub const CELL_BG_INCORRECT: Color = Color::Indexed(235);
pub const CELL_BG_CONTAINS: Color = Color::Indexed(3);
pub const CELL_BG_CORRECT: Color = Color::Indexed(2);

pub const LETTER_FG: Color = CELL_FG;
pub const LETTER_BG_UNUSED: Color = CELL_BG_EMPTY;
pub const LETTER_FG_INCORRECT: Color = Color::Indexed(241);
pub const LETTER_BG_INCORRECT: Color = CELL_BG_INCORRECT;
pub const LETTER_BG_CONTAINS: Color = CELL_BG_CONTAINS;
pub const LETTER_BG_CORRECT: Color = CELL_BG_CORRECT;