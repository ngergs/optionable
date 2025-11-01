#[derive(kube::Resource)]
#[resource(inherit = APIService)]
pub struct APIServiceAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIService {
    type Optioned = APIServiceAc;
}
#[automatically_derived]
impl crate::Optionable for APIServiceAc {
    type Optioned = APIServiceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIService {
    fn into_optioned(self) -> APIServiceAc {
        APIServiceAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: APIServiceAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: APIServiceAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIService;
