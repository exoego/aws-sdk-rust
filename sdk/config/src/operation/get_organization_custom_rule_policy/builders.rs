// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_organization_custom_rule_policy::_get_organization_custom_rule_policy_output::GetOrganizationCustomRulePolicyOutputBuilder;

pub use crate::operation::get_organization_custom_rule_policy::_get_organization_custom_rule_policy_input::GetOrganizationCustomRulePolicyInputBuilder;

impl GetOrganizationCustomRulePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_organization_custom_rule_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetOrganizationCustomRulePolicy`.
///
/// <p>Returns the policy definition containing the logic for your organization Config Custom Policy rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetOrganizationCustomRulePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
    > for GetOrganizationCustomRulePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetOrganizationCustomRulePolicyFluentBuilder {
    /// Creates a new `GetOrganizationCustomRulePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetOrganizationCustomRulePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
        crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of your organization Config Custom Policy rule.</p>
    pub fn organization_config_rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_config_rule_name(input.into());
        self
    }
    /// <p>The name of your organization Config Custom Policy rule.</p>
    pub fn set_organization_config_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_config_rule_name(input);
        self
    }
    /// <p>The name of your organization Config Custom Policy rule.</p>
    pub fn get_organization_config_rule_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_config_rule_name()
    }
}
