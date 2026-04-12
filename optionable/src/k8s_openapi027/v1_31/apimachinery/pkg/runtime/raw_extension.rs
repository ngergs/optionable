#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RawExtension is used to hold extensions in external versions.
///
/// To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.
///
/// // Internal package:
///
///   type MyAPIObject struct {
///         runtime.TypeMeta `json:",inline"`
///         MyPlugin runtime.Object `json:"myPlugin"`
///     }
///
///   type PluginA struct {
///         AOption string `json:"aOption"`
///     }
///
/// // External package:
///
///   type MyAPIObject struct {
///         runtime.TypeMeta `json:",inline"`
///         MyPlugin runtime.RawExtension `json:"myPlugin"`
///     }
///
///   type PluginA struct {
///         AOption string `json:"aOption"`
///     }
///
/// // On the wire, the JSON will look something like this:
///
///   {
///         "kind":"MyAPIObject",
///         "apiVersion":"v1",
///         "myPlugin": {
///             "kind":"PluginA",
///             "aOption":"foo",
///         },
///     }
///
/// So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RawExtensionAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi027::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::runtime::RawExtension {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
impl crate::Optionable for RawExtensionAc {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::runtime::RawExtension {
    fn into_optioned(self) -> RawExtensionAc {
        RawExtensionAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: RawExtensionAc) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: RawExtensionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            self.0 = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::runtime::RawExtension>
for RawExtensionAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::runtime::RawExtension,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::apimachinery::pkg::runtime::RawExtension, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::runtime::RawExtension,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
