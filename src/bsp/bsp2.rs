use itertools::Itertools;

use crate::primitives::{line2d::Line2D, line2d::SplitResult, segment2d::Segment2D};

#[derive(Debug, Clone, PartialEq)]
pub struct Bsp2d {
    line: Line2D,
    pub front: Option<Box<Bsp2d>>,
    pub back: Option<Box<Bsp2d>>,
    pub coplanar_front: Vec<Segment2D>,
    pub coplanar_back: Vec<Segment2D>,
}

impl Bsp2d {
    fn new(line: Line2D) -> Self {
        Self {
            line,
            front: None,
            back: None,
            coplanar_front: Vec::new(),
            coplanar_back: Vec::new(),
        }
    }

    fn lines_amount(&self) -> usize {
        let mut amount = self.coplanar_front.len() + self.coplanar_back.len();
        if let Some(f) = self.front.as_ref().map(|f| f.lines_amount()) {
            amount += f;
        }
        if let Some(f) = self.back.as_ref().map(|f| f.lines_amount()) {
            amount += f;
        }
        amount
    }

    pub fn union(self, other: Self) -> Self {
        let (_front1, mut back1) = self.clone().clip_by(&other);
        let (_front2, back2) = other.clone().clip_by(&self);

        back1.merge_tree(back2);
        back1
    }
    pub fn diff(self, other: Self) -> Self {
        let (_front1, mut back1) = self.clone().clip_by(&other);
        let (front2, _back2) = other.clone().clip_by(&self);
        dbg!(front2.clone().invert().into_iter().collect_vec());
        dbg!(back1.clone().into_iter().collect_vec());

        back1.merge_tree(front2.invert());
        back1
    }

    pub(crate) fn merge_tree(&mut self, mut other: Self) {
        let mut lines = other.coplanar_front;
        lines.append(&mut other.coplanar_back);
        let mut results = self.clip_segments(lines);

        self.coplanar_front.append(&mut results.coplanar_front);
        self.coplanar_back.append(&mut results.coplanar_back);

        if let Some(front) = self.front.as_mut() {
            front.join_segments(results.front);
        } else {
            self.front = Self::build(results.front).map(Box::new);
        }
        if let Some(back) = self.back.as_mut() {
            back.join_segments(results.back);
        } else {
            self.back = Self::build(results.back).map(Box::new);
        }

        if let Some(other_front) = other.front.take() {
            self.merge_tree(*other_front);
        }
        if let Some(other_back) = other.back.take() {
            self.merge_tree(*other_back);
        }
    }

    pub(crate) fn join_segments(&mut self, segments: Vec<Segment2D>) {
        let mut splitted = self.clip_segments(segments);

        self.coplanar_front.append(&mut splitted.coplanar_front);
        self.coplanar_back.append(&mut splitted.coplanar_front);
        if let Some(front) = self.front.as_mut() {
            front.join_segments(splitted.front);
        } else {
            self.front = Self::build(splitted.front).map(Box::new);
        }

        if let Some(back) = self.back.as_mut() {
            back.join_segments(splitted.back);
        } else {
            self.back = Self::build(splitted.back).map(Box::new);
        }
    }

    pub(crate) fn clip_by(self, mm: &Bsp2d) -> (Self, Self) {
        let Self {
            coplanar_front,
            mut coplanar_back,
            mut front,
            mut back,
            line,
        } = self;
        let mut segments = coplanar_front;
        segments.append(&mut coplanar_back);
        let mut results = mm.clip_segments(segments);
        let mut front_tree = Self::new(line.clone());
        let mut back_tree = Self::new(line);

        front_tree.coplanar_front = results.front;
        front_tree.coplanar_back = results.coplanar_back;

        back_tree.coplanar_back = results.back;
        back_tree.coplanar_front.append(&mut results.coplanar_front);
        //back_tree.coplanar_back.append(&mut results.coplanar_back);

        if let Some(tree) = front.take() {
            let (fftree, bbtree) = tree.clip_by(mm);
            front_tree.front = Some(Box::new(fftree));
            back_tree.front = Some(Box::new(bbtree));
        }
        if let Some(tree) = back.take() {
            let (fftree, bbtree) = tree.clip_by(mm);
            front_tree.back = Some(Box::new(fftree));
            back_tree.back = Some(Box::new(bbtree));
        }
        (front_tree, back_tree)
    }

    pub(crate) fn invert(mut self) -> Self {
        let coplanar_back = self.coplanar_front.into_iter().map(|f| f.flip()).collect();
        let coplanar_front = self.coplanar_back.into_iter().map(|f| f.flip()).collect();
        self.coplanar_back = coplanar_back;
        self.coplanar_front = coplanar_front;
        self.line = self.line.flip();
        let back = self.front.take().map(|tree| Box::new(tree.invert()));
        let front = self.back.take().map(|tree| Box::new(tree.invert()));

        self.front = front;
        self.back = back;

        self
    }

    fn clip_segments(&self, lines: Vec<Segment2D>) -> SplitResult {
        let (front, back, coplanar_front, coplanar_back) = lines
            .into_iter()
            .map(|segment| self.line.split_segment(segment))
            .fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |(mut front, mut back, mut coplanar_front, mut coplanar_back), mut split| {
                    front.append(&mut split.front);
                    back.append(&mut split.back);
                    coplanar_front.append(&mut split.coplanar_front);
                    coplanar_back.append(&mut split.coplanar_back);

                    (front, back, coplanar_front, coplanar_back)
                },
            );

        let mut split_result = SplitResult::default();

        if let Some(tree) = self.front.as_ref() {
            let result = tree.clip_segments(front);
            split_result = split_result
                .fronts(result.front)
                .backs(result.back)
                .coplanar_fronts(result.coplanar_front)
                .coplanar_backs(result.coplanar_back);

            let result = tree.clip_segments(coplanar_front);
            split_result = split_result
                .fronts(result.front)
                .backs(result.back)
                .coplanar_fronts(result.coplanar_front)
                .coplanar_backs(result.coplanar_back);
        } else {
            split_result = split_result.fronts(front).coplanar_fronts(coplanar_front);
        }
        if let Some(tree) = self.back.as_ref() {
            let result = tree.clip_segments(back);
            split_result = split_result
                .fronts(result.front)
                .backs(result.back)
                .coplanar_fronts(result.coplanar_front)
                .coplanar_backs(result.coplanar_back);
            let result = tree.clip_segments(coplanar_back);
            split_result = split_result
                .fronts(result.front)
                .backs(result.back)
                .coplanar_fronts(result.coplanar_front)
                .coplanar_backs(result.coplanar_back);
        } else {
            split_result = split_result.backs(back).coplanar_backs(coplanar_back);
        }
        split_result
    }
}

pub struct LinesBspIterator<I>
where
    I: Iterator<Item = Segment2D>,
{
    len: usize,
    inner: I,
}
impl<I> Iterator for LinesBspIterator<I>
where
    I: Iterator<Item = Segment2D>,
{
    type Item = Segment2D;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl IntoIterator for Bsp2d {
    type Item = Segment2D;

    type IntoIter = LinesBspIterator<Box<dyn Iterator<Item = Segment2D>>>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.lines_amount();
        let mut my: Box<dyn Iterator<Item = Segment2D>> =
            Box::new(self.coplanar_front.into_iter().chain(self.coplanar_back));
        if let Some(fronts) = self.front {
            my = Box::new(my.chain(fronts.into_iter()));
        }
        if let Some(backs) = self.back {
            my = Box::new(my.chain(backs.into_iter()));
        }

        LinesBspIterator {
            inner: Box::new(my),
            len,
        }
    }
}

impl Bsp2d {
    pub fn merge(&mut self, other: impl IntoIterator<Item = Segment2D>) {
        let (front, back, mut coplanar_front, mut coplanar_back) =
            other.into_iter().map(|f| self.line.split_segment(f)).fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |(mut front, mut back, mut coplanar_front, mut coplanar_back), mut split| {
                    front.append(&mut split.front);
                    back.append(&mut split.back);
                    coplanar_front.append(&mut split.coplanar_front);
                    coplanar_back.append(&mut split.coplanar_back);

                    (front, back, coplanar_front, coplanar_back)
                },
            );
        if !front.is_empty() {
            if let Some(tree) = self.front.as_mut() {
                tree.merge(front);
            } else {
                self.front = Self::build(front).map(Box::new);
            }
        }
        if !back.is_empty() {
            if let Some(tree) = self.back.as_mut() {
                tree.merge(back);
            } else {
                self.back = Self::build(back).map(Box::new);
            }
        }
        self.coplanar_back.append(&mut coplanar_back);
        self.coplanar_front.append(&mut coplanar_front);
    }

    pub fn build(polygon: impl IntoIterator<Item = Segment2D>) -> Option<Self> {
        let mut iter = polygon.into_iter();
        let segment = iter.next();
        segment.and_then(|segment| {
            let line = segment.get_line();
            let (front, back, coplanar_front, coplanar_back) =
                iter.map(|f| line.split_segment(f)).fold(
                    (Vec::new(), Vec::new(), vec![segment], Vec::new()),
                    |(mut front, mut back, mut coplanar_front, mut coplanar_back), mut split| {
                        front.append(&mut split.front);
                        back.append(&mut split.back);
                        coplanar_front.append(&mut split.coplanar_front);
                        coplanar_back.append(&mut split.coplanar_back);

                        (front, back, coplanar_front, coplanar_back)
                    },
                );
            let front = if front.is_empty() {
                None
            } else {
                Some(Box::new(Bsp2d::build(front)?))
            };
            let back = if back.is_empty() {
                None
            } else {
                Some(Box::new(Bsp2d::build(back)?))
            };

            Some(Self {
                line,
                front,
                back,
                coplanar_front,
                coplanar_back,
            })
        })
    }
}
