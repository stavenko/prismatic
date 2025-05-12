use std::marker::PhantomData;

pub struct ParametricIterator<F> {
    segments: usize,
    current_segment: usize,
    _tp: PhantomData<F>,
}

impl<F> ParametricIterator<F> {
    pub fn new(segments: usize) -> Self {
        Self {
            segments,
            current_segment: 0,
            _tp: Default::default(),
        }
    }

    pub fn new_with_start(segments: usize, start: usize) -> Self {
        Self {
            segments,
            current_segment: start,
            _tp: Default::default(),
        }
    }
}

impl<F> Iterator for ParametricIterator<F>
where
    F: From<usize>,
    F: std::ops::Div<Output = F>,
{
    type Item = (F, F);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.current_segment;
        let next = first + 1;
        self.current_segment += 1;
        if next > self.segments {
            None
        } else {
            let first = F::from(first) / F::from(self.segments);
            let next = F::from(next) / F::from(self.segments);
            Some((first, next))
        }
    }
}
