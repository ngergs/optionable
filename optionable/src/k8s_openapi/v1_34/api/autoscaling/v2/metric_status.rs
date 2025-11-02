#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: <Option<
        ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: <Option<
        ::k8s_openapi::api::autoscaling::v2::ObjectMetricStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: <Option<
        ::k8s_openapi::api::autoscaling::v2::PodsMetricStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricStatus {
    type Optioned = MetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for MetricStatusAc {
    type Optioned = MetricStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricStatus {
    fn into_optioned(self) -> MetricStatusAc {
        MetricStatusAc {
            container_resource: crate::OptionableConvert::into_optioned(
                self.container_resource,
            ),
            external: crate::OptionableConvert::into_optioned(self.external),
            object: crate::OptionableConvert::into_optioned(self.object),
            pods: crate::OptionableConvert::into_optioned(self.pods),
            resource: crate::OptionableConvert::into_optioned(self.resource),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: MetricStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_resource: crate::OptionableConvert::try_from_optioned(
                value.container_resource,
            )?,
            external: crate::OptionableConvert::try_from_optioned(value.external)?,
            object: crate::OptionableConvert::try_from_optioned(value.object)?,
            pods: crate::OptionableConvert::try_from_optioned(value.pods)?,
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: MetricStatusAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.container_resource,
            other.container_resource,
        )?;
        crate::OptionableConvert::merge(&mut self.external, other.external)?;
        crate::OptionableConvert::merge(&mut self.object, other.object)?;
        crate::OptionableConvert::merge(&mut self.pods, other.pods)?;
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
