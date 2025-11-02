#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IPAddressSpec {
    fn into_optioned(self) -> IPAddressSpecAc {
        IPAddressSpecAc {
            parent_ref: Some(crate::OptionableConvert::into_optioned(self.parent_ref)),
        }
    }
    fn try_from_optioned(
        value: IPAddressSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            parent_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .parent_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "parent_ref",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: IPAddressSpecAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.parent_ref {
            crate::OptionableConvert::merge(&mut self.parent_ref, other_value)?;
        }
        Ok(())
    }
}
