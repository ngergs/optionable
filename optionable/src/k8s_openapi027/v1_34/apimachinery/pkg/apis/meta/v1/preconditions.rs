#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreconditionsAc {
    /// Specifies the target ResourceVersion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    /// Specifies the target UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions {
    type Optioned = PreconditionsAc;
}
#[automatically_derived]
impl crate::Optionable for PreconditionsAc {
    type Optioned = PreconditionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions {
    fn into_optioned(self) -> PreconditionsAc {
        PreconditionsAc {
            resource_version: self.resource_version,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: PreconditionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            resource_version: value.resource_version,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: PreconditionsAc) -> Result<(), crate::Error> {
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.resource_version,
                other.resource_version,
            )?;
        }
        if self.uid.is_none() {
            self.uid = crate::OptionableConvert::try_from_optioned(other.uid)?;
        } else {
            crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions,
> for PreconditionsAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
