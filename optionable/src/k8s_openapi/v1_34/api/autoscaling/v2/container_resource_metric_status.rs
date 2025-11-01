pub struct ContainerResourceMetricStatusAc {
    pub container: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus {
    type Optioned = ContainerResourceMetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerResourceMetricStatusAc {
    type Optioned = ContainerResourceMetricStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricStatus {
    fn into_optioned(self) -> ContainerResourceMetricStatusAc {
        ContainerResourceMetricStatusAc {
            container: Some(crate::OptionableConvert::into_optioned(self.container)),
            current: Some(crate::OptionableConvert::into_optioned(self.current)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: ContainerResourceMetricStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container: crate::OptionableConvert::try_from_optioned(
                value
                    .container
                    .ok_or(crate::optionable::Error {
                        missing_field: "container",
                    })?,
            )?,
            current: crate::OptionableConvert::try_from_optioned(
                value
                    .current
                    .ok_or(crate::optionable::Error {
                        missing_field: "current",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
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
        other: ContainerResourceMetricStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container {
            crate::OptionableConvert::merge(&mut self.container, other_value)?;
        }
        if let Some(other_value) = other.current {
            crate::OptionableConvert::merge(&mut self.current, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
