pub struct MetricIdentifierOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricIdentifier {
    type Optioned = MetricIdentifierOpt;
}
#[automatically_derived]
impl crate::Optionable for MetricIdentifierOpt {
    type Optioned = MetricIdentifierOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::MetricIdentifier {
    fn into_optioned(self) -> MetricIdentifierOpt {
        MetricIdentifierOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(
        value: MetricIdentifierOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.selector)?,
        })
    }
    fn merge(
        &mut self,
        other: MetricIdentifierOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}
