pub struct ResourceAttributesOpt {
    pub field_selector: <Option<
        ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes,
    > as crate::Optionable>::Optioned,
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub label_selector: <Option<
        ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes,
    > as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub subresource: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::ResourceAttributes {
    type Optioned = ResourceAttributesOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceAttributesOpt {
    type Optioned = ResourceAttributesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::ResourceAttributes {
    fn into_optioned(self) -> ResourceAttributesOpt {
        ResourceAttributesOpt {
            field_selector: <Option<
                ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes,
            > as crate::OptionableConvert>::into_optioned(self.field_selector),
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            label_selector: <Option<
                ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes,
            > as crate::OptionableConvert>::into_optioned(self.label_selector),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            resource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource),
            subresource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.subresource),
            verb: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.verb),
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: ResourceAttributesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field_selector: <Option<
                ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes,
            > as crate::OptionableConvert>::try_from_optioned(value.field_selector)?,
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            label_selector: <Option<
                ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes,
            > as crate::OptionableConvert>::try_from_optioned(value.label_selector)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            resource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource)?,
            subresource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.subresource)?,
            verb: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.verb)?,
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceAttributesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes,
        > as crate::OptionableConvert>::merge(
            &mut self.field_selector,
            other.field_selector,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        <Option<
            ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes,
        > as crate::OptionableConvert>::merge(
            &mut self.label_selector,
            other.label_selector,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.resource, other.resource)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.subresource, other.subresource)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.verb, other.verb)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
