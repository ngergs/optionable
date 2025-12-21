#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceReferenceAc {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference {
    fn into_optioned(self) -> ServiceReferenceAc {
        ServiceReferenceAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: Some(crate::OptionableConvert::into_optioned(self.namespace)),
            path: crate::OptionableConvert::into_optioned(self.path),
            port: crate::OptionableConvert::into_optioned(self.port),
        }
    }
    fn try_from_optioned(value: ServiceReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: crate::OptionableConvert::try_from_optioned(
                value
                    .namespace
                    .ok_or(crate::Error {
                        missing_field: "namespace",
                    })?,
            )?,
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
        })
    }
    fn merge(&mut self, other: ServiceReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.namespace {
            crate::OptionableConvert::merge(&mut self.namespace, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference,
> for ServiceReferenceAc {
    fn from_optionable(
        value: k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
