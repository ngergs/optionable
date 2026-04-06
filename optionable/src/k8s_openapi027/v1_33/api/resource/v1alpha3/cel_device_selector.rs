#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CELDeviceSelector contains a CEL expression for selecting a device.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CELDeviceSelectorAc {
    /// Expression is a CEL expression which evaluates a single device. It must evaluate to true when the device under consideration satisfies the desired criteria, and false when it does not. Any other result is an error and causes allocation of devices to abort.
    ///
    /// The expression's input is an object named "device", which carries the following properties:
    ///  - driver (string): the name of the driver which defines this device.
    ///  - attributes (map\[string\]object): the device's attributes, grouped by prefix
    ///    (e.g. device.attributes\["dra.example.com"\] evaluates to an object with all
    ///    of the attributes which were prefixed by "dra.example.com".
    ///  - capacity (map\[string\]object): the device's capacities, grouped by prefix.
    ///
    /// Example: Consider a device with driver="dra.example.com", which exposes two attributes named "model" and "ext.example.com/family" and which exposes one capacity named "modules". This input to this expression would have the following fields:
    ///
    ///   device.driver
    ///     device.attributes\["dra.example.com"\].model
    ///     device.attributes\["ext.example.com"\].family
    ///     device.capacity\["dra.example.com"\].modules
    ///
    /// The device.driver field can be used to check for a specific driver, either as a high-level precondition (i.e. you only want to consider devices from this driver) or as part of a multi-clause expression that is meant to consider devices from different drivers.
    ///
    /// The value type of each attribute is defined by the device definition, and users who write these expressions must consult the documentation for their specific drivers. The value type of each capacity is Quantity.
    ///
    /// If an unknown prefix is used as a lookup in either device.attributes or device.capacity, an empty map will be returned. Any reference to an unknown field will cause an evaluation error and allocation to abort.
    ///
    /// A robust expression should check for the existence of attributes before referencing them.
    ///
    /// For ease of use, the cel.bind() function is enabled, and can be used to simplify expressions that access multiple attributes with the same domain. For example:
    ///
    ///   cel.bind(dra, device.attributes\["dra.example.com"\], dra.someBool && dra.anotherBool)
    ///
    /// The length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for CELDeviceSelectorAc {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector {
    fn into_optioned(self) -> CELDeviceSelectorAc {
        CELDeviceSelectorAc {
            expression: Some(self.expression),
        }
    }
    fn try_from_optioned(value: CELDeviceSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value
                .expression
                .ok_or(crate::Error {
                    missing_field: "expression",
                })?,
        })
    }
    fn merge(&mut self, other: CELDeviceSelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            self.expression = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector>
for CELDeviceSelectorAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::CELDeviceSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
