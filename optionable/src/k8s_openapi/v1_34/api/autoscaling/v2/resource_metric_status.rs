pub struct ResourceMetricStatusOpt {
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::resource_metric_status::ResourceMetricStatus {
    type Optioned = ResourceMetricStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceMetricStatusOpt {
    type Optioned = ResourceMetricStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::resource_metric_status::ResourceMetricStatus {
    fn into_optioned(self) -> ResourceMetricStatusOpt {
        ResourceMetricStatusOpt {
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
        value: ResourceMetricStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ResourceMetricStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
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
