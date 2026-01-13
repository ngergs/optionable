#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIServiceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    type Optioned = APIServiceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for APIServiceStatusAc {
    type Optioned = APIServiceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    fn into_optioned(self) -> APIServiceStatusAc {
        APIServiceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(value: APIServiceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(&mut self, other: APIServiceStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus,
> for APIServiceStatusAc {
    fn from_optionable(
        value: k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
