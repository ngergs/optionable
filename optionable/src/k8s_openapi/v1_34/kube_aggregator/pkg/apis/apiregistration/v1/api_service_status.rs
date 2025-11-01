pub struct APIServiceStatusAc {
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    type Optioned = APIServiceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for APIServiceStatusAc {
    type Optioned = APIServiceStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus {
    fn into_optioned(self) -> APIServiceStatusAc {
        APIServiceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: APIServiceStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: APIServiceStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
