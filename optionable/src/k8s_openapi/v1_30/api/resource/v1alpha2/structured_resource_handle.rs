#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StructuredResourceHandleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::resource::v1alpha2::DriverAllocationResult,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_claim_parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_class_parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::StructuredResourceHandle {
    type Optioned = StructuredResourceHandleAc;
}
#[automatically_derived]
impl crate::Optionable for StructuredResourceHandleAc {
    type Optioned = StructuredResourceHandleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::StructuredResourceHandle {
    fn into_optioned(self) -> StructuredResourceHandleAc {
        StructuredResourceHandleAc {
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            results: Some(crate::OptionableConvert::into_optioned(self.results)),
            vendor_claim_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_claim_parameters,
            ),
            vendor_class_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_class_parameters,
            ),
        }
    }
    fn try_from_optioned(
        value: StructuredResourceHandleAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            results: crate::OptionableConvert::try_from_optioned(
                value
                    .results
                    .ok_or(crate::Error {
                        missing_field: "results",
                    })?,
            )?,
            vendor_claim_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_claim_parameters,
            )?,
            vendor_class_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_class_parameters,
            )?,
        })
    }
    fn merge(&mut self, other: StructuredResourceHandleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        if let Some(other_value) = other.results {
            crate::OptionableConvert::merge(&mut self.results, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.vendor_claim_parameters,
            other.vendor_claim_parameters,
        )?;
        crate::OptionableConvert::merge(
            &mut self.vendor_class_parameters,
            other.vendor_class_parameters,
        )?;
        Ok(())
    }
}
