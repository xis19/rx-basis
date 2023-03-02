use std::error::Error;

use crate::details::{
    angular_momentum::AngularMomentum, atomic_basis_set::AtomicBasisSet,
    gaussian_exp::SegmentedContraction,
};

#[derive(Debug)]
pub struct BasisSetParseError(String);

impl BasisSetParseError {
    fn new(message: &str) -> Self {
        BasisSetParseError(message.to_string())
    }
}

impl std::fmt::Display for BasisSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.to_string(), self.0)
    }
}

impl Error for BasisSetParseError {
    fn description(&self) -> &str {
        "Failed to parse Gaussian basis set information"
    }
}

#[derive(Debug, PartialEq)]
pub enum BasisSetAssignmentType {
    // Basis set for a type of atom
    Atom(String),
    // Basis set for a particle in the molecule, specified by the index starting with 0
    ParticleIndex(i32),
}

fn read_single_basis_set_line(
    stream: &mut dyn Iterator<Item = Result<String, std::io::Error>>,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut option_item = stream.next();
    while let Some(item) = option_item {
        match item {
            Ok(string) => {
                if string.starts_with("!") || string.trim().len() == 0 {
                    option_item = stream.next();
                    continue;
                }
                if string.starts_with("****") {
                    return Ok(None);
                }
                return Ok(Some(string));
            }
            Err(error) => {
                return Err(Box::new(BasisSetParseError(error.to_string())));
            }
        }
    }
    return Ok(None);
}

fn parse_basis_set_first_line(
    first_line: &Option<String>,
) -> Result<BasisSetAssignmentType, Box<dyn Error>> {
    match first_line {
        None => Err(Box::new(BasisSetParseError::new("Bad basis set"))),
        Some(declaration_line) => {
            let mut split = declaration_line.split_whitespace();
            let value = split
                .next()
                .ok_or_else(|| BasisSetParseError::new("Expect atom/particle index"))?;
            let particle_index = value.parse::<i32>();
            match particle_index {
                Err(_) => Ok(BasisSetAssignmentType::Atom(value.to_string())),
                Ok(v) => Ok(BasisSetAssignmentType::ParticleIndex(v)),
            }
        }
    }
}

fn parse_cgto_first_line(line: &Option<String>) -> Result<(String, i32), Box<dyn Error>> {
    match line {
        None => Err(Box::new(BasisSetParseError::new(
            "Expecting CGTO declaration",
        ))),
        Some(declaration_line) => {
            let mut split = declaration_line.split_whitespace();
            let angular_momentum: String = split
                .next()
                .ok_or_else(|| {
                    BasisSetParseError::new(
                        "Expecting angular momentum and number of Gaussian primitives",
                    )
                })?
                .to_string();
            match split.next() {
                None => Err(Box::new(BasisSetParseError::new("Bad CGTO declaration"))),
                Some(value) => {
                    let num_gaussian_primitives: i32 = value.parse()?;
                    Ok((angular_momentum, num_gaussian_primitives))
                }
            }
        }
    }
}

fn parse_floats(line: &Option<String>) -> Result<Vec<f64>, Box<dyn Error>> {
    match line {
        None => Err(Box::new(BasisSetParseError::new(
            "Expecting line of floats",
        ))),
        Some(value_line) => match value_line
            .split_whitespace()
            .map(|i| i.parse::<f64>())
            .collect()
        {
            Ok(value) => Ok(value),
            Err(err) => Err(Box::new(BasisSetParseError(err.to_string()))),
        },
    }
}

fn add_basis_set_cgto(
    basis_set: &mut AtomicBasisSet,
    angular_momentum_string: &str,
    data: &Vec<Vec<f64>>,
) {
    // The index of the exponental term
    let mut index = 1 as usize;
    // Angular momentum should be Ss Pp Dd Ff Gg Hh, etc.
    for angular_momentum_ch in angular_momentum_string.as_bytes().into_iter() {
        let angular_momentum = AngularMomentum::from(*angular_momentum_ch as char);
        let mut segmented_contraction = SegmentedContraction::new();
        for gaussian_index in 0..data.len() {
            let coefficient = data[gaussian_index][0];
            let exponental = data[gaussian_index][index];
            segmented_contraction.add(coefficient, exponental);
        }
        basis_set.add_segmented_contraction(angular_momentum, segmented_contraction);
        index += 1;
    }
}

pub fn read_basis_set(
    stream: &mut dyn Iterator<Item = Result<String, std::io::Error>>,
) -> Result<(BasisSetAssignmentType, AtomicBasisSet), Box<dyn Error>> {
    let mut basis_set = AtomicBasisSet::new();
    let mut read_result = read_single_basis_set_line(stream)?;
    let basis_set_assignment_type = parse_basis_set_first_line(&read_result)?;

    read_result = read_single_basis_set_line(stream)?;
    while !read_result.is_none() {
        let cgto_declaration = parse_cgto_first_line(&read_result)?;

        let mut basis_set_data = Vec::<Vec<f64>>::new();
        for _ in 0..cgto_declaration.1 {
            let primitive_line = read_single_basis_set_line(stream)?;
            basis_set_data.push(parse_floats(&primitive_line)?);
        }

        add_basis_set_cgto(&mut basis_set, &cgto_declaration.0, &basis_set_data);

        read_result = read_single_basis_set_line(stream)?;
    }

    Ok((basis_set_assignment_type, basis_set))
}

impl ToString for BasisSetAssignmentType {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl ToString for AtomicBasisSet {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, Cursor};

    use approx::assert_abs_diff_eq;

    use crate::{
        details::angular_momentum::AngularMomentum,
        io::gaussian::{parse_basis_set_first_line, BasisSetAssignmentType},
    };

    use super::{parse_cgto_first_line, parse_floats, read_basis_set};

    #[test]
    fn test_parse_floats() {
        assert!(parse_floats(&None).is_err());
        assert!(parse_floats(&Some("1.0 err 2.0".to_string())).is_err());

        let parsed = parse_floats(&Some("1.0 2.0 3.0".to_string())).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_abs_diff_eq!(parsed[0], 1.0);
        assert_abs_diff_eq!(parsed[1], 2.0);
        assert_abs_diff_eq!(parsed[2], 3.0);
    }

    #[test]
    fn test_parse_cgto_first_line() {
        assert!(parse_cgto_first_line(&None).is_err());
        assert!(parse_cgto_first_line(&Some(" S ".to_string())).is_err());

        let good = parse_cgto_first_line(&Some(" SP 6 ".to_string())).unwrap();
        assert!(good.0 == "SP");
        assert_eq!(good.1, 6);
    }

    #[test]
    fn test_parse_basis_set_first_line() {
        assert!(parse_basis_set_first_line(&None).is_err());

        assert_eq!(
            parse_basis_set_first_line(&Some("C 0".to_string())).unwrap(),
            BasisSetAssignmentType::Atom("C".to_string())
        );

        assert_eq!(
            parse_basis_set_first_line(&Some("1 0".to_string())).unwrap(),
            BasisSetAssignmentType::ParticleIndex(1)
        );
    }

    // 6-311G basis set for C
    const CARBON_BASIS_SET: &'static str = "\n
!----------------------------------------------------------------------
! Basis Set Exchange
! Version v0.9
! https://www.basissetexchange.org
!----------------------------------------------------------------------
!   Basis set: 6-311G
! Description: VTZ Valence Triple Zeta: 3 Funct.'s/Valence AO
!        Role: orbital
!     Version: 0  (Data from the Original Basis Set Exchange)
!----------------------------------------------------------------------


C     0
S    6   1.00
   4563.240                  0.00196665
    682.0240                 0.0152306
    154.9730                 0.0761269
     44.45530                0.2608010
     13.02900                0.6164620
      1.827730               0.2210060
SP   3   1.00
     20.96420                0.114660               0.0402487
      4.803310               0.919999               0.237594
      1.459330              -0.00303068             0.815854
SP   1   1.00
      0.4834560              1.000000               1.000000
SP   1   1.00
      0.1455850              1.000000               1.000000
****
";

    #[test]
    fn test_load_carbon_basis_set() {
        let input_stream = Cursor::new(CARBON_BASIS_SET);

        let (assignment_type, basis_set) = read_basis_set(&mut input_stream.lines()).unwrap();
        assert_eq!(
            assignment_type,
            BasisSetAssignmentType::Atom("C".to_string())
        );
        assert_eq!(basis_set.get_num_contracted_functions(), 7);
        assert_eq!(basis_set.get_num_gaussian_primitives(), 16);

        // Check the basis set values
        let mut cgto_iter = basis_set.into_iter();
        let (cgto1_am, cgto1_sc) = cgto_iter.next().unwrap();
        assert_eq!(cgto1_am, AngularMomentum::S);
        assert_eq!(cgto1_sc.get_num_primitives(), 6);
        assert_abs_diff_eq!(cgto1_sc.get(2).unwrap().coefficient(), 154.9730);
        assert_abs_diff_eq!(cgto1_sc.get(3).unwrap().exponental(), 0.2608010);

        cgto_iter.next();
        let (cgto2_am, cgto2_sc) = cgto_iter.next().unwrap();
        assert_eq!(cgto2_am, AngularMomentum::S);
        assert_eq!(cgto2_sc.get_num_primitives(), 1);
        assert_abs_diff_eq!(cgto2_sc.get(0).unwrap().coefficient(), 0.4834560);
        assert_abs_diff_eq!(cgto2_sc.get(0).unwrap().exponental(), 1.0);

        cgto_iter.next();
        let (cgto3_am, cgto3_sc) = cgto_iter.next().unwrap();
        assert_eq!(cgto3_am, AngularMomentum::P);
        assert_eq!(cgto3_sc.get_num_primitives(), 3);
        assert_abs_diff_eq!(cgto3_sc.get(2).unwrap().coefficient(), 1.459330);
        assert_abs_diff_eq!(cgto3_sc.get(2).unwrap().exponental(), 0.815854);

        cgto_iter.next();
        cgto_iter.next();
        assert!(cgto_iter.next().is_none());
    }
}
