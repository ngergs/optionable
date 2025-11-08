#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ForNodeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::ForNode {
    type Optioned = ForNodeAc;
}
#[automatically_derived]
impl crate::Optionable for ForNodeAc {
    type Optioned = ForNodeAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::ForNode {
    fn into_optioned(self) -> ForNodeAc {
        ForNodeAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: ForNodeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ForNodeAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
