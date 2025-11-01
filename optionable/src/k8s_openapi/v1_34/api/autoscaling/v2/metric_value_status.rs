pub struct MetricValueStatusAc {
    pub average_utilization: <Option<i32> as crate::Optionable>::Optioned,
    pub average_value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricValueStatus {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
impl crate::Optionable for MetricValueStatusAc {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::MetricValueStatus {
    fn into_optioned(self) -> MetricValueStatusAc {
        MetricValueStatusAc {
            average_utilization: crate::OptionableConvert::into_optioned(
                self.average_utilization,
            ),
            average_value: crate::OptionableConvert::into_optioned(self.average_value),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: MetricValueStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            average_utilization: crate::OptionableConvert::try_from_optioned(
                value.average_utilization,
            )?,
            average_value: crate::OptionableConvert::try_from_optioned(
                value.average_value,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(
        &mut self,
        other: MetricValueStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.average_utilization,
            other.average_utilization,
        )?;
        crate::OptionableConvert::merge(&mut self.average_value, other.average_value)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
