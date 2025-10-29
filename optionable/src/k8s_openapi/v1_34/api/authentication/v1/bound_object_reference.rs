pub struct BoundObjectReferenceOpt {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::BoundObjectReference {
    type Optioned = BoundObjectReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for BoundObjectReferenceOpt {
    type Optioned = BoundObjectReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::BoundObjectReference {
    fn into_optioned(self) -> BoundObjectReferenceOpt {
        BoundObjectReferenceOpt {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            name: crate::OptionableConvert::into_optioned(self.name),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: BoundObjectReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: BoundObjectReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
