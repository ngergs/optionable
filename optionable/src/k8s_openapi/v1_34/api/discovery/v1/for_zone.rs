#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForZoneAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::ForZone {
    type Optioned = ForZoneAc;
}
#[automatically_derived]
impl crate::Optionable for ForZoneAc {
    type Optioned = ForZoneAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::ForZone {
    fn into_optioned(self) -> ForZoneAc {
        ForZoneAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: ForZoneAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ForZoneAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
