#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ScaleSpec describes the attributes of a scale subresource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScaleSpecAc {
    /// replicas is the desired number of instances for the scaled object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v1::ScaleSpec {
    type Optioned = ScaleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleSpecAc {
    type Optioned = ScaleSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v1::ScaleSpec {
    fn into_optioned(self) -> ScaleSpecAc {
        ScaleSpecAc {
            replicas: self.replicas,
        }
    }
    fn try_from_optioned(value: ScaleSpecAc) -> Result<Self, crate::Error> {
        Ok(Self { replicas: value.replicas })
    }
    fn merge(&mut self, other: ScaleSpecAc) -> Result<(), crate::Error> {
        if self.replicas.is_none() {
            self.replicas = crate::OptionableConvert::try_from_optioned(other.replicas)?;
        } else {
            crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v1::ScaleSpec>
for ScaleSpecAc {
    fn from_optionable(value: k8s_openapi027::api::autoscaling::v1::ScaleSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v1::ScaleSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v1::ScaleSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
