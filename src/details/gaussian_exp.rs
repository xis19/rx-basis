use std::vec::Vec;

#[derive(Clone, Copy)]
pub struct GaussianPrimitive {
    coefficient: f64,
    exponental: f64,
}

impl GaussianPrimitive {
    pub fn new(coefficient: f64, exponental: f64) -> Self {
        GaussianPrimitive {
            coefficient,
            exponental,
        }
    }

    pub fn coefficient(&self) -> f64 {
        self.coefficient
    }

    pub fn exponental(&self) -> f64 {
        self.exponental
    }
}

pub struct SegmentedContraction(Vec<GaussianPrimitive>);

impl SegmentedContraction {
    pub fn new() -> Self {
        SegmentedContraction(vec![])
    }

    pub fn add(&mut self, coefficient: f64, exponental: f64) -> &mut Self {
        self.add_primitive(GaussianPrimitive::new(coefficient, exponental))
    }

    pub fn add_primitive(&mut self, primitive: GaussianPrimitive) -> &mut Self {
        self.0.push(primitive);
        self
    }

    pub fn get_num_primitives(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&GaussianPrimitive> {
        self.0.get(index)
    }
}
