#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// APIServiceStatus contains derived information about an API server
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIServiceStatusAc {
    /// Current service state of apiService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition as crate::Optionable>::Optioned,
        >,
    >,
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
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
