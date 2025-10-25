pub struct ContainerResourceMetricStatusOpt {
    pub container: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::container_resource_metric_status::ContainerResourceMetricStatus {
    type Optioned = ContainerResourceMetricStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerResourceMetricStatusOpt {
    type Optioned = ContainerResourceMetricStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::container_resource_metric_status::ContainerResourceMetricStatus {
    fn into_optioned(self) -> ContainerResourceMetricStatusOpt {
        ContainerResourceMetricStatusOpt {
            container: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.container,
                ),
            ),
            current: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::into_optioned(
                    self.current,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ContainerResourceMetricStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .container
                    .ok_or(crate::optionable::Error {
                        missing_field: "container",
                    })?,
            )?,
            current: <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::try_from_optioned(
                value
                    .current
                    .ok_or(crate::optionable::Error {
                        missing_field: "current",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerResourceMetricStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.container,
                other_value,
            )?;
        }
        if let Some(other_value) = other.current {
            <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::merge(
                &mut self.current,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
