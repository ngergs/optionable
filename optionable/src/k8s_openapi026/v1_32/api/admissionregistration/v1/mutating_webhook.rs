#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingWebhookAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admission_review_versions: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_config: Option<
        <::k8s_openapi026::api::admissionregistration::v1::WebhookClientConfig as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_conditions: <Option<
        std::vec::Vec<::k8s_openapi026::api::admissionregistration::v1::MatchCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_selector: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: <Option<
        std::vec::Vec<
            ::k8s_openapi026::api::admissionregistration::v1::RuleWithOperations,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::admissionregistration::v1::MutatingWebhook {
    type Optioned = MutatingWebhookAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingWebhookAc {
    type Optioned = MutatingWebhookAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::admissionregistration::v1::MutatingWebhook {
    fn into_optioned(self) -> MutatingWebhookAc {
        MutatingWebhookAc {
            admission_review_versions: Some(
                crate::OptionableConvert::into_optioned(self.admission_review_versions),
            ),
            client_config: Some(
                crate::OptionableConvert::into_optioned(self.client_config),
            ),
            failure_policy: crate::OptionableConvert::into_optioned(self.failure_policy),
            match_conditions: crate::OptionableConvert::into_optioned(
                self.match_conditions,
            ),
            match_policy: crate::OptionableConvert::into_optioned(self.match_policy),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            object_selector: crate::OptionableConvert::into_optioned(
                self.object_selector,
            ),
            reinvocation_policy: crate::OptionableConvert::into_optioned(
                self.reinvocation_policy,
            ),
            rules: crate::OptionableConvert::into_optioned(self.rules),
            side_effects: Some(
                crate::OptionableConvert::into_optioned(self.side_effects),
            ),
            timeout_seconds: crate::OptionableConvert::into_optioned(
                self.timeout_seconds,
            ),
        }
    }
    fn try_from_optioned(value: MutatingWebhookAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admission_review_versions: crate::OptionableConvert::try_from_optioned(
                value
                    .admission_review_versions
                    .ok_or(crate::Error {
                        missing_field: "admission_review_versions",
                    })?,
            )?,
            client_config: crate::OptionableConvert::try_from_optioned(
                value
                    .client_config
                    .ok_or(crate::Error {
                        missing_field: "client_config",
                    })?,
            )?,
            failure_policy: crate::OptionableConvert::try_from_optioned(
                value.failure_policy,
            )?,
            match_conditions: crate::OptionableConvert::try_from_optioned(
                value.match_conditions,
            )?,
            match_policy: crate::OptionableConvert::try_from_optioned(
                value.match_policy,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            object_selector: crate::OptionableConvert::try_from_optioned(
                value.object_selector,
            )?,
            reinvocation_policy: crate::OptionableConvert::try_from_optioned(
                value.reinvocation_policy,
            )?,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
            side_effects: crate::OptionableConvert::try_from_optioned(
                value
                    .side_effects
                    .ok_or(crate::Error {
                        missing_field: "side_effects",
                    })?,
            )?,
            timeout_seconds: crate::OptionableConvert::try_from_optioned(
                value.timeout_seconds,
            )?,
        })
    }
    fn merge(&mut self, other: MutatingWebhookAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.admission_review_versions {
            crate::OptionableConvert::merge(
                &mut self.admission_review_versions,
                other_value,
            )?;
        }
        if let Some(other_value) = other.client_config {
            crate::OptionableConvert::merge(&mut self.client_config, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.failure_policy, other.failure_policy)?;
        crate::OptionableConvert::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        crate::OptionableConvert::merge(&mut self.match_policy, other.match_policy)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        crate::OptionableConvert::merge(
            &mut self.object_selector,
            other.object_selector,
        )?;
        crate::OptionableConvert::merge(
            &mut self.reinvocation_policy,
            other.reinvocation_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        if let Some(other_value) = other.side_effects {
            crate::OptionableConvert::merge(&mut self.side_effects, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::admissionregistration::v1::MutatingWebhook,
> for MutatingWebhookAc {
    fn from_optionable(
        value: k8s_openapi026::api::admissionregistration::v1::MutatingWebhook,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::admissionregistration::v1::MutatingWebhook,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::admissionregistration::v1::MutatingWebhook,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
