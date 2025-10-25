pub struct MetricSpecOpt {
    pub container_resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource,
    > as crate::Optionable>::Optioned,
    pub external: <Option<
        ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource,
    > as crate::Optionable>::Optioned,
    pub object: <Option<
        ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
    > as crate::Optionable>::Optioned,
    pub pods: <Option<
        ::k8s_openapi::api::autoscaling::v2::PodsMetricSource,
    > as crate::Optionable>::Optioned,
    pub resource: <Option<
        ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource,
    > as crate::Optionable>::Optioned,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricSpec {
    type Optioned = MetricSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for MetricSpecOpt {
    type Optioned = MetricSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricSpec {
    fn into_optioned(self) -> MetricSpecOpt {
        MetricSpecOpt {
            container_resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource,
            > as crate::OptionableConvert>::into_optioned(self.container_resource),
            external: <Option<
                ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource,
            > as crate::OptionableConvert>::into_optioned(self.external),
            object: <Option<
                ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
            > as crate::OptionableConvert>::into_optioned(self.object),
            pods: <Option<
                ::k8s_openapi::api::autoscaling::v2::PodsMetricSource,
            > as crate::OptionableConvert>::into_optioned(self.pods),
            resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource,
            > as crate::OptionableConvert>::into_optioned(self.resource),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: MetricSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource,
            > as crate::OptionableConvert>::try_from_optioned(value.container_resource)?,
            external: <Option<
                ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource,
            > as crate::OptionableConvert>::try_from_optioned(value.external)?,
            object: <Option<
                ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
            > as crate::OptionableConvert>::try_from_optioned(value.object)?,
            pods: <Option<
                ::k8s_openapi::api::autoscaling::v2::PodsMetricSource,
            > as crate::OptionableConvert>::try_from_optioned(value.pods)?,
            resource: <Option<
                ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource,
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
    fn merge(&mut self, other: MetricSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource,
        > as crate::OptionableConvert>::merge(
            &mut self.container_resource,
            other.container_resource,
        )?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource,
        > as crate::OptionableConvert>::merge(&mut self.external, other.external)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
        > as crate::OptionableConvert>::merge(&mut self.object, other.object)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::PodsMetricSource,
        > as crate::OptionableConvert>::merge(&mut self.pods, other.pods)?;
        <Option<
            ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource,
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
