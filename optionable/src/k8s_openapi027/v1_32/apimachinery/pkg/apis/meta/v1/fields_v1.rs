#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.
///
/// Each key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:\<name\>', where \<name\> is the name of a field in a struct, or key in a map 'v:\<value\>', where \<value\> is the exact json formatted value of a list item 'i:\<index\>', where \<index\> is position of a item in a list 'k:\<keys\>', where \<keys\> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.
///
/// The exact format is defined in sigs.k8s.io/structured-merge-diff
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldsV1Ac(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi027::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1 {
    type Optioned = FieldsV1Ac;
}
#[automatically_derived]
impl crate::Optionable for FieldsV1Ac {
    type Optioned = FieldsV1Ac;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1 {
    fn into_optioned(self) -> FieldsV1Ac {
        FieldsV1Ac(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: FieldsV1Ac) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: FieldsV1Ac) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            self.0 = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1>
for FieldsV1Ac {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
