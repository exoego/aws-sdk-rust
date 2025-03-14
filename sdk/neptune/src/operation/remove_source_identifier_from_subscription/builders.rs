// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_source_identifier_from_subscription::_remove_source_identifier_from_subscription_output::RemoveSourceIdentifierFromSubscriptionOutputBuilder;

pub use crate::operation::remove_source_identifier_from_subscription::_remove_source_identifier_from_subscription_input::RemoveSourceIdentifierFromSubscriptionInputBuilder;

impl RemoveSourceIdentifierFromSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_source_identifier_from_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveSourceIdentifierFromSubscription`.
///
/// <p>Removes a source identifier from an existing event notification subscription.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveSourceIdentifierFromSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_source_identifier_from_subscription::builders::RemoveSourceIdentifierFromSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionOutput,
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionError,
    > for RemoveSourceIdentifierFromSubscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionOutput,
            crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveSourceIdentifierFromSubscriptionFluentBuilder {
    /// Creates a new `RemoveSourceIdentifierFromSubscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveSourceIdentifierFromSubscription as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::remove_source_identifier_from_subscription::builders::RemoveSourceIdentifierFromSubscriptionInputBuilder {
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
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscription::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscription::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionOutput,
        crate::operation::remove_source_identifier_from_subscription::RemoveSourceIdentifierFromSubscriptionError,
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
    /// <p>The name of the event notification subscription you want to remove a source identifier from.</p>
    pub fn subscription_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subscription_name(input.into());
        self
    }
    /// <p>The name of the event notification subscription you want to remove a source identifier from.</p>
    pub fn set_subscription_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subscription_name(input);
        self
    }
    /// <p>The name of the event notification subscription you want to remove a source identifier from.</p>
    pub fn get_subscription_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subscription_name()
    }
    /// <p>The source identifier to be removed from the subscription, such as the <b>DB instance identifier</b> for a DB instance or the name of a security group.</p>
    pub fn source_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_identifier(input.into());
        self
    }
    /// <p>The source identifier to be removed from the subscription, such as the <b>DB instance identifier</b> for a DB instance or the name of a security group.</p>
    pub fn set_source_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_identifier(input);
        self
    }
    /// <p>The source identifier to be removed from the subscription, such as the <b>DB instance identifier</b> for a DB instance or the name of a security group.</p>
    pub fn get_source_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_identifier()
    }
}
