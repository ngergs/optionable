pub struct APIServiceStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    type Optioned = APIServiceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for APIServiceStatusOpt {
    type Optioned = APIServiceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    fn into_optioned(self) -> APIServiceStatusOpt {
        APIServiceStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: APIServiceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: APIServiceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
