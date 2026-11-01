#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceDerivedAttribute defines a derived attribute computed via CEL.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceDerivedAttributeAc {
    /// Expression is a CEL expression evaluated against each candidate device. The expression must evaluate to a primitive scalar (string, integer, boolean, or semver) or a list of these scalars (\[\]string, \[\]int64, \[\]bool, \[\]semver) to act as a virtual grouping key. Any other return type is an error and causes CEL evaluation for the device to fail.
    ///
    /// The expression's input is an object named "device", which carries the same properties as in a CELDeviceSelector.
    ///
    /// When pod scheduling encounters CEL runtime errors (such as looking up an attribute that isn't defined) for some devices, it will abort allocation and fail scheduling for the Pod. Surfacing evaluation errors immediately prevents silent topology matching failures that are extremely hard to detect. A robust expression should, for example, check for the existence of attributes before referencing them to avoid runtime evaluation errors.
    ///
    /// The expression gets evaluated after a device has passed the other selector expressions for the request in which this expression is used. This allows writing expressions that are tailored towards the specific devices being requested (for example, by assuming the device is from a certain vendor and skipping those checks).
    ///
    /// The length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps; the combined cost of all derived attributes in a claim is capped by a shared CEL cost budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
    /// Name is the identifier for this derived attribute, used in constraints.
    ///
    /// It must be a DNS subdomain followed by a slash ("/") followed by a C identifier (e.g. "example.com/numaNode" or "derived/numaNode").
    ///
    /// If the chosen name matches an existing physical attribute from a driver, the derived attribute's expression will shadow the physical attribute, and its evaluated value will be used in constraints instead. When the goal is to define a derived attribute that is only used within the ResourceClaim and not meant to shadow an existing attribute, use a domain prefix that no DRA driver should be using (e.g. "derived/myAttribute").
    ///
    /// It is not valid to define a derived attribute that isn't used in at least one constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::resource::v1::DeviceDerivedAttribute {
    type Optioned = DeviceDerivedAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceDerivedAttributeAc {
    type Optioned = DeviceDerivedAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1::DeviceDerivedAttribute {
    fn into_optioned(self) -> DeviceDerivedAttributeAc {
        DeviceDerivedAttributeAc {
            expression: Some(self.expression),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: DeviceDerivedAttributeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value
                .expression
                .ok_or(crate::Error {
                    missing_field: "expression",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: DeviceDerivedAttributeAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            self.expression = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::resource::v1::DeviceDerivedAttribute>
for DeviceDerivedAttributeAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1::DeviceDerivedAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1::DeviceDerivedAttribute,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1::DeviceDerivedAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for DeviceDerivedAttributeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.expression, other.expression);
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
