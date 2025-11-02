#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricIdentifierAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricIdentifier {
    type Optioned = MetricIdentifierAc;
}
#[automatically_derived]
impl crate::Optionable for MetricIdentifierAc {
    type Optioned = MetricIdentifierAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricIdentifier {
    fn into_optioned(self) -> MetricIdentifierAc {
        MetricIdentifierAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(
        value: MetricIdentifierAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(
        &mut self,
        other: MetricIdentifierAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}
