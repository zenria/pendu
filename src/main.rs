use ncurses::*;
use rand::prelude::*;

mod potence;
use crate::potence::Potence;

mod lettre_a_trouver;
use crate::lettre_a_trouver::LettreATrouver;

fn get_max_xy() -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    return (max_x, max_y);
}

fn print_center(text: &str, y: i32) {
    let (max_x, _) = get_max_xy();
    let x = (max_x - text.len() as i32) / 2;
    mv(y, x);
    printw(text);
}

///
/// Le jeu et son état courant
///
struct Jeu {
    potence: Potence,
    a_trouver: Vec<LettreATrouver>,
    deja_tapee: Vec<u8>,
}

impl Jeu {
    fn new(mots: &Vec<&str>) -> Self {
        let rnd = random::<usize>() % mots.len();
        let letters: Vec<u8> = mots[rnd].to_string().to_uppercase().into();
        let coucou: Vec<LettreATrouver> = letters.into_iter().map(|c| c.into()).collect();
        Jeu {
            potence: Potence::Debut,
            a_trouver: coucou,
            deja_tapee: Vec::new(),
        }
    }
    fn gagne(&self) -> bool {
        self.a_trouver.iter().all(|l| l.is_found())
    }
    fn perdu(&self) -> bool {
        match self.potence {
            Potence::Perdu => true,
            _ => false,
        }
    }
    fn fini(&self) -> bool {
        self.perdu() || self.gagne()
    }
    ///
    /// L'algo principal de jeu, on se régale sur les iterateurs
    ///
    fn lettre(&mut self, c: u8) {
        if self.perdu() || self.gagne() {
            // Jeu terminé
            return;
        }
        if self.deja_tapee.contains(&c) {
            // Lettre déjà utilisée
            return;
        }
        self.deja_tapee.push(c);

        if self
            .a_trouver
            .iter_mut()
            .filter(|l| **l == c)
            .fold(0, |count, l| {
                // pour chaque lettre trouvée, on la marque comme tel, et on compte
                l.set_found();
                count + 1
            })
            == 0
        {
            // Pas de lettre trouvée!
            match self.potence.next() {
                // Quand on perd => on marque comme trouvées toutes les lettres, ainsi,
                // elles sont affichées au joueur !
                Some(Potence::Perdu) => self.a_trouver.iter_mut().for_each(|l| {
                    l.set_force_display();
                }),
                _ => (),
            }
        }
    }
}

///
/// Affichage de du jeu
///
///
fn print_jeu(jeu: &Jeu) {
    clear();
    print_center("LE PENDU", 2);
    let (max_x, _) = get_max_xy();

    let a_trouver_x = (max_x - jeu.a_trouver.len() as i32 * 2) / 2;

    jeu.a_trouver.iter().enumerate().for_each(|(i, l)| {
        mvprintw(4, a_trouver_x + i as i32 * 2, format!("{}", l).as_str());
    });

    let pendu_x = max_x / 2 - 8;
    jeu.potence
        .as_strs()
        .iter()
        .enumerate()
        .for_each(|(y, ligne)| {
            mvprintw(y as i32 + 8, pendu_x, ligne);
        });

    if jeu.fini() {
        attron(A_BOLD());
        if jeu.perdu() {
            print_center("PERDU :(", 6);
        } else {
            print_center("GAGNE !!", 6);
        }
        attroff(A_BOLD());

        print_center(
            " Appuie sur R pour recommencer",
            jeu.potence.as_strs().len() as i32 + 8,
        );
    } else {
        print_center("Choisi une lettre: ", 6);
    }
}
///
/// Init & Boucle principale
///
fn main() {
    // Chargement de la liste de mots
    let mots: String = include_str!("mots.txt").to_string();
    let mots: Vec<&str> = mots.lines().filter(|mot| mot.len() > 0).collect();
    /* Setup ncurses. */
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    let mut jeu = Jeu::new(&mots);
    print_jeu(&jeu);
    loop {
        let ch = getch();

        let ch = match ch {
            3 | 37 => break,
            x if x >= 'A' as i32 && x <= 'Z' as i32 => x,
            x if x >= 'a' as i32 && x <= 'z' as i32 => x - ('a' as i32 - 'A' as i32),
            _ => continue,
        };
        jeu.lettre(ch as u8);

        if jeu.fini() && ch == 'R' as i32 {
            jeu = Jeu::new(&mots);
        }

        print_jeu(&jeu);
    }

    endwin();
}
