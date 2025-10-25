pub struct ContainerResourceMetricSourceOpt {
    pub container: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::container_resource_metric_source::ContainerResourceMetricSource {
    type Optioned = ContainerResourceMetricSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerResourceMetricSourceOpt {
    type Optioned = ContainerResourceMetricSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::container_resource_metric_source::ContainerResourceMetricSource {
    fn into_optioned(self) -> ContainerResourceMetricSourceOpt {
        ContainerResourceMetricSourceOpt {
            container: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.container,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            target: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::into_optioned(
                    self.target,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ContainerResourceMetricSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .container
                    .ok_or(crate::optionable::Error {
                        missing_field: "container",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            target: <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::try_from_optioned(
                value
                    .target
                    .ok_or(crate::optionable::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerResourceMetricSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.container,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.target {
            <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::merge(
                &mut self.target,
                other_value,
            )?;
        }
        Ok(())
    }
}
