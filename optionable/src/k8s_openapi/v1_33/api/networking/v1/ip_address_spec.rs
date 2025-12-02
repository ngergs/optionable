#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<
        <::k8s_openapi::api::networking::v1::ParentReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IPAddressSpec {
    type Optioned = IPAddressSpecAc;
}
#[automatically_derived]
impl crate::Optionable for IPAddressSpecAc {
    type Optioned = IPAddressSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IPAddressSpec {
    fn into_optioned(self) -> IPAddressSpecAc {
        IPAddressSpecAc {
            parent_ref: Some(crate::OptionableConvert::into_optioned(self.parent_ref)),
        }
    }
    fn try_from_optioned(value: IPAddressSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            parent_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .parent_ref
                    .ok_or(crate::Error {
                        missing_field: "parent_ref",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: IPAddressSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.parent_ref {
            crate::OptionableConvert::merge(&mut self.parent_ref, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::networking::v1::IPAddressSpec>
for IPAddressSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::networking::v1::IPAddressSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::networking::v1::IPAddressSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::networking::v1::IPAddressSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
