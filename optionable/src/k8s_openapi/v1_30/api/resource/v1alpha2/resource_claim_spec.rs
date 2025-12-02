#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_ref: <Option<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_class_name: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSpecAc {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec {
    fn into_optioned(self) -> ResourceClaimSpecAc {
        ResourceClaimSpecAc {
            allocation_mode: crate::OptionableConvert::into_optioned(
                self.allocation_mode,
            ),
            parameters_ref: crate::OptionableConvert::into_optioned(self.parameters_ref),
            resource_class_name: Some(
                crate::OptionableConvert::into_optioned(self.resource_class_name),
            ),
        }
    }
    fn try_from_optioned(value: ResourceClaimSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation_mode: crate::OptionableConvert::try_from_optioned(
                value.allocation_mode,
            )?,
            parameters_ref: crate::OptionableConvert::try_from_optioned(
                value.parameters_ref,
            )?,
            resource_class_name: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_class_name
                    .ok_or(crate::Error {
                        missing_field: "resource_class_name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClaimSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.allocation_mode,
            other.allocation_mode,
        )?;
        crate::OptionableConvert::merge(&mut self.parameters_ref, other.parameters_ref)?;
        if let Some(other_value) = other.resource_class_name {
            crate::OptionableConvert::merge(&mut self.resource_class_name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec>
for ResourceClaimSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceClaimSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
