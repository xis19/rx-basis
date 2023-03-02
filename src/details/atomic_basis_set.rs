use std::vec::Vec;

use super::{angular_momentum::AngularMomentum, gaussian_exp::SegmentedContraction};

pub struct AtomicBasisSet(Vec<Vec<SegmentedContraction>>);

impl AtomicBasisSet {
    pub fn new() -> Self {
        AtomicBasisSet(vec![])
    }

    pub fn get_num_contracted_functions(&self) -> usize {
        self.0
            .iter()
            .map(|angular_momentum| angular_momentum.len())
            .sum()
    }

    pub fn get_num_gaussian_primitives(&self) -> usize {
        self.0
            .iter()
            .map(|seg_contractions| {
                seg_contractions
                    .iter()
                    .map(|seg_contraction| seg_contraction.get_num_primitives())
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn get_highest_angular_momentum(&self) -> AngularMomentum {
        let len = self.0.len();

        if len == 0 {
            AngularMomentum::UnsupportedAngularMomentum
        } else {
            AngularMomentum::from(len - 1)
        }
    }

    pub fn add_segmented_contraction(
        &mut self,
        angular_momentum: AngularMomentum,
        segmented_contraction: SegmentedContraction,
    ) -> &mut Self {
        let angular_momentum_num = angular_momentum as usize;
        while self.0.len() <= angular_momentum_num {
            self.0.push(vec![]);
        }
        self.0[angular_momentum_num].push(segmented_contraction);
        self
    }
}

pub struct SegmentedContractionIntoIterator<'a> {
    ao_basis_set: &'a AtomicBasisSet,
    angular_momentum_index: usize,
    segmented_contraction_index: usize,
}

impl<'a> SegmentedContractionIntoIterator<'a> {
    pub fn new(ao_basis_set: &'a AtomicBasisSet) -> Self {
        SegmentedContractionIntoIterator {
            ao_basis_set,
            angular_momentum_index: 0,
            segmented_contraction_index: 0,
        }
    }
}

impl<'a> Iterator for SegmentedContractionIntoIterator<'a> {
    type Item = (AngularMomentum, &'a SegmentedContraction);

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Option<Self::Item> = None;

        while self.angular_momentum_index < self.ao_basis_set.0.len() {
            let scgtos = &self.ao_basis_set.0[self.angular_momentum_index];
            if self.segmented_contraction_index < scgtos.len() {
                result = Some((
                    AngularMomentum::from(self.angular_momentum_index),
                    &scgtos[self.segmented_contraction_index],
                ));
                self.segmented_contraction_index += 1;
                break;
            } else {
                self.segmented_contraction_index = 0;
                self.angular_momentum_index += 1;
            }
        }

        result
    }
}

impl<'a> IntoIterator for &'a AtomicBasisSet {
    type Item = (AngularMomentum, &'a SegmentedContraction);

    type IntoIter = SegmentedContractionIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SegmentedContractionIntoIterator::new(&self)
    }
}
