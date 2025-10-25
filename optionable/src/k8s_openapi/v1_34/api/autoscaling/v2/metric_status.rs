pub struct MetricStatusOpt {
    pub container_resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus,
    > as crate::Optionable>::Optioned,
    pub external: <Option<
        ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus,
    > as crate::Optionable>::Optioned,
    pub object: <Option<
        ::k8s_openapi::api::autoscaling::v2::ObjectMetricStatus,
    > as crate::Optionable>::Optioned,
    pub pods: <Option<
        ::k8s_openapi::api::autoscaling::v2::PodsMetricStatus,
    > as crate::Optionable>::Optioned,
    pub resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
    > as crate::Optionable>::Optioned,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricStatus {
    type Optioned = MetricStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for MetricStatusOpt {
    type Optioned = MetricStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricStatus {
    fn into_optioned(self) -> MetricStatusOpt {
        MetricStatusOpt {
            container_resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus,
            > as crate::OptionableConvert>::into_optioned(self.container_resource),
            external: <Option<
                ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus,
            > as crate::OptionableConvert>::into_optioned(self.external),
            object: <Option<
                ::k8s_openapi::api::autoscaling::v2::ObjectMetricStatus,
            > as crate::OptionableConvert>::into_optioned(self.object),
            pods: <Option<
                ::k8s_openapi::api::autoscaling::v2::PodsMetricStatus,
            > as crate::OptionableConvert>::into_optioned(self.pods),
            resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
            > as crate::OptionableConvert>::into_optioned(self.resource),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: MetricStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.container_resource)?,
            external: <Option<
                ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.external)?,
            object: <Option<
                ::k8s_openapi::api::autoscaling::v2::ObjectMetricStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.object)?,
            pods: <Option<
                ::k8s_openapi::api::autoscaling::v2::PodsMetricStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.pods)?,
            resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.resource)?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: MetricStatusOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus,
        > as crate::OptionableConvert>::merge(
            &mut self.container_resource,
            other.container_resource,
        )?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus,
        > as crate::OptionableConvert>::merge(&mut self.external, other.external)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ObjectMetricStatus,
        > as crate::OptionableConvert>::merge(&mut self.object, other.object)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::PodsMetricStatus,
        > as crate::OptionableConvert>::merge(&mut self.pods, other.pods)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
        > as crate::OptionableConvert>::merge(&mut self.resource, other.resource)?;
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
