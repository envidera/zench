use super::IBatcher;

#[derive(Debug, Clone, Copy)]
pub struct F {
    pub(crate) batch_size: usize,
}

impl Default for F {
    fn default() -> Self {
        Self { batch_size: 100 }
    }
}

impl IBatcher for F {
    fn estimate_batch_size<F, R>(&self, _closure: &mut F) -> usize
    where
        F: FnMut() -> R,
    {
        self.batch_size
    }
}

impl F {
    #[allow(unused)]
    pub(crate) fn set_batch_size(mut self, value: usize) -> Self {
        self.batch_size = value;
        self
    }
}
