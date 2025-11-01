pub struct DaemonSetOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::apps::v1::DaemonSetSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::apps::v1::DaemonSetStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::DaemonSet {
    type Optioned = DaemonSetOpt;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetOpt {
    type Optioned = DaemonSetOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::DaemonSet {
    fn into_optioned(self) -> DaemonSetOpt {
        DaemonSetOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: DaemonSetOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: DaemonSetOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
