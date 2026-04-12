#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
///
/// The serialization format is:
///
///  \<quantity\>        ::= \<signedNumber\>\<suffix\>
///
///   (Note that \<suffix\> may be empty, from the "" case in \<decimalSI\>.)
///
/// \<digit\>           ::= 0 | 1 | ... | 9 \<digits\>          ::= \<digit\> | \<digit\>\<digits\> \<number\>          ::= \<digits\> | \<digits\>.\<digits\> | \<digits\>. | .\<digits\> \<sign\>            ::= "+" | "-" \<signedNumber\>    ::= \<number\> | \<sign\>\<number\> \<suffix\>          ::= \<binarySI\> | \<decimalExponent\> | \<decimalSI\> \<binarySI\>        ::= Ki | Mi | Gi | Ti | Pi | Ei
///
///   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
///
/// \<decimalSI\>       ::= m | "" | k | M | G | T | P | E
///
///   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
///
/// \<decimalExponent\> ::= "e" \<signedNumber\> | "E" \<signedNumber\>
///
/// No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
///
/// When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
///
/// Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
///
/// - No precision is lost - No fractional digits will be emitted - The exponent (or suffix) is as large as possible.
///
/// The sign will be omitted unless the number is negative.
///
/// Examples:
///
/// - 1.5 will be serialized as "1500m" - 1.5Gi will be serialized as "1536Mi"
///
/// Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
///
/// Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
///
/// This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[serde(from = "crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc")]
pub struct QuantityAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<std::string::String>,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::api::resource::Quantity {
    type Optioned = QuantityAc;
}
#[automatically_derived]
impl crate::Optionable for QuantityAc {
    type Optioned = QuantityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::api::resource::Quantity {
    fn into_optioned(self) -> QuantityAc {
        QuantityAc(Some(self.0))
    }
    fn try_from_optioned(value: QuantityAc) -> Result<Self, crate::Error> {
        Ok(Self(value.0.ok_or(crate::Error { missing_field: "0" })?))
    }
    fn merge(&mut self, other: QuantityAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            self.0 = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::api::resource::Quantity>
for QuantityAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl From<crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc>
for QuantityAc {
    fn from(
        value: crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc,
    ) -> Self {
        QuantityAc(
            match value {
                crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc::Int(
                    i,
                ) => i.map(|i| i.to_string()),
                crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc::String(
                    s,
                ) => s,
            },
        )
    }
}
