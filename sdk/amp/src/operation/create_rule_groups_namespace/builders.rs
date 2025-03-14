// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_rule_groups_namespace::_create_rule_groups_namespace_output::CreateRuleGroupsNamespaceOutputBuilder;

pub use crate::operation::create_rule_groups_namespace::_create_rule_groups_namespace_input::CreateRuleGroupsNamespaceInputBuilder;

impl CreateRuleGroupsNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_rule_groups_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRuleGroupsNamespace`.
///
/// <p>The <code>CreateRuleGroupsNamespace</code> operation creates a rule groups namespace within a workspace. A rule groups namespace is associated with exactly one rules file. A workspace can have multiple rule groups namespaces.</p>
/// <p>Use this operation only to create new rule groups namespaces. To update an existing rule groups namespace, use <code>PutRuleGroupsNamespace</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRuleGroupsNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_rule_groups_namespace::builders::CreateRuleGroupsNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceOutput,
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceError,
    > for CreateRuleGroupsNamespaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceOutput,
            crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRuleGroupsNamespaceFluentBuilder {
    /// Creates a new `CreateRuleGroupsNamespace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRuleGroupsNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::create_rule_groups_namespace::builders::CreateRuleGroupsNamespaceInputBuilder {
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
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceOutput,
        crate::operation::create_rule_groups_namespace::CreateRuleGroupsNamespaceError,
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
    /// <p>The ID of the workspace to add the rule groups namespace.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace to add the rule groups namespace.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// <p>The ID of the workspace to add the rule groups namespace.</p>
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workspace_id()
    }
    /// <p>The name for the new rule groups namespace.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the new rule groups namespace.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name for the new rule groups namespace.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The rules file to use in the new namespace.</p>
    /// <p>Contains the base64-encoded version of the YAML rules file.</p>
    /// <p>For details about the rule groups namespace structure, see <a href="https://docs.aws.amazon.com/prometheus/latest/APIReference/yaml-RuleGroupsNamespaceData.html">RuleGroupsNamespaceData</a>.</p>
    pub fn data(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.data(input);
        self
    }
    /// <p>The rules file to use in the new namespace.</p>
    /// <p>Contains the base64-encoded version of the YAML rules file.</p>
    /// <p>For details about the rule groups namespace structure, see <a href="https://docs.aws.amazon.com/prometheus/latest/APIReference/yaml-RuleGroupsNamespaceData.html">RuleGroupsNamespaceData</a>.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_data(input);
        self
    }
    /// <p>The rules file to use in the new namespace.</p>
    /// <p>Contains the base64-encoded version of the YAML rules file.</p>
    /// <p>For details about the rule groups namespace structure, see <a href="https://docs.aws.amazon.com/prometheus/latest/APIReference/yaml-RuleGroupsNamespaceData.html">RuleGroupsNamespaceData</a>.</p>
    pub fn get_data(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_data()
    }
    /// <p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tag keys and values to associate with the rule groups namespace.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The list of tag keys and values to associate with the rule groups namespace.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The list of tag keys and values to associate with the rule groups namespace.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
