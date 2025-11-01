pub struct HostIPAc {
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HostIP {
    type Optioned = HostIPAc;
}
#[automatically_derived]
impl crate::Optionable for HostIPAc {
    type Optioned = HostIPAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HostIP {
    fn into_optioned(self) -> HostIPAc {
        HostIPAc {
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
        }
    }
    fn try_from_optioned(value: HostIPAc) -> Result<Self, crate::optionable::Error> {
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
    fn merge(&mut self, other: HostIPAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        Ok(())
    }
}
