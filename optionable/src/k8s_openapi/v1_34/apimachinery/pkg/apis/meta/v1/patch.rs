pub enum PatchOpt {
    Json(
        Option<
            <std::vec::Vec<
                ::k8s_openapi::serde_json::Value,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Merge(Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>),
    StrategicMerge(
        Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
    ),
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::patch::Patch {
    type Optioned = PatchOpt;
}
#[automatically_derived]
impl crate::Optionable for PatchOpt {
    type Optioned = PatchOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::patch::Patch {
    fn into_optioned(self) -> PatchOpt {
        match self {
            Self::Json(self_0) => {
                PatchOpt::Json(
                    Some(
                        <std::vec::Vec<
                            ::k8s_openapi::serde_json::Value,
                        > as crate::OptionableConvert>::into_optioned(self_0),
                    ),
                )
            }
            Self::Merge(self_0) => {
                PatchOpt::Merge(
                    Some(
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
            Self::StrategicMerge(self_0) => {
                PatchOpt::StrategicMerge(
                    Some(
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
        }
    }
    fn try_from_optioned(other: PatchOpt) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                PatchOpt::Json(other_0) => {
                    Self::Json(
                        <std::vec::Vec<
                            ::k8s_openapi::serde_json::Value,
                        > as crate::OptionableConvert>::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                PatchOpt::Merge(other_0) => {
                    Self::Merge(
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                PatchOpt::StrategicMerge(other_0) => {
                    Self::StrategicMerge(
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: PatchOpt) -> Result<(), crate::optionable::Error> {
        match other {
            PatchOpt::Json(other_0) => {
                if let Self::Json(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <std::vec::Vec<
                            ::k8s_openapi::serde_json::Value,
                        > as crate::OptionableConvert>::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchOpt::Json(other_0))?;
                }
            }
            PatchOpt::Merge(other_0) => {
                if let Self::Merge(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchOpt::Merge(other_0))?;
                }
            }
            PatchOpt::StrategicMerge(other_0) => {
                if let Self::StrategicMerge(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(PatchOpt::StrategicMerge(other_0))?;
                }
            }
        }
        Ok(())
    }
}
