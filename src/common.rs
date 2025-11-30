use nalgebra::{Dim, Matrix, Matrix2, Scalar, Storage, Vector2};
use nalgebra::{SVector, Vector3};
use num::rational::Ratio;
use num::{Rational64, Signed};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

pub type Rational128 = Ratio<i128>;
pub type Vec2i = Vector2<i64>;
pub type Vec2r = Vector2<Rational64>;
pub type Vec2r128 = Vector2<Rational128>;
pub type Vec2f = Vector2<f64>;
pub type Vec3i = Vector3<i64>;
pub type Vec3r = Vector3<Rational64>;
pub type Vec3r128 = Vector3<Rational128>;
pub type Mat2i = Matrix2<i64>;
pub type Mat2r = Matrix2<Rational64>;
pub type Mat2r128 = Matrix2<Rational128>;

pub fn lp1_norm<T: Scalar + Signed, R: Dim, C: Dim, S: Storage<T, R, C>>(
    v: &Matrix<T, R, C, S>,
) -> T {
    match v.shape() {
        (1, _) | (_, 1) => v.fold(T::zero(), |a, e| a + e.abs()),
        _ => {
            panic!("lp1 norm is only implemented for vectors")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'N' | 'U' | '^' => Self::North,
            'S' | 'D' | 'v' => Self::South,
            'E' | 'R' | '>' => Self::East,
            'W' | 'L' | '<' => Self::West,
            _ => {
                return Err(());
            }
        })
    }
}

impl Direction {
    pub const VALUES: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    pub fn rotate_cw(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub fn vec(&self) -> Vec2i {
        match self {
            Self::North => Vec2i::new(0, -1),
            Self::South => Vec2i::new(0, 1),
            Self::East => Vec2i::new(1, 0),
            Self::West => Vec2i::new(-1, 0),
        }
    }

    pub fn offset(&self, pos: &Vec2i) -> Vec2i {
        self.offset_with_amount(pos, 1)
    }

    pub fn offset_with_amount(&self, pos: &Vec2i, amount: i64) -> Vec2i {
        pos + amount * self.vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub size_x: usize,
    pub size_y: usize,
    grid: Vec<T>,
}

impl<T> FromStr for Grid<T>
where
    char: TryInto<T>,
{
    type Err = <char as TryInto<T>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut size_x = None;
        let mut size_y = 0;
        let grid = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .flat_map(|l| {
                size_y += 1;
                match size_x {
                    None => size_x = Some(l.len()),
                    Some(size_x) if size_x == l.len() => {}
                    _ => {
                        panic!("non rectangular grid");
                    }
                }
                l.chars().map(char::try_into)
            })
            .collect::<Result<_, _>>()?;
        Ok(Grid {
            size_x: size_x.unwrap(),
            size_y,
            grid,
        })
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_from_element(size_x: usize, size_y: usize, element: T) -> Self {
        Self {
            size_x,
            size_y,
            grid: vec![element; size_x * size_y],
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_from_default(size_x: usize, size_y: usize) -> Self {
        Self::new_from_element(size_x, size_y, T::default())
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, pos: &Vec2i) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.size_x && pos.y >= 0 && (pos.y as usize) < self.size_y
    }

    pub fn mod_get(&self, pos: &Vec2i) -> &T {
        let x = pos.x.rem_euclid(self.size_x as i64);
        let y = pos.y.rem_euclid(self.size_y as i64);
        &self[Vec2i::new(x, y)]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn iter_col(&self, x: i64) -> impl Iterator<Item = &T> {
        (0..self.size_y).map(move |y| &self[Vec2i::new(x, y as i64)])
    }

    pub fn iter_row(&self, y: i64) -> impl Iterator<Item = &T> {
        (0..self.size_x).map(move |x| &self[Vec2i::new(x as i64, y)])
    }

    pub fn pos_iter(&self) -> impl Iterator<Item = (Vec2i, &T)> {
        self.grid.iter().enumerate().map(|(i, t)| {
            (
                Vec2i::new((i % self.size_x) as _, (i / self.size_x) as _),
                t,
            )
        })
    }

    pub fn pos_iter_col(&self, x: i64) -> impl Iterator<Item = (Vec2i, &T)> {
        (0..self.size_y).map(move |y| {
            let pos = Vec2i::new(x, y as i64);
            (pos, &self[pos])
        })
    }

    pub fn pos_iter_row(&self, y: i64) -> impl Iterator<Item = (Vec2i, &T)> {
        (0..self.size_x).map(move |x| {
            let pos = Vec2i::new(x as i64, y);
            (pos, &self[pos])
        })
    }
}

impl<T> Index<Vec2i> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2i) -> &Self::Output {
        &self.grid[(index.x as usize) + self.size_x * (index.y as usize)]
    }
}

impl<T> IndexMut<Vec2i> for Grid<T> {
    fn index_mut(&mut self, index: Vec2i) -> &mut Self::Output {
        &mut self.grid[(index.x as usize) + self.size_x * (index.y as usize)]
    }
}

pub fn parse_split_whitespace<T: FromStr, B: FromIterator<T>>(
    s: &str,
) -> Result<B, <T as FromStr>::Err> {
    s.split_whitespace().map(str::parse).collect()
}

pub fn parse_split<T: FromStr, B: FromIterator<T>>(
    s: &str,
    pat: char,
) -> Result<B, <T as FromStr>::Err> {
    s.split(pat)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

pub fn parse_lines<T: FromStr, B: FromIterator<T>>(s: &str) -> Result<B, <T as FromStr>::Err> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
}

#[derive(Error, Debug)]
pub enum ParseVecError<T> {
    #[error("missing element")]
    MissingElement,
    #[error("too many elements")]
    TooManyElements,
    #[error("parse error")]
    ParseError(#[from] T),
}

pub fn parse_vec<T: Scalar + FromStr, const D: usize>(
    s: &str,
) -> Result<SVector<T, D>, ParseVecError<<T as FromStr>::Err>> {
    let mut it = s
        .trim_matches(|c: char| {
            matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '|') || c.is_whitespace()
        })
        .split(|c: char| matches!(c, ',' | ';' | '|') || c.is_whitespace())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse);

    let mut data: [Option<T>; D] = std::array::from_fn(|_| None);
    for elem in data.iter_mut() {
        *elem = Some(it.next().ok_or(ParseVecError::MissingElement)??);
    }

    if it.next().is_some() {
        return Err(ParseVecError::TooManyElements);
    }

    Ok(SVector::from_iterator(data.into_iter().flatten()))
}
