use crate::{Optionable, OptionableConvert, OptionedConvert};

/// Merges `other` into `target` using Kubernetes-style `set` merge logic.
/// This means that all elements from `other` which are already present in `target` are discarded
/// and the other ones that are missing in `target` get appended.
pub fn merge_set<TARGET, OTHER, T>(target: &mut TARGET, other: OTHER)
where
    TARGET: Extend<T>,
    for<'a> &'a TARGET: IntoIterator<Item = &'a T>,
    OTHER: IntoIterator<Item = T>,
    T: PartialEq,
{
    for el in other {
        if !target.into_iter().any(|el_target| &el == el_target) {
            target.extend(Some(el));
        }
    }
}

/// Trait for `try_merge_map` to check if the map keys for the respective merge candidate are equal.
pub trait MapKeysEq: Optionable {
    /// Whether the keys of two map list merge candidates are equal.
    fn keys_eq(&self, other: &Self::Optioned) -> bool;
}

/// Merges `other` into `target` using Kubernetes-style `map` merge logic.
/// This means that all elements from `other` which are already present in `target` are merged using `OptionableConvert::merge`
/// and the other ones that are missing in `target` get appended.
/// The merge logic short circuits, i.e. only the first key match from `MapKeysEq` gets its content merged.
///
/// # Errors
/// - When merging into an existing entry via `OptionableConvert::merge_into` fails.
/// - When appending (creating a full type from an optioned one) via `OptionableConvert::try_into_optionable` fails.
pub fn try_merge_optioned_map<TARGET, OTHER, T>(
    target: &mut TARGET,
    other: OTHER,
) -> Result<(), crate::Error>
where
    TARGET: Extend<T>,
    for<'a> &'a mut TARGET: IntoIterator<Item = &'a mut T>,
    OTHER: IntoIterator<Item = T::Optioned>,
    T: OptionableConvert + MapKeysEq,
    T::Optioned: OptionedConvert<T>,
{
    for el in other {
        if let Some(el_target) = target.into_iter().find(|el_target| el_target.keys_eq(&el)) {
            el.merge_into(el_target)?;
        } else {
            target.extend(Some(el.try_into_optionable()?));
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn merge_set() {
        let mut target = vec![0, 1, 2];
        super::merge_set(&mut target, vec![3]);
        assert_eq!(&target, &vec![0, 1, 2, 3]);
        super::merge_set(&mut target, vec![0, 2, 4, 6, 8]);
        assert_eq!(&target, &vec![0, 1, 2, 3, 4, 6, 8]);
        super::merge_set(&mut target, vec![0, 5, 9]);
        assert_eq!(&target, &vec![0, 1, 2, 3, 4, 6, 8, 5, 9]);
    }

    // `try_merge_map` is tested in ../tests/try_merge_map.rs
}
