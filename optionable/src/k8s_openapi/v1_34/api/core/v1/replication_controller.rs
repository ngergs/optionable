pub struct ReplicationControllerOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::ReplicationControllerSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::core::v1::ReplicationControllerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ReplicationController {
    type Optioned = ReplicationControllerOpt;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerOpt {
    type Optioned = ReplicationControllerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ReplicationController {
    fn into_optioned(self) -> ReplicationControllerOpt {
        ReplicationControllerOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicationControllerOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
