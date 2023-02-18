use std::{collections::HashMap, hash::BuildHasher};

use crate::{Cell, CellIndex};
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("Unsigned integer underflow ocurred in cell index")]
    CellIndexUnderflow,
    #[error("Unsigned integer overflow ocurred in cell index")]
    CellIndexOverflow,
    #[error("Could not access cell at index: {0}")]
    CellAccessError(usize),
}

pub trait Memory {
    fn init() -> Self;
    fn modify<F>(&mut self, index: usize, op: F) -> Result<(), Error>
    where
        F: Fn(Cell) -> Cell;
    fn move_left(&mut self, index: CellIndex) -> Result<CellIndex, Error>;
    fn move_right(&mut self, index: CellIndex) -> Result<CellIndex, Error>;
    fn cell_value(&self, index: CellIndex) -> Result<Cell, Error>;
}

impl<S> Memory for HashMap<usize, Cell, S>
where
    S: BuildHasher + Default,
{
    #[inline]
    fn init() -> Self {
        Self::from_iter([(0, 0)])
    }

    fn move_left(&mut self, index: usize) -> Result<usize, Error> {
        let cell_index = index.wrapping_sub(1);
        self.entry(cell_index).or_insert(0);
        Ok(cell_index)
    }

    fn move_right(&mut self, index: usize) -> Result<usize, Error> {
        let cell_index = index.wrapping_add(1);
        self.entry(cell_index).or_insert(0);
        Ok(cell_index)
    }

    fn modify<F>(&mut self, index: usize, op: F) -> Result<(), Error>
    where
        F: Fn(Cell) -> Cell,
    {
        let cell_val = self
            .get_mut(&index)
            .map_or_else(|| Err(Error::CellAccessError(index)), Ok)?;
        *cell_val = op(*cell_val);
        Ok(())
    }

    fn cell_value(&self, index: CellIndex) -> Result<Cell, Error> {
        self.get(&index)
            .map_or_else(|| Err(Error::CellAccessError(index)), Ok)
            .copied()
    }
}

impl Memory for Vec<Cell> {
    #[inline]
    fn init() -> Self {
        vec![0]
    }

    fn move_left(&mut self, index: usize) -> Result<usize, Error> {
        let cell_index = index
            .checked_sub(1)
            .map_or_else(|| Err(Error::CellIndexUnderflow), Ok)?;
        Ok(cell_index)
    }

    fn move_right(&mut self, index: usize) -> Result<usize, Error> {
        let cell_index = index.wrapping_add(1);
        if cell_index == self.len() {
            self.push(0);
        }
        Ok(cell_index)
    }

    fn modify<F>(&mut self, index: usize, op: F) -> Result<(), Error>
    where
        F: Fn(Cell) -> Cell,
    {
        let cell_val = self
            .get_mut(index)
            .map_or_else(|| Err(Error::CellAccessError(index)), Ok)?;
        *cell_val = op(*cell_val);
        Ok(())
    }

    fn cell_value(&self, index: CellIndex) -> Result<Cell, Error> {
        self.get(index)
            .map_or_else(|| Err(Error::CellAccessError(index)), Ok)
            .copied()
    }
}
