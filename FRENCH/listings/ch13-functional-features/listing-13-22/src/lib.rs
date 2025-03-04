struct Compteur {
    compteur: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compteur: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.compteur < 5 {
            self.compteur += 1;
            Some(self.compteur)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn appel_direct_a_next() {
        let mut compteur = Compteur::new();

        assert_eq!(compteur.next(), Some(1));
        assert_eq!(compteur.next(), Some(2));
        assert_eq!(compteur.next(), Some(3));
        assert_eq!(compteur.next(), Some(4));
        assert_eq!(compteur.next(), Some(5));
        assert_eq!(compteur.next(), None);
    }
    // ANCHOR_END: here
}
