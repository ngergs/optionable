pub struct ObjectReferenceAc {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub field_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ObjectReference {
    type Optioned = ObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectReferenceAc {
    type Optioned = ObjectReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ObjectReference {
    fn into_optioned(self) -> ObjectReferenceAc {
        ObjectReferenceAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            field_path: crate::OptionableConvert::into_optioned(self.field_path),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: ObjectReferenceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            field_path: crate::OptionableConvert::try_from_optioned(value.field_path)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: ObjectReferenceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.field_path, other.field_path)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
