use std::fmt::{Display, Error, Formatter, Write};

///
/// Modélisation de chaque lettre à trouver.
///
/// Seules les majuscules entre A et Z sont à trouver, les autres caractères
/// son considérés comme déjà trouvés.
///
pub struct LettreATrouver {
    // code ascii, en majuscule
    letter: u8,
    found: bool,
}

impl LettreATrouver {
    pub fn is_found(&self) -> bool {
        self.found
    }
    pub fn set_found(&mut self) {
        self.found = true
    }
    pub fn set_force_display(&mut self) {
        self.set_found()
    }
}

impl PartialEq<u8> for LettreATrouver {
    fn eq(&self, other: &u8) -> bool {
        self.letter == *other
    }
}

impl From<u8> for LettreATrouver {
    fn from(c: u8) -> LettreATrouver {
        LettreATrouver {
            letter: c,
            found: c < 'A' as u8 || c > 'Z' as u8,
        }
    }
}

impl Display for LettreATrouver {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.found {
            f.write_char(self.letter as char)
        } else {
            f.write_char('_')
        }
    }
}
