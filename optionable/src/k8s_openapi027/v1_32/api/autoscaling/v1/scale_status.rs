#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ScaleStatus represents the current status of a scale subresource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScaleStatusAc {
    /// replicas is the actual number of observed instances of the scaled object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is the label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v1::ScaleStatus {
    type Optioned = ScaleStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleStatusAc {
    type Optioned = ScaleStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v1::ScaleStatus {
    fn into_optioned(self) -> ScaleStatusAc {
        ScaleStatusAc {
            replicas: Some(self.replicas),
            selector: self.selector,
        }
    }
    fn try_from_optioned(value: ScaleStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            replicas: value
                .replicas
                .ok_or(crate::Error {
                    missing_field: "replicas",
                })?,
            selector: value.selector,
        })
    }
    fn merge(&mut self, other: ScaleStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        if other.selector.is_some() {
            self.selector = other.selector;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v1::ScaleStatus>
for ScaleStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v1::ScaleStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v1::ScaleStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v1::ScaleStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
