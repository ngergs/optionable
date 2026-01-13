#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selector: <Option<
        ::k8s_openapi027::api::authorization::v1::FieldSelectorAttributes,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: <Option<
        ::k8s_openapi027::api::authorization::v1::LabelSelectorAttributes,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authorization::v1::ResourceAttributes {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceAttributesAc {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::ResourceAttributes {
    fn into_optioned(self) -> ResourceAttributesAc {
        ResourceAttributesAc {
            field_selector: crate::OptionableConvert::into_optioned(self.field_selector),
            group: crate::OptionableConvert::into_optioned(self.group),
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            resource: crate::OptionableConvert::into_optioned(self.resource),
            subresource: crate::OptionableConvert::into_optioned(self.subresource),
            verb: crate::OptionableConvert::into_optioned(self.verb),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(value: ResourceAttributesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_selector: crate::OptionableConvert::try_from_optioned(
                value.field_selector,
            )?,
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            subresource: crate::OptionableConvert::try_from_optioned(value.subresource)?,
            verb: crate::OptionableConvert::try_from_optioned(value.verb)?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(&mut self, other: ResourceAttributesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.field_selector, other.field_selector)?;
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        crate::OptionableConvert::merge(&mut self.subresource, other.subresource)?;
        crate::OptionableConvert::merge(&mut self.verb, other.verb)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::authorization::v1::ResourceAttributes>
for ResourceAttributesAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::ResourceAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::ResourceAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::ResourceAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
