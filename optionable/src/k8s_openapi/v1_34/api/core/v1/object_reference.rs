pub struct ObjectReferenceOpt {
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
    type Optioned = ObjectReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for ObjectReferenceOpt {
    type Optioned = ObjectReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ObjectReference {
    fn into_optioned(self) -> ObjectReferenceOpt {
        ObjectReferenceOpt {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_version),
            field_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.field_path),
            kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.kind),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: ObjectReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_version)?,
            field_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.field_path)?,
            kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.kind)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: ObjectReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.api_version, other.api_version)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.field_path, other.field_path)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.kind, other.kind)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
