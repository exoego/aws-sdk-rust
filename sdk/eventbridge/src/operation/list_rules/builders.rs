// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_rules::_list_rules_output::ListRulesOutputBuilder;

pub use crate::operation::list_rules::_list_rules_input::ListRulesInputBuilder;

impl ListRulesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_rules::ListRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_rules::ListRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_rules();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListRules`.
///
/// <p>Lists your Amazon EventBridge rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p>
/// <p>The maximum number of results per page for requests is 100.</p>
/// <p>ListRules does not list the targets of a rule. To see the targets associated with a rule, use <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_ListTargetsByRule.html">ListTargetsByRule</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListRulesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_rules::builders::ListRulesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_rules::ListRulesOutput, crate::operation::list_rules::ListRulesError>
    for ListRulesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_rules::ListRulesOutput, crate::operation::list_rules::ListRulesError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListRulesFluentBuilder {
    /// Creates a new `ListRules`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListRules as a reference.
    pub fn as_input(&self) -> &crate::operation::list_rules::builders::ListRulesInputBuilder {
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
        crate::operation::list_rules::ListRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_rules::ListRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_rules::ListRules::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_rules::ListRules::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_rules::ListRulesOutput,
        crate::operation::list_rules::ListRulesError,
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
    /// <p>The prefix matching the rule name.</p>
    pub fn name_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_prefix(input.into());
        self
    }
    /// <p>The prefix matching the rule name.</p>
    pub fn set_name_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_prefix(input);
        self
    }
    /// <p>The prefix matching the rule name.</p>
    pub fn get_name_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_prefix()
    }
    /// <p>The name or ARN of the event bus to list the rules for. If you omit this, the default event bus is used.</p>
    pub fn event_bus_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_bus_name(input.into());
        self
    }
    /// <p>The name or ARN of the event bus to list the rules for. If you omit this, the default event bus is used.</p>
    pub fn set_event_bus_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_bus_name(input);
        self
    }
    /// <p>The name or ARN of the event bus to list the rules for. If you omit this, the default event bus is used.</p>
    pub fn get_event_bus_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_bus_name()
    }
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
}
