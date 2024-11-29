use std::usize;

pub struct Interval {
    pub start: usize,
    pub end: usize,
}

impl Interval {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub const MAX: Self = Self {
        start: 0,
        end: usize::MAX,
    };

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let start: usize = usize::max(self.start, other.start);

        let end: usize = usize::min(self.end, other.end);

        (start <= end).then(|| Self::new(start, end))
    }

    pub fn is_intersecting(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    pub fn complement(&self) -> (Option<Self>, Option<Self>) {
        (
            (self.start > 0).then(|| Interval::new(0, self.start - 1)),
            (self.end < usize::MAX).then(|| Interval::new(self.end + 1, usize::MAX)),
        )
    }
}
