pub struct PodIPOpt {
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodIP {
    type Optioned = PodIPOpt;
}
#[automatically_derived]
impl crate::Optionable for PodIPOpt {
    type Optioned = PodIPOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodIP {
    fn into_optioned(self) -> PodIPOpt {
        PodIPOpt {
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
        }
    }
    fn try_from_optioned(value: PodIPOpt) -> Result<Self, crate::optionable::Error> {
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
    fn merge(&mut self, other: PodIPOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        Ok(())
    }
}
