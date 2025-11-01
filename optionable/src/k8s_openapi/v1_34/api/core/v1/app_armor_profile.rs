pub struct AppArmorProfileAc {
    pub localhost_profile: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::AppArmorProfile {
    type Optioned = AppArmorProfileAc;
}
#[automatically_derived]
impl crate::Optionable for AppArmorProfileAc {
    type Optioned = AppArmorProfileAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::AppArmorProfile {
    fn into_optioned(self) -> AppArmorProfileAc {
        AppArmorProfileAc {
            localhost_profile: crate::OptionableConvert::into_optioned(
                self.localhost_profile,
            ),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: AppArmorProfileAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            localhost_profile: crate::OptionableConvert::try_from_optioned(
                value.localhost_profile,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AppArmorProfileAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.localhost_profile,
            other.localhost_profile,
        )?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
