pub struct DeviceConstraintOpt {
    pub distinct_attribute: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub match_attribute: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::device_constraint::DeviceConstraint {
    type Optioned = DeviceConstraintOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceConstraintOpt {
    type Optioned = DeviceConstraintOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::device_constraint::DeviceConstraint {
    fn into_optioned(self) -> DeviceConstraintOpt {
        DeviceConstraintOpt {
            distinct_attribute: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.distinct_attribute),
            match_attribute: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.match_attribute),
            requests: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceConstraintOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            distinct_attribute: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.distinct_attribute)?,
            match_attribute: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.match_attribute)?,
            requests: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceConstraintOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.distinct_attribute,
            other.distinct_attribute,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.match_attribute,
            other.match_attribute,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
