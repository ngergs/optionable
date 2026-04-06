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
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject {
    type Optioned = ServiceAccountSubjectAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountSubjectAc {
    type Optioned = ServiceAccountSubjectAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject {
    fn into_optioned(self) -> ServiceAccountSubjectAc {
        ServiceAccountSubjectAc {
            name: Some(self.name),
            namespace: Some(self.namespace),
        }
    }
    fn try_from_optioned(value: ServiceAccountSubjectAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            namespace: value
                .namespace
                .ok_or(crate::Error {
                    missing_field: "namespace",
                })?,
        })
    }
    fn merge(&mut self, other: ServiceAccountSubjectAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.namespace {
            self.namespace = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject>
for ServiceAccountSubjectAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
