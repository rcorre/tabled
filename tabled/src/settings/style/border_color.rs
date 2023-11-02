//! This module contains a configuration of a Border to set its color via [`BorderColor`].

use core::marker::PhantomData;

use crate::{
    grid::{
        config::{Border as GridBorder, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::{style::On, CellOption, Color, TableOption},
};

/// Border represents a border color of a Cell.
///
/// ```text
///                         top border
///                             |
///                             V
/// corner top left ------> +_______+  <---- corner top left
///                         |       |
/// left border ----------> |  cell |  <---- right border
///                         |       |
/// corner bottom right --> +_______+  <---- corner bottom right
///                             ^
///                             |
///                        bottom border
/// ```
///
/// # Example
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{Modify, style::{Style, Border}, object::Rows}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BorderColor<T, B, L, R> {
    inner: GridBorder<Color>,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
}

impl<T, B, L, R> BorderColor<T, B, L, R> {
    pub(crate) const fn from_border(inner: GridBorder<Color>) -> BorderColor<T, B, L, R> {
        BorderColor {
            inner,
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
        }
    }
}

impl BorderColor<(), (), (), ()> {
    /// Creates an empty border.
    pub const fn new() -> Self {
        Self::from_border(GridBorder::empty())
    }
}

impl BorderColor<On, On, On, On> {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
        top: Color,
        bottom: Color,
        left: Color,
        right: Color,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    ) -> Self {
        Self::from_border(GridBorder::full(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ))
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub fn filled(c: Color) -> Self {
        Self::full(
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c,
        )
    }
}

impl<T, B, L, R> BorderColor<T, B, L, R> {
    /// Set a top border character.
    pub fn top(mut self, c: Color) -> BorderColor<On, B, L, R> {
        self.inner.top = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: Color) -> BorderColor<T, On, L, R> {
        self.inner.bottom = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a left border character.
    pub fn left(mut self, c: Color) -> BorderColor<T, B, On, R> {
        self.inner.left = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a right border character.
    pub fn right(mut self, c: Color) -> BorderColor<T, B, L, On> {
        self.inner.right = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Converts a border into a general data structure.
    pub fn into_inner(self) -> GridBorder<Color> {
        self.inner
    }
}

impl<B, R> BorderColor<On, B, On, R> {
    /// Set a top left intersection character.
    pub fn corner_top_left(mut self, c: Color) -> Self {
        self.inner.left_top_corner = Some(c);
        self
    }
}

impl<B, L> BorderColor<On, B, L, On> {
    /// Set a top right intersection character.
    pub fn corner_top_right(mut self, c: Color) -> Self {
        self.inner.right_top_corner = Some(c);
        self
    }
}

impl<T, R> BorderColor<T, On, On, R> {
    /// Set a bottom left intersection character.
    pub fn corner_bottom_left(mut self, c: Color) -> Self {
        self.inner.left_bottom_corner = Some(c);
        self
    }
}

impl<T, L> BorderColor<T, On, L, On> {
    /// Set a bottom right intersection character.
    pub fn corner_bottom_right(mut self, c: Color) -> Self {
        self.inner.right_bottom_corner = Some(c);
        self
    }
}

impl<T, B, L, R> From<BorderColor<T, B, L, R>> for GridBorder<Color> {
    fn from(value: BorderColor<T, B, L, R>) -> Self {
        value.inner
    }
}

impl<Data, T, B, L, R> CellOption<Data, ColoredConfig> for BorderColor<T, B, L, R>
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = self.inner.clone().convert();

        for pos in entity.iter(count_rows, count_columns) {
            cfg.set_border_color(pos, border_color.clone());
        }
    }
}

impl<Data, D, T, B, L, R> TableOption<Data, D, ColoredConfig> for BorderColor<T, B, L, R>
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = self.inner.clone().convert();

        for row in 0..count_rows {
            for col in 0..count_columns {
                cfg.set_border_color((row, col), border_color.clone());
            }
        }
    }
}
