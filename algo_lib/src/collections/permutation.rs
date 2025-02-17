use crate::collections::iter_ext::IterExt;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::mem::MaybeUninit;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Permutation {
    p: Vec<usize>,
    base: usize,
}

impl Permutation {
    pub fn new(p: Vec<usize>) -> Self {
        Self::new_with_base(p, 0)
    }

    pub fn new_with_base(mut p: Vec<usize>, base: usize) -> Self {
        if base != 0 {
            p.iter_mut().for_each(|a| *a -= base);
        }
        debug_assert!(Self::check(&p));
        Self { p, base }
    }

    pub fn new_ident(size: usize) -> Self {
        Self::new_ident_with_base(size, 0)
    }

    pub fn new_ident_with_base(size: usize, base: usize) -> Self {
        Self {
            p: (0..size).collect_vec(),
            base,
        }
    }

    pub fn inv(&self) -> Self {
        let size = self.p.len();
        let res = unsafe {
            let mut res = MaybeUninit::new(Vec::with_capacity(size));
            (*res.as_mut_ptr()).set_len(size);
            for i in 0..size {
                let ptr: *mut usize = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(self.p[i]).write(i);
            }
            res.assume_init()
        };
        Self {
            p: res,
            base: self.base,
        }
    }

    pub fn set_base(&mut self, b: usize) {
        self.base = b;
    }

    pub fn len(&self) -> usize {
        self.p.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        // edition 2021
        self.p.iter().map(move |i| *i + self.base)
    }

    fn check(p: &[usize]) -> bool {
        let mut was = vec![false; p.len()];
        for i in p {
            if was[*i] {
                return false;
            }
            was[*i] = true;
        }
        true
    }
}

impl PartialEq<Self> for Permutation {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for Permutation {}

pub trait PermutationRead {
    fn read_permutation(&mut self, size: usize) -> Permutation {
        self.read_permutation_with_base(size, 1)
    }
    fn read_permutation_with_base(&mut self, size: usize, base: usize) -> Permutation;
}

impl PermutationRead for Input<'_> {
    fn read_permutation_with_base(&mut self, size: usize, base: usize) -> Permutation {
        Permutation::new_with_base(self.read_vec(size), base)
    }
}

impl Readable for Permutation {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_permutation(size)
    }
}

impl Writable for Permutation {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.p.iter().map(|i| *i + self.base));
    }
}

impl Mul for &Permutation {
    type Output = Permutation;

    fn mul(self, rhs: Self) -> Self::Output {
        let size = self.p.len();
        let mut res = Vec::with_capacity(size);
        for i in 0..size {
            res.push(self.p[rhs.p[i]]);
        }
        Permutation {
            p: res,
            base: self.base,
        }
    }
}
