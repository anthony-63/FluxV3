#[derive(Clone, Default)]
pub struct Score {
    pub hits: usize,
    pub misses: usize,
    pub total: usize,
}

impl Score {
    pub fn get_accuracy(&self) -> f64 {
        return (self.hits as f64 / self.total as f64) * 100.
    }
}