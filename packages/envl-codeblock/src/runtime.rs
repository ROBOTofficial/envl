use std::{ops::BitOr, str::FromStr};

use proc_macro2::Group;
use quote::TokenStreamExt;

#[doc(hidden)]
pub use alloc::format;
#[doc(hidden)]
pub use core::option::Option;

#[doc(hidden)]
pub type Delimiter = proc_macro2::Delimiter;
#[doc(hidden)]
pub type Span = proc_macro2::Span;
#[doc(hidden)]
pub type TokenStream = proc_macro2::TokenStream;

#[doc(hidden)]
pub struct HasIterator<const B: bool>;

impl BitOr<HasIterator<false>> for HasIterator<false> {
    type Output = HasIterator<false>;
    fn bitor(self, _rhs: HasIterator<false>) -> HasIterator<false> {
        HasIterator::<false>
    }
}

impl BitOr<HasIterator<false>> for HasIterator<true> {
    type Output = HasIterator<true>;
    fn bitor(self, _rhs: HasIterator<false>) -> HasIterator<true> {
        HasIterator::<true>
    }
}

impl BitOr<HasIterator<true>> for HasIterator<false> {
    type Output = HasIterator<true>;
    fn bitor(self, _rhs: HasIterator<true>) -> HasIterator<true> {
        HasIterator::<true>
    }
}

impl BitOr<HasIterator<true>> for HasIterator<true> {
    type Output = HasIterator<true>;
    fn bitor(self, _rhs: HasIterator<true>) -> HasIterator<true> {
        HasIterator::<true>
    }
}

#[doc(hidden)]
pub trait CheckHasIterator<const B: bool>: Sized {
    fn check(self) {}
}

impl CheckHasIterator<true> for HasIterator<true> {}

#[doc(hidden)]
pub fn push_group(tokens: &mut TokenStream, delimiter: Delimiter, inner: TokenStream) {
    tokens.append(Group::new(delimiter, inner));
}

#[doc(hidden)]
pub fn push_ident(tokens: &mut TokenStream, token: String) {
    tokens.extend([TokenStream::from_str(&token).unwrap()]);
}
#[doc(hidden)]
pub mod ext {
    use super::{HasIterator, RepInterp};
    use alloc::collections::btree_set::{self, BTreeSet};
    use core::slice;
    use quote::ToTokens;

    #[doc(hidden)]
    pub trait RepIteratorExt: Iterator + Sized {
        fn quote_into_iter(self) -> (Self, HasIterator<true>) {
            (self, HasIterator::<true>)
        }
    }

    impl<T: Iterator> RepIteratorExt for T {}

    #[doc(hidden)]
    pub trait RepToTokensExt {
        fn next(&self) -> Option<&Self> {
            Some(self)
        }

        fn quote_into_iter(&self) -> (&Self, HasIterator<false>) {
            (self, HasIterator::<false>)
        }
    }

    impl<T: ToTokens + ?Sized> RepToTokensExt for T {}

    #[doc(hidden)]
    pub trait RepAsIteratorExt<'q> {
        type Iter: Iterator;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>);
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &mut T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for [T] {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            (self.iter(), HasIterator::<true>)
        }
    }

    impl<'q, T: 'q, const N: usize> RepAsIteratorExt<'q> for [T; N] {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            (self.iter(), HasIterator::<true>)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for Vec<T> {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            (self.iter(), HasIterator::<true>)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for BTreeSet<T> {
        type Iter = btree_set::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            (self.iter(), HasIterator::<true>)
        }
    }

    impl<'q, T: RepAsIteratorExt<'q>> RepAsIteratorExt<'q> for RepInterp<T> {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIterator<true>) {
            self.0.quote_into_iter()
        }
    }
}

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct RepInterp<T>(pub T);

impl<T> RepInterp<T> {
    pub fn next(self) -> Option<T> {
        Some(self.0)
    }
}

impl<T: Iterator> Iterator for RepInterp<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T: quote::ToTokens> quote::ToTokens for RepInterp<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}
