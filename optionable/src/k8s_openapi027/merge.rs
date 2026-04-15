use k8s_openapi027::DeepMerge;

use crate::merge::MapKeysEq;

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
