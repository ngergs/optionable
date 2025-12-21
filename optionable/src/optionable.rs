macro_rules! import_std_or_alloc {
    ($($path:ident)::* :: {$($id:ident $(as $alias:ident)?),* $(,)?}) => {
        #[cfg(all(feature = "alloc", not(feature = "std")))]
        use alloc::$($path)::*::{$($id $(as $alias)?),*};
        #[cfg(feature = "std")]
        use std::$($path)::*::{$($id $(as $alias)?),*};
    };
}

use crate::{Error, Optionable, OptionableConvert};
use core::cell::{Cell, RefCell};
#[cfg(any(feature = "alloc", feature = "std"))]
use core::fmt::Debug;
import_std_or_alloc!(boxed::{Box});
import_std_or_alloc!(borrow::{Cow, ToOwned});
import_std_or_alloc!(vec::{Vec});
import_std_or_alloc!(collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque});
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
import_std_or_alloc!(string::{String});
#[cfg(feature = "std")]
use std::ffi::{OsStr, OsString};
#[cfg(feature = "std")]
use std::hash::{BuildHasher, Hash};
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
import_std_or_alloc!(rc::{Rc, Weak as RcWeak});
import_std_or_alloc!(sync::{Arc, Weak as ArcWeak});
#[cfg(feature = "std")]
use std::sync::{Mutex, RwLock};
#[cfg(feature = "std")]
use std::time::Duration;

// Blanket implementation for references to `Optionable` types.
impl<'a, T: ?Sized + Optionable> Optionable for &'a T {
    type Optioned = &'a T::Optioned;
}

// Blanket implementation for mut references to `Optionable` types.
impl<'a, T: ?Sized + Optionable> Optionable for &'a mut T {
    type Optioned = &'a mut T::Optioned;
}

/// Helper macro to generate an impl for `Optionalable` where the `Optioned` type
/// resolves to itself for types without inner structure like primitives (e.g. `i32`).
macro_rules! impl_optional_self {
    ($($(#[$attr:meta])? $t:ty),+ $(,)?) => {
        $(
            $(#[$attr])?
            impl crate::Optionable for $t{
                type Optioned = Self;
            }

            $(#[$attr])?
            impl crate::OptionableConvert for $t{
                fn into_optioned(self) -> $t {
                    self
                }

                fn try_from_optioned(value: Self::Optioned) -> Result<Self, crate::Error> {
                    Ok(value)
                }

                fn merge(&mut self, other: Self::Optioned) -> Result<(), crate::Error> {
                    *self = other;
                    Ok(())
                }
        })*
    };
}
#[cfg(any(feature = "chrono04", feature = "jiff02", feature = "serde_json"))]
pub(crate) use impl_optional_self;

/// Only implements `Optionable`, not `OptionableConvert`
macro_rules! impl_optional_self_unsized {
    ($($(#[$attr:meta])? $t:ty),+ $(,)?) => {
        $(
            $(#[$attr])?
            impl crate::Optionable for $t{
                type Optioned = Self;
        })*
    }
}

impl_optional_self!(
    // Rust primitives don't have inner structure, https://doc.rust-lang.org/rust-by-example/primitives.html
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    char,
    bool,
    // Other types without inner structure
    (),
    #[cfg(any(feature = "alloc", feature = "std"))]
    String,
    #[cfg(feature = "std")]
    OsString,
    #[cfg(feature = "std")]
    Duration,
    #[cfg(feature = "std")]
    PathBuf
);

impl_optional_self_unsized!(
    str,
    #[cfg(feature = "std")]
    OsStr,
    #[cfg(feature = "std")]
    Path
);

/// Helper macro to generate an impl for `Optionable` for Containers.
/// Containers can be made optional by getting a corresponding container over the associated optional type.
macro_rules! impl_container{
    ($($(#[$attr:meta])? $t:ident),* $(,)?) => {
        $(
            $(#[$attr])?
            impl<T: Optionable> Optionable for $t<T>
                where T::Optioned: Sized
            {
                type Optioned = $t<T::Optioned>;
        })*
    };
}

/// Helper macro to generate an impl for `Optionable` for Containers that do not require the wrapped value to be sized.
/// Containers can be made optional by getting a corresponding container over the associated optional type.
macro_rules! impl_container_unsized {
    ($($(#[$attr:meta])? $t:ident),* $(,)?) => {
        $(
            $(#[$attr])?
            impl<T: Optionable> Optionable for $t<T>
            {
                type Optioned = $t<T::Optioned>;
        })*
    };
}

/// Static macro to hold the inner impl (i.e. the code inside the impl{...}) for an `IntoIterator` type
#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! inner_impl_convert_into_iter {
    ($t:ty) => {
        fn into_optioned(self) -> <$t as Optionable>::Optioned {
            self.into_iter().map(T::into_optioned).collect()
        }

        fn try_from_optioned(value: <$t as Optionable>::Optioned) -> Result<Self, Error> {
            value.into_iter().map(T::try_from_optioned).collect()
        }

        fn merge(&mut self, other: <$t as Optionable>::Optioned) -> Result<(), Error> {
            *self = Self::try_from_optioned(other)?;
            Ok(())
        }
    };
}

/// Helper macro to generate an impl for `OptionableConvert` for Containers with linear structure (e.g. `Vec`).
/// Automatically adds to the impl #[cfg(any(feature = "alloc", feature = "std"))]
#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! impl_container_convert_linear {
    ($($t:ident),+ $(, where=$w:ident)?) => {
        $(
            #[cfg(any(feature = "alloc", feature = "std"))]
            impl<T: OptionableConvert> OptionableConvert for $t<T>
            where T::Optioned: Sized{
            inner_impl_convert_into_iter!($t<T>);
        })*
    };
}

/// Helper macro to generate an impl for `OptionableConvert` for Containers with linear structure that require `cmp:Ord` (e.g. `BTreeSet`).
/// Automatically adds to the impl #[cfg(any(feature = "alloc", feature = "std"))]
#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! impl_container_convert_linear_ord {
    ($($t:ident),+ $(, where=$w:ident)?) => {
        $(
            #[cfg(any(feature = "alloc", feature = "std"))]
            impl<T: OptionableConvert> OptionableConvert for $t<T>
            where T: Ord,
                  T::Optioned: Sized+Ord{
            inner_impl_convert_into_iter!($t<T>);
        })*
    };
}

impl_container!(
    Option,
    #[cfg(any(feature = "alloc", feature = "std"))]
    Vec,
    #[cfg(any(feature = "alloc", feature = "std"))]
    VecDeque,
    #[cfg(any(feature = "alloc", feature = "std"))]
    LinkedList,
    #[cfg(any(feature = "alloc", feature = "std"))]
    BTreeSet,
    #[cfg(any(feature = "alloc", feature = "std"))]
    BinaryHeap
);
impl_container_unsized!(
    Cell,
    RefCell,
    #[cfg(any(feature = "alloc", feature = "std"))]
    Box,
    #[cfg(any(feature = "alloc", feature = "std"))]
    Rc,
    #[cfg(any(feature = "alloc", feature = "std"))]
    RcWeak,
    #[cfg(any(feature = "alloc", feature = "std"))]
    Arc,
    #[cfg(any(feature = "alloc", feature = "std"))]
    ArcWeak,
    #[cfg(feature = "std")]
    Mutex,
    #[cfg(feature = "std")]
    RwLock,
);
#[cfg(any(feature = "alloc", feature = "std"))]
impl_container_convert_linear!(Vec, VecDeque, LinkedList);
#[cfg(any(feature = "alloc", feature = "std"))]
impl_container_convert_linear_ord!(BTreeSet, BinaryHeap);

impl<T: Optionable, const N: usize> Optionable for [T; N]
where
    T::Optioned: Sized,
{
    type Optioned = [T::Optioned; N];
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: OptionableConvert, const N: usize> OptionableConvert for [T; N]
where
    T: Debug,
    T::Optioned: Sized,
{
    fn into_optioned(self) -> Self::Optioned {
        self.map(T::into_optioned)
    }

    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error> {
        // unwrapping here is safe as it just would error if we would not produce N outputs from N inputs
        Ok(value
            .into_iter()
            .map(T::try_from_optioned)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap())
    }

    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error> {
        *self = Self::try_from_optioned(other)?;
        Ok(())
    }
}

impl<T: Optionable> Optionable for [T]
where
    T::Optioned: Sized,
{
    type Optioned = [T::Optioned];
}

impl<T: OptionableConvert> OptionableConvert for Option<T>
where
    T::Optioned: Sized,
{
    fn into_optioned(self) -> Option<T::Optioned> {
        self.map(T::into_optioned)
    }

    fn try_from_optioned(value: Option<T::Optioned>) -> Result<Self, Error> {
        value.map(T::try_from_optioned).transpose()
    }

    fn merge(&mut self, other: Option<T::Optioned>) -> Result<(), Error> {
        if let Some(val) = self {
            if let Some(val_other) = other {
                val.merge(val_other)?;
            }
        } else if let Some(val_other) = other {
            *self = Some(T::try_from_optioned(val_other)?);
        }
        Ok(())
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a, T: ?Sized + Optionable + ToOwned> Optionable for Cow<'a, T>
where
    T::Optioned: ToOwned,
{
    type Optioned = Cow<'a, T::Optioned>;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: OptionableConvert + Clone> OptionableConvert for Cow<'_, T>
where
    T::Optioned: Clone,
{
    fn into_optioned(self) -> Self::Optioned {
        Cow::Owned(self.into_owned().into_optioned())
    }

    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error> {
        T::try_from_optioned(value.into_owned()).map(Cow::Owned)
    }

    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error> {
        let mut self_owned = self.clone().into_owned();
        let other_owned = other.into_owned();
        self_owned.merge(other_owned)?;
        *self = Cow::Owned(self_owned);
        Ok(())
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: OptionableConvert> OptionableConvert for Box<T>
where
    T::Optioned: Sized,
{
    fn into_optioned(self) -> Box<T::Optioned> {
        let inner = *self;
        Box::new(T::into_optioned(inner))
    }

    fn try_from_optioned(value: Box<T::Optioned>) -> Result<Self, Error> {
        let inner = *value;
        Ok(Box::new(T::try_from_optioned(inner)?))
    }

    fn merge(&mut self, other: Box<T::Optioned>) -> Result<(), Error> {
        let inner = &mut **self;
        let other_inner = *other;
        inner.merge(other_inner)?;
        Ok(())
    }
}

impl<T: Optionable, E> Optionable for Result<T, E>
where
    T::Optioned: Sized,
{
    type Optioned = Result<T::Optioned, E>;
}

#[cfg(feature = "std")]
impl<T: Optionable, S> Optionable for HashSet<T, S>
where
    T::Optioned: Sized,
{
    type Optioned = HashSet<T::Optioned, S>;
}

#[cfg(feature = "std")]
impl<T: OptionableConvert, S: Default + BuildHasher> OptionableConvert for HashSet<T, S>
where
    T: Ord + Hash,
    T::Optioned: Sized + Eq + Hash,
{
    inner_impl_convert_into_iter!(HashSet<T,S>);
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<K, T: Optionable> Optionable for BTreeMap<K, T>
where
    T::Optioned: Sized,
{
    type Optioned = BTreeMap<K, T::Optioned>;
}

/// Static macro to hold the inner impl for map-like types
#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! inner_impl_convert_map {
    ($t:ty) => {
        fn into_optioned(self) -> $t {
            self.into_iter()
                .map(|(k, v)| (k, T::into_optioned(v)))
                .collect()
        }

        fn try_from_optioned(value: $t) -> Result<Self, Error> {
            value
                .into_iter()
                .map(|(k, v)| Ok((k, T::try_from_optioned(v)?)))
                .collect()
        }

        fn merge(&mut self, other: $t) -> Result<(), Error> {
            other.into_iter().try_for_each(|(k, v)| {
                self.insert(k, T::try_from_optioned(v)?);
                Ok(())
            })
        }
    };
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<K: Ord, T: OptionableConvert> OptionableConvert for BTreeMap<K, T>
where
    T::Optioned: Sized,
{
    inner_impl_convert_map!(BTreeMap<K, T::Optioned>);
}

#[cfg(feature = "std")]
impl<K, T: Optionable, S> Optionable for HashMap<K, T, S>
where
    T::Optioned: Sized,
{
    type Optioned = HashMap<K, T::Optioned, S>;
}

#[cfg(feature = "std")]
impl<K: Ord + Hash, T: OptionableConvert, S: BuildHasher + Default> OptionableConvert
    for HashMap<K, T, S>
where
    T::Optioned: Sized,
{
    inner_impl_convert_map!(HashMap<K, T::Optioned, S>);
}
macro_rules! impl_tuple {
    // For tuples of length 2: (T1, T2)
    ($(($T:ident, $i:tt)),*) => {
        impl<$($T),*> Optionable for ($($T),*)
        where
            $(
                $T: Optionable,
                $T::Optioned: Sized
            ),*
        {
            type Optioned = ($(Option::<$T::Optioned>),*);
        }

        impl<$($T),*> OptionableConvert for ($($T),*)
        where
            $(
                $T: OptionableConvert,
                $T::Optioned: Sized
            ),*
        {
            fn into_optioned(self) -> Self::Optioned {
                ($(Some(self.$i.into_optioned())),*)
            }

            fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error> {
                Ok((
                    $($T::try_from_optioned(value.$i.ok_or(Error { missing_field: "$i" })?)?),*
                ))
            }

            fn merge(&mut self, other: Self::Optioned) -> Result<(), Error> {
                $(if let Some(other_value) = other.$i { self.$i.merge(other_value)?; })*
                Ok(())
            }
        }
    };
}

// Tuple implementation...
impl_tuple!((T0, 0), (T1, 1));
impl_tuple!((T0, 0), (T1, 1), (T2, 2));
impl_tuple!((T0, 0), (T1, 1), (T2, 2), (T3, 3));
impl_tuple!((T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4));
impl_tuple!((T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5));
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14)
);
impl_tuple!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14),
    (T15, 15)
);

#[cfg(test)]
mod tests {
    use crate::Optionable;
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use alloc::vec;
    #[cfg(feature = "std")]
    use std::vec;
    import_std_or_alloc!(vec::{Vec});
    import_std_or_alloc!(string::{String});
    import_std_or_alloc!(borrow::{Cow,ToOwned});
    use crate::Error;
    #[cfg(feature = "std")]
    use std::collections::{BTreeMap, HashMap};

    #[test]
    /// Check that an exemplary primitive type like `i32` resolves to itself as `Optioned` type.
    /// As all primitives share the same macro-generated code it does not add any value to iterate through
    /// all of them. If we missed a primitive type at the macro invocation we would also miss it at listing
    /// the types for the test.
    fn primitive_types_optioned_self() {
        let a: i32 = 10;
        let _: <i32 as Optionable>::Optioned = a;
    }

    #[test]
    /// Check that &str implements `Optionable`.
    fn str() {
        let a = "hello";
        let _: <&str as Optionable>::Optioned = a;
    }

    #[test]
    /// Check that pointer to `Optionable` types implement optionable.
    fn ptr() {
        let a = 2;
        let _: <&i32 as Optionable>::Optioned = &a;
    }

    #[test]
    /// Check that an array implements optionable.
    fn array() {
        let a = [1, 2, 3];
        let _: <[i32; 3] as Optionable>::Optioned = a;
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    /// Check that `Vec` implements optionable as an example container.
    fn container() {
        let a = vec![1, 2, 3];
        let _: <Vec<i64> as Optionable>::Optioned = a;
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    /// Check that `Cow` implements optionable.
    fn cow() {
        let a = Cow::Owned("hello".to_owned());
        let _: <Cow<String> as Optionable>::Optioned = a;
        let b: Cow<String> = Cow::Borrowed(&a);
        let _: <Cow<String> as Optionable>::Optioned = b;
        let c: Cow<str> = Cow::Borrowed("hello");
        let _: <Cow<str> as Optionable>::Optioned = c;
    }

    #[test]
    /// Check that `Result` implements optionable.
    fn result() {
        let a = Ok::<_, Error>(42);
        let _: <Result<i32, _> as Optionable>::Optioned = a;
    }

    #[test]
    #[cfg(feature = "std")]
    /// Check that `HashMap` and `BTreeMap` implements optionable.
    fn map() {
        let a = HashMap::from([(1, "a".to_owned())]);
        let _: <HashMap<i32, String> as Optionable>::Optioned = a;

        let a = BTreeMap::from([(1, "a".to_owned())]);
        let _: <BTreeMap<i32, String> as Optionable>::Optioned = a;
    }
}
