#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPIngressPathAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<
        <::k8s_openapi027::api::networking::v1::IngressBackend as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_type: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::HTTPIngressPath {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPIngressPathAc {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::HTTPIngressPath {
    fn into_optioned(self) -> HTTPIngressPathAc {
        HTTPIngressPathAc {
            backend: Some(crate::OptionableConvert::into_optioned(self.backend)),
            path: crate::OptionableConvert::into_optioned(self.path),
            path_type: Some(crate::OptionableConvert::into_optioned(self.path_type)),
        }
    }
    fn try_from_optioned(value: HTTPIngressPathAc) -> Result<Self, crate::Error> {
        Ok(Self {
            backend: crate::OptionableConvert::try_from_optioned(
                value
                    .backend
                    .ok_or(crate::Error {
                        missing_field: "backend",
                    })?,
            )?,
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            path_type: crate::OptionableConvert::try_from_optioned(
                value
                    .path_type
                    .ok_or(crate::Error {
                        missing_field: "path_type",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HTTPIngressPathAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.backend {
            crate::OptionableConvert::merge(&mut self.backend, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        if let Some(other_value) = other.path_type {
            crate::OptionableConvert::merge(&mut self.path_type, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::HTTPIngressPath>
for HTTPIngressPathAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::HTTPIngressPath,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::HTTPIngressPath, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::HTTPIngressPath,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
