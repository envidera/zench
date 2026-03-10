use crate::benchmark::outlier;
use crate::benchmark::outlier::Outlier;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Samples {
    samples: Vec<Sample>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub(crate) value: f64,
    pub(crate) outlier: bool,
}

impl Samples {
    pub(crate) fn new() -> Self {
        Samples {
            samples: Vec::new(),
        }
    }

    pub(crate) fn count_outliers(&self) -> usize {
        self.samples
            .iter()
            .filter(|s| s.is_outlier())
            .count()
    }

    pub(crate) fn len(&self) -> usize {
        self.samples
            .len()
    }

    // data() return the raw samples f64 values
    pub(crate) fn data(&self) -> Vec<f64> {
        self.iter()
            .map(|f| f.value)
            .collect()
    }

    // remove the outliers, and return data
    pub(crate) fn data_without_outliers(&self) -> Vec<f64> {
        self.iter()
            .filter(|s| !s.outlier)
            .map(|s| s.value)
            .collect()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Sample> {
        self.samples
            .iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Sample> {
        self.samples
            .iter_mut()
    }

    pub fn mark_outliers(&mut self, algorithm: &Outlier) {
        match algorithm {
            Outlier::Iqr => outlier::iqr(self),
            Outlier::Iqrz => outlier::iqrz(self),
            Outlier::ModifiedZScore => outlier::modified_z_score(self),
        }
    }
}

impl Sample {
    pub(crate) fn is_outlier(&self) -> bool {
        self.outlier
    }

    pub(crate) fn outlier(&mut self) {
        self.outlier = true;
    }
}

impl From<&[f64]> for Samples {
    fn from(data: &[f64]) -> Self {
        let samples: Vec<Sample> = data
            .iter()
            .map(|&s| Sample {
                value: s,
                outlier: false,
            })
            .collect();

        let mut s = Samples { samples };
        s.mark_outliers(&Outlier::default());
        s
    }
}

impl From<Vec<f64>> for Samples {
    fn from(value: Vec<f64>) -> Self {
        let data: Vec<Sample> = value
            .iter()
            .map(|&s| Sample {
                value: s,
                outlier: false,
            })
            .collect();

        Samples { samples: data }
    }
}
