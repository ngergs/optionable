pub struct ForZoneOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::ForZone {
    type Optioned = ForZoneOpt;
}
#[automatically_derived]
impl crate::Optionable for ForZoneOpt {
    type Optioned = ForZoneOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::ForZone {
    fn into_optioned(self) -> ForZoneOpt {
        ForZoneOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(value: ForZoneOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ForZoneOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
