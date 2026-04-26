use std::collections::BTreeMap;

use k8s_openapi027::DeepMerge;

use crate::merge::MapKeysEq;

/// Merges `other` into `target` by calling `DeepMerge::merge_from` for entries existing in both.
/// Entires only existing in `other` are inserted into `target`.
pub fn merge_granular<K, V>(target: &mut BTreeMap<K, V>, other: BTreeMap<K, V>)
where
    K: Ord,
    V: DeepMerge,
{
    other.into_iter().for_each(|(k, other_v)| {
        if let Some(target_v) = target.get_mut(&k) {
            target_v.merge_from(other_v);
        } else {
            target.insert(k, other_v);
        }
    });
}

/// Merges `other` into `target` by calling `DeepMerge::merge_from` for entries existing in both.
/// Entires only existing in `other` are inserted into `target`.
/// Handles the Option wrapping the respective type.
pub fn merge_granular_option_wrapped<K, V>(
    target: &mut Option<BTreeMap<K, V>>,
    other: Option<BTreeMap<K, V>>,
) where
    K: Ord,
    V: DeepMerge,
{
    if let Some(other) = other {
        if let Some(target) = target {
            merge_granular(target, other);
        } else {
            *target = Some(other.into());
        }
    }
}

/// Merges `other` into `target` using Kubernetes-style `map` merge logic.
/// This means that all elements from `other` which are already present in `target` are merged using `OptionableConvert::merge`
/// and the other ones that are missing in `target` get appended.
/// The merge logic short circuits, i.e. only the first key match from `MapKeysEq` gets its content merged.
pub fn merge_map<TARGET, OTHER, T>(target: &mut TARGET, other: OTHER)
where
    TARGET: Extend<T>,
    for<'a> &'a mut TARGET: IntoIterator<Item = &'a mut T>,
    OTHER: IntoIterator<Item = T>,
    T: DeepMerge + MapKeysEq,
{
    for el in other {
        if let Some(el_target) = target.into_iter().find(|el_target| el_target.keys_eq(&el)) {
            el_target.merge_from(el);
        } else {
            target.extend(Some(el));
        }
    }
}

/// Merges `other` into `target` using Kubernetes-style `map` merge logic.
/// This means that all elements from `other` which are already present in `target` are merged using `OptionableConvert::merge`
/// and the other ones that are missing in `target` get appended.
/// The merge logic short circuits, i.e. only the first key match from `MapKeysEq` gets its content merged.
/// Handles the Option wrapping the respective type.
pub fn merge_map_option_wrapped<TARGET, OTHER, T>(target: &mut Option<TARGET>, other: Option<OTHER>)
where
    TARGET: Extend<T>,
    for<'a> &'a mut TARGET: IntoIterator<Item = &'a mut T>,
    OTHER: IntoIterator<Item = T> + Into<TARGET>,
    T: DeepMerge + MapKeysEq,
{
    if let Some(other) = other {
        if let Some(target) = target {
            merge_map(target, other);
        } else {
            *target = Some(other.into());
        }
    }
}
