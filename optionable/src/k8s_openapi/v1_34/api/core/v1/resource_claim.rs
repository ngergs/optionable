#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceClaim {
    type Optioned = ResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimAc {
    type Optioned = ResourceClaimAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceClaim {
    fn into_optioned(self) -> ResourceClaimAc {
        ResourceClaimAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            request: crate::OptionableConvert::into_optioned(self.request),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            request: crate::OptionableConvert::try_from_optioned(value.request)?,
        })
    }
    fn merge(&mut self, other: ResourceClaimAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.request, other.request)?;
        Ok(())
    }
}
