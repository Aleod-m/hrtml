mod utils {
    use std::fmt;
    use std::ops::Range;

    pub type IStr = std::rc::Rc<str>;
    pub type IVec<T> = std::rc::Rc<[T]>;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Loc {
        pub(crate) pos: usize,
        pub row: usize,
        pub col: usize,
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Span {
        pub start: Loc,
        pub end: Loc,
    }

    impl Span {
        pub fn new(start: Loc, end: Loc) -> Self {
            Self { start, end }
        }

        pub fn is_overlaping(&self, other: Self) -> bool {
            self.start.pos < other.end.pos && self.start.pos > other.start.pos
            || 
            self.end.pos < other.end.pos && self.end.pos > other.start.pos
        }

        pub fn around(a: Self, b: Self) -> Self {
            let start = if a.start.pos < b.start.pos { a } else { b }.start;
            let end = if a.end.pos < b.end.pos { b } else { a }.end;
            Self::new(start, end)
        }

        pub fn inside(a: Self, b: Self) -> Option<Self> {
            if a.is_overlaping(b) {
                return None;
            } else {
                let start = if a.end.pos < b.end.pos { a } else { b }.end;
                let end = if a.start.pos < b.start.pos { b } else { a }.start;
                Some(Self::new(start, end))
            }
        }
    }

    impl Into<Range<usize>> for Span {
        fn into(self) -> Range<usize> {
            self.start.pos..self.end.pos
        }
    }

    impl fmt::Display for Span {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "start: {}\tend: {}", self.start, self.end)
        }
    }

    impl fmt::Display for Loc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.row, self.col)
        }
    }
}
