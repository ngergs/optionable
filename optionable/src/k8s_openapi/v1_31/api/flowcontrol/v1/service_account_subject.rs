#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceAccountSubjectAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject {
    type Optioned = ServiceAccountSubjectAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountSubjectAc {
    type Optioned = ServiceAccountSubjectAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject {
    fn into_optioned(self) -> ServiceAccountSubjectAc {
        ServiceAccountSubjectAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: Some(crate::OptionableConvert::into_optioned(self.namespace)),
        }
    }
    fn try_from_optioned(value: ServiceAccountSubjectAc) -> Result<Self, crate::Error> {
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
        })
    }
    fn merge(&mut self, other: ServiceAccountSubjectAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.namespace {
            crate::OptionableConvert::merge(&mut self.namespace, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject>
for ServiceAccountSubjectAc {
    fn from_optionable(
        value: ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
