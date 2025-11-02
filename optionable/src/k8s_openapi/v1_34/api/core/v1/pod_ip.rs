#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodIPAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodIP {
    type Optioned = PodIPAc;
}
#[automatically_derived]
impl crate::Optionable for PodIPAc {
    type Optioned = PodIPAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodIP {
    fn into_optioned(self) -> PodIPAc {
        PodIPAc {
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
        }
    }
    fn try_from_optioned(value: PodIPAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ip: crate::OptionableConvert::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::optionable::Error {
                        missing_field: "ip",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodIPAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        Ok(())
    }
}
