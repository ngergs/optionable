#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerResourceMetricStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<
        <::k8s_openapi026::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus {
    type Optioned = ContainerResourceMetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerResourceMetricStatusAc {
    type Optioned = ContainerResourceMetricStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus {
    fn into_optioned(self) -> ContainerResourceMetricStatusAc {
        ContainerResourceMetricStatusAc {
            container: Some(crate::OptionableConvert::into_optioned(self.container)),
            current: Some(crate::OptionableConvert::into_optioned(self.current)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: ContainerResourceMetricStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            container: crate::OptionableConvert::try_from_optioned(
                value
                    .container
                    .ok_or(crate::Error {
                        missing_field: "container",
                    })?,
            )?,
            current: crate::OptionableConvert::try_from_optioned(
                value
                    .current
                    .ok_or(crate::Error {
                        missing_field: "current",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerResourceMetricStatusAc,
    ) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus,
> for ContainerResourceMetricStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::autoscaling::v2::ContainerResourceMetricStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
