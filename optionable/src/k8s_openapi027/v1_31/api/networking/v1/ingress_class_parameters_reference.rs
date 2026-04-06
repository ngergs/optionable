#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressClassParametersReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::networking::v1::IngressClassParametersReference {
    type Optioned = IngressClassParametersReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for IngressClassParametersReferenceAc {
    type Optioned = IngressClassParametersReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::IngressClassParametersReference {
    fn into_optioned(self) -> IngressClassParametersReferenceAc {
        IngressClassParametersReferenceAc {
            api_group: self.api_group,
            kind: Some(self.kind),
            name: Some(self.name),
            namespace: self.namespace,
            scope: self.scope,
        }
    }
    fn try_from_optioned(
        value: IngressClassParametersReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: value.api_group,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            namespace: value.namespace,
            scope: value.scope,
        })
    }
    fn merge(
        &mut self,
        other: IngressClassParametersReferenceAc,
    ) -> Result<(), crate::Error> {
        self.api_group = other.api_group;
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.namespace = other.namespace;
        self.scope = other.scope;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::networking::v1::IngressClassParametersReference,
> for IngressClassParametersReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressClassParametersReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::IngressClassParametersReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressClassParametersReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
