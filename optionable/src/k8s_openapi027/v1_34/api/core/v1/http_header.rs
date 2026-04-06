#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HTTPHeader describes a custom header to be used in HTTP probes
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPHeaderAc {
    /// The header field name. This will be canonicalized upon output, so case-variant names will be understood as the same header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// The header field value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::HTTPHeader {
    type Optioned = HTTPHeaderAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPHeaderAc {
    type Optioned = HTTPHeaderAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::HTTPHeader {
    fn into_optioned(self) -> HTTPHeaderAc {
        HTTPHeaderAc {
            name: Some(self.name),
            value: Some(self.value),
        }
    }
    fn try_from_optioned(value: HTTPHeaderAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            value: value
                .value
                .ok_or(crate::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(&mut self, other: HTTPHeaderAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::HTTPHeader> for HTTPHeaderAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::HTTPHeader) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::HTTPHeader, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::HTTPHeader,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
