//! A vector implemented on a trie. Unlike standard vector does not support insertion and removal
//! of an element results in the last element being placed in the empty position.
use std::marker::PhantomData;

use crate::*;

/// An iterable implementation of vector that stores its content on the trie.
/// Uses the following map: index -> element.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Vector<T> {
    pub len: u64,
    pub prefix: Vec<u8>,
    #[borsh_skip]
    el: PhantomData<T>,

    #[borsh_skip]
    pub data: Vec<T>,
}

impl<T> Vector<T> {
    /// Returns the number of elements in the vector, also referred to as its size.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Create new vector with zero elements. Use `id` as a unique identifier on the trie.
    pub fn new(id: Vec<u8>) -> Self {
        Self {
            len: 0,
            prefix: id,
            el: PhantomData,
            data: Vec::new(),
        }
    }

    pub fn index_to_lookup_key(&self, index: u64) -> Vec<u8> {
        append_slice(&self.prefix, &index.to_le_bytes()[..])
    }
}

impl<T> Vector<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn parse(&mut self, state: &mut State) {
        let values: Vec<_> = (0..self.len)
            .map(|i| {
                let key = self.index_to_lookup_key(i);
                T::try_from_slice(&state.remove(&key).unwrap()).unwrap()
            })
            .collect();
        self.data.extend(values);
    }
}
