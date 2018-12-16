///
/// Le niveau d'avancement dans le jeu, on aurait pu appeler ça NiveauDePertition ;)
///
#[derive(Clone, Copy, PartialEq)]
pub enum Potence {
    Debut,
    Etape1,
    Etape2,
    Etape3,
    Etape4,
    Etape5,
    Etape6,
    Etape7,
    Etape8,
    Perdu,
}

use self::Potence::*;

///
/// La façon d'afficher chaque niveau
///
impl Potence {
    pub fn as_strs(&self) -> Vec<&str> {
        match self {
            Debut => vec![],
            Etape1 => vec![
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "            \n",
                "============\n",
            ],
            Etape2 => vec![
                "            \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "============\n",
            ],
            Etape3 => vec![
                "    ,=====Y=\n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "    ||      \n",
                "============\n",
            ],
            Etape4 => vec![
                "    ,=====Y=\n",
                "    ||   /  \n",
                "    ||  /   \n",
                "    || /    \n",
                "    ||/     \n",
                "    ||      \n",
                "    ||      \n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
            Etape5 => vec![
                "    ,=====Y=\n",
                "    ||   /| \n",
                "    ||  / | \n",
                "    || /  | \n",
                "    ||/     \n",
                "    ||      \n",
                "    ||      \n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
            Etape6 => vec![
                "    ,=====Y=\n",
                "    ||   /| \n",
                "    ||  / | \n",
                "    || /  | \n",
                "    ||/   o \n",
                "    ||      \n",
                "    ||      \n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
            Etape7 => vec![
                "    ,=====Y=\n",
                "    ||   /| \n",
                "    ||  / | \n",
                "    || /  | \n",
                "    ||/   o \n",
                "    ||    | \n",
                "    ||      \n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
            Etape8 => vec![
                "    ,=====Y=\n",
                "    ||   /| \n",
                "    ||  / | \n",
                "    || /  | \n",
                "    ||/   o \n",
                "    ||   /|\\\n",
                "    ||      \n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
            Perdu => vec![
                "    ,=====Y=\n",
                "    ||   /| \n",
                "    ||  / | \n",
                "    || /  | \n",
                "    ||/   o \n",
                "    ||   /|\\\n",
                "    ||   / \\\n",
                "   /||      \n",
                "  //||      \n",
                "============\n",
            ],
        }
    }

    fn move_forward(&mut self) {
        match self {
            Potence::Debut => *self = Potence::Etape1,
            Potence::Etape1 => *self = Potence::Etape2,
            Potence::Etape2 => *self = Potence::Etape3,
            Potence::Etape3 => *self = Potence::Etape4,
            Potence::Etape4 => *self = Potence::Etape5,
            Potence::Etape5 => *self = Potence::Etape6,
            Potence::Etape6 => *self = Potence::Etape7,
            Potence::Etape7 => *self = Potence::Etape8,
            Potence::Etape8 => *self = Potence::Perdu,
            Potence::Perdu => (),
        }
    }
}

impl Iterator for Potence {
    type Item = Self;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if *self == Perdu {
            None
        } else {
            self.move_forward();
            Some(*self)
        }
    }
}
