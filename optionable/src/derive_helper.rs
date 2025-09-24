//! Helpers, primarily for making the derive implementation easier and more efficient.

use crate::optionable::Error;
use crate::{OptionableConvert, OptionedConvert};

/// Simplifies calling `into_optioned` for derived statements.
/// I.e. `Some(<String as ::optionable::OptionableConvert>::into_optioned(self_0))`
/// becomes `optionable::helper::into_optioned(self_0)`.
pub fn into_optioned_some<T: OptionableConvert>(val: T) -> Option<T::Optioned> {
    Some(val.into_optioned())
}

/// Simplifies calling `into_optioned` for derived statements.
/// I.e. `<String as ::optionable::OptionableConvert>::into_optioned(self_0)`
/// becomes `optionable::helper::into_optioned(self_0)`.
pub fn into_optioned<T: OptionableConvert>(val: T) -> T::Optioned {
    val.into_optioned()
}

/// Simplified calling `try_from_optioned` for derived statements.
/// I.e. `<String as ::optionable::OptionableConvert>::try_from_optioned(value.name.ok_or(optionable::optionable::Error{ missing_fields: std::vec!["name"]})?)?`
/// becomes `optionable::helper::try_from_optioned(value.name, "name")?`.
pub fn try_from_optioned<T: OptionableConvert>(
    val: Option<T::Optioned>,
    field_id: &'static str,
) -> Result<T, Error> {
    val.ok_or(Error {
        missing_fields: std::vec![field_id],
    })?
    .try_into_optionable()
}

/// Simplifies calling `merge` for derived statements.
/// I.e. `<String as ::optionable::OptionableConvert>::merge(&mut self.0, other.0)?`
/// becomes `optionable::helper::merge(self_0, other.0)?`.
pub fn merge<T: OptionableConvert>(val: &mut T, other: T::Optioned) -> Result<(), Error> {
    val.merge(other)
}

/// Simplifies calling `merge` for derived statements.
/// I.e. `if let Some(other_value) = other.0 {<String as ::optionable::OptionableConvert>::merge(&mut self.0, other_value)?}`
/// becomes `optionable::helper::merge_option(self_0, other_value)?`.
pub fn merge_option<T: OptionableConvert>(
    val: &mut T,
    other: Option<T::Optioned>,
) -> Result<(), Error> {
    if let Some(other) = other {
        val.merge(other)?;
    }
    Ok(())
}
