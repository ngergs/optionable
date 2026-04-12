#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Mutation specifies the CEL expression which is used to apply the Mutation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutationAc {
    /// applyConfiguration defines the desired configuration values of an object. The configuration is applied to the admission object using \[structured merge diff\](https://github.com/kubernetes-sigs/structured-merge-diff). A CEL expression is used to create apply configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_configuration: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration as crate::Optionable>::Optioned,
    >,
    /// jsonPatch defines a \[JSON patch\](https://jsonpatch.com/) operation to perform a mutation to the object. A CEL expression is used to create the JSON patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_patch: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch as crate::Optionable>::Optioned,
    >,
    /// patchType indicates the patch strategy used. Allowed values are "ApplyConfiguration" and "JSONPatch". Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_type: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::Mutation {
    type Optioned = MutationAc;
}
#[automatically_derived]
impl crate::Optionable for MutationAc {
    type Optioned = MutationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::Mutation {
    fn into_optioned(self) -> MutationAc {
        MutationAc {
            apply_configuration: crate::OptionableConvert::into_optioned(
                self.apply_configuration,
            ),
            json_patch: crate::OptionableConvert::into_optioned(self.json_patch),
            patch_type: Some(self.patch_type),
        }
    }
    fn try_from_optioned(value: MutationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            apply_configuration: crate::OptionableConvert::try_from_optioned(
                value.apply_configuration,
            )?,
            json_patch: crate::OptionableConvert::try_from_optioned(value.json_patch)?,
            patch_type: value
                .patch_type
                .ok_or(crate::Error {
                    missing_field: "patch_type",
                })?,
        })
    }
    fn merge(&mut self, other: MutationAc) -> Result<(), crate::Error> {
        if self.apply_configuration.is_none() {
            self.apply_configuration = crate::OptionableConvert::try_from_optioned(
                other.apply_configuration,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.apply_configuration,
                other.apply_configuration,
            )?;
        }
        if self.json_patch.is_none() {
            self.json_patch = crate::OptionableConvert::try_from_optioned(
                other.json_patch,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.json_patch, other.json_patch)?;
        }
        if let Some(other_value) = other.patch_type {
            self.patch_type = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::Mutation,
> for MutationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::Mutation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::Mutation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::Mutation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
