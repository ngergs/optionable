#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricTargetAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricTarget {
    type Optioned = MetricTargetAc;
}
#[automatically_derived]
impl crate::Optionable for MetricTargetAc {
    type Optioned = MetricTargetAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricTarget {
    fn into_optioned(self) -> MetricTargetAc {
        MetricTargetAc {
            average_utilization: crate::OptionableConvert::into_optioned(
                self.average_utilization,
            ),
            average_value: crate::OptionableConvert::into_optioned(self.average_value),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: MetricTargetAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            average_utilization: crate::OptionableConvert::try_from_optioned(
                value.average_utilization,
            )?,
            average_value: crate::OptionableConvert::try_from_optioned(
                value.average_value,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: MetricTargetAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.average_utilization,
            other.average_utilization,
        )?;
        crate::OptionableConvert::merge(&mut self.average_value, other.average_value)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
