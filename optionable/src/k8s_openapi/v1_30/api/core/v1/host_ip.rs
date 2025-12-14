#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostIPAc {
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HostIP {
    fn into_optioned(self) -> HostIPAc {
        HostIPAc {
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
        }
    }
    fn try_from_optioned(value: HostIPAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ip: crate::OptionableConvert::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::Error {
                        missing_field: "ip",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HostIPAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::HostIP> for HostIPAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::HostIP) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::HostIP, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::HostIP,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
