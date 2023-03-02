/// Angular momentum

#[repr(i8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AngularMomentum {
    S = 0,
    P = 1,
    D = 2,
    F = 3,
    G = 4,
    H = 5,

    UnsupportedAngularMomentum = -1,
}

impl From<char> for AngularMomentum {
    fn from(ch: char) -> Self {
        match ch {
            'S' | 's' => AngularMomentum::S,
            'P' | 'p' => AngularMomentum::P,
            'D' | 'd' => AngularMomentum::D,
            'F' | 'f' => AngularMomentum::F,
            'G' | 'g' => AngularMomentum::G,
            'H' | 'h' => AngularMomentum::H,
            _ => AngularMomentum::UnsupportedAngularMomentum,
        }
    }
}

impl From<usize> for AngularMomentum {
    fn from(us: usize) -> Self {
        match us {
            0 => AngularMomentum::S,
            1 => AngularMomentum::P,
            2 => AngularMomentum::D,
            3 => AngularMomentum::F,
            4 => AngularMomentum::G,
            5 => AngularMomentum::H,
            _ => AngularMomentum::UnsupportedAngularMomentum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AngularMomentum;

    #[test]
    fn test_to_angular_momentums() {
        assert_eq!(AngularMomentum::from('S'), AngularMomentum::S);
        assert_eq!(AngularMomentum::from('s'), AngularMomentum::S);
        assert_eq!(AngularMomentum::from(0), AngularMomentum::S);

        assert_eq!(AngularMomentum::from('P'), AngularMomentum::P);
        assert_eq!(AngularMomentum::from('p'), AngularMomentum::P);
        assert_eq!(AngularMomentum::from(1), AngularMomentum::P);

        assert_eq!(AngularMomentum::from('D'), AngularMomentum::D);
        assert_eq!(AngularMomentum::from('d'), AngularMomentum::D);
        assert_eq!(AngularMomentum::from(2), AngularMomentum::D);

        assert_eq!(AngularMomentum::from('F'), AngularMomentum::F);
        assert_eq!(AngularMomentum::from('f'), AngularMomentum::F);
        assert_eq!(AngularMomentum::from(3), AngularMomentum::F);

        assert_eq!(AngularMomentum::from('G'), AngularMomentum::G);
        assert_eq!(AngularMomentum::from('g'), AngularMomentum::G);
        assert_eq!(AngularMomentum::from(4), AngularMomentum::G);

        assert_eq!(AngularMomentum::from('H'), AngularMomentum::H);
        assert_eq!(AngularMomentum::from('h'), AngularMomentum::H);
        assert_eq!(AngularMomentum::from(5), AngularMomentum::H);

        assert_eq!(
            AngularMomentum::from('T'),
            AngularMomentum::UnsupportedAngularMomentum
        );
        assert_eq!(
            AngularMomentum::from(6),
            AngularMomentum::UnsupportedAngularMomentum
        );
    }

    #[test]
    fn test_from_angular_momentum() {
        assert_eq!(AngularMomentum::S as i8, 0);
        assert_eq!(AngularMomentum::P as i8, 1);
        assert_eq!(AngularMomentum::D as i8, 2);
        assert_eq!(AngularMomentum::F as i8, 3);
        assert_eq!(AngularMomentum::G as i8, 4);
        assert_eq!(AngularMomentum::H as i8, 5);
    }
}
