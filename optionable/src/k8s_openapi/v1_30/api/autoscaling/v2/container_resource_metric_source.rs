#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource {
    type Optioned = ContainerResourceMetricSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerResourceMetricSourceAc {
    type Optioned = ContainerResourceMetricSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ContainerResourceMetricSource {
    fn into_optioned(self) -> ContainerResourceMetricSourceAc {
        ContainerResourceMetricSourceAc {
            container: Some(crate::OptionableConvert::into_optioned(self.container)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(
        value: ContainerResourceMetricSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container: crate::OptionableConvert::try_from_optioned(
                value
                    .container
                    .ok_or(crate::optionable::Error {
                        missing_field: "container",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            target: crate::OptionableConvert::try_from_optioned(
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
        other: ContainerResourceMetricSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container {
            crate::OptionableConvert::merge(&mut self.container, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
