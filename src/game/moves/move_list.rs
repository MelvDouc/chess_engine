use std::{
    ops::{Index, IndexMut},
    slice,
};

use crate::{
    game::moves::{Move, NULL_MOVE},
    macros::const_while,
};

const MAX_MOVES: usize = 255;

pub(crate) struct MoveList {
    moves: [Move; MAX_MOVES],
    len: usize,
}

impl MoveList {
    pub(crate) const fn new() -> Self {
        Self {
            moves: [NULL_MOVE; MAX_MOVES],
            len: 0,
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) const fn contains(&self, mv: Move) -> bool {
        const_while!(i, 0, self.len, {
            if self.moves[i] == mv {
                return true;
            }
        });

        false
    }

    pub(crate) const fn push(&mut self, mv: Move) {
        self.moves[self.len] = mv;
        self.len += 1;
    }

    pub(crate) const fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.moves[i];
        self.moves[i] = self.moves[j];
        self.moves[j] = tmp;
    }

    pub(crate) fn retain(&mut self, mut predicate: impl FnMut(Move) -> bool) {
        let mut j = 0;

        const_while!(i, 0, self.len, {
            if predicate(self.moves[i]) {
                self.moves[j] = self.moves[i];
                j += 1;
            }
        });

        self.len = j;
    }

    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        &mut self.moves[..self.len]
    }
}

impl Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, i: usize) -> &Self::Output {
        &self.moves[i]
    }
}

impl IndexMut<usize> for MoveList {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.moves[i]
    }
}
impl<'a> IntoIterator for &'a MoveList {
    type Item = &'a Move;
    type IntoIter = slice::Iter<'a, Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves[..self.len].iter()
    }
}

impl<'a> IntoIterator for &'a mut MoveList {
    type Item = &'a mut Move;
    type IntoIter = slice::IterMut<'a, Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves[..self.len].iter_mut()
    }
}
