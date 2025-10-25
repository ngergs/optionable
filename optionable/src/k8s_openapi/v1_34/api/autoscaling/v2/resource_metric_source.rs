pub struct ResourceMetricSourceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource {
    type Optioned = ResourceMetricSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceMetricSourceOpt {
    type Optioned = ResourceMetricSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ResourceMetricSource {
    fn into_optioned(self) -> ResourceMetricSourceOpt {
        ResourceMetricSourceOpt {
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
        value: ResourceMetricSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ResourceMetricSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
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
