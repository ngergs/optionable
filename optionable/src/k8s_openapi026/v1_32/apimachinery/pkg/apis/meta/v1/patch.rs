#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, std::fmt::Debug)]
#[serde(rename_all_fields = "camelCase", deny_unknown_fields)]
#[serde(untagged)]
pub enum PatchAc {
    Json(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <std::vec::Vec<
                ::k8s_openapi026::serde_json::Value,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Merge(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<::k8s_openapi026::serde_json::Value as crate::Optionable>::Optioned>,
    ),
    StrategicMerge(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<::k8s_openapi026::serde_json::Value as crate::Optionable>::Optioned>,
    ),
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch {
    type Optioned = PatchAc;
}
#[automatically_derived]
impl crate::Optionable for PatchAc {
    type Optioned = PatchAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch {
    fn into_optioned(self) -> PatchAc {
        match self {
            Self::Json(self_0) => {
                PatchAc::Json(Some(crate::OptionableConvert::into_optioned(self_0)))
            }
            Self::Merge(self_0) => {
                PatchAc::Merge(Some(crate::OptionableConvert::into_optioned(self_0)))
            }
            Self::StrategicMerge(self_0) => {
                PatchAc::StrategicMerge(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(other: PatchAc) -> Result<Self, crate::Error> {
        Ok(
            match other {
                PatchAc::Json(other_0) => {
                    Self::Json(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                PatchAc::Merge(other_0) => {
                    Self::Merge(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                PatchAc::StrategicMerge(other_0) => {
                    Self::StrategicMerge(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: PatchAc) -> Result<(), crate::Error> {
        match other {
            PatchAc::Json(other_0) => {
                if let Self::Json(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchAc::Json(other_0))?;
                }
            }
            PatchAc::Merge(other_0) => {
                if let Self::Merge(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchAc::Merge(other_0))?;
                }
            }
            PatchAc::StrategicMerge(other_0) => {
                if let Self::StrategicMerge(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchAc::StrategicMerge(other_0))?;
                }
            }
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch>
for PatchAc {
    fn from_optionable(
        value: k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::apimachinery::pkg::apis::meta::v1::Patch,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
