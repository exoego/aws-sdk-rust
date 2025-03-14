// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_bridge_sources::_add_bridge_sources_output::AddBridgeSourcesOutputBuilder;

pub use crate::operation::add_bridge_sources::_add_bridge_sources_input::AddBridgeSourcesInputBuilder;

impl AddBridgeSourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_bridge_sources::AddBridgeSourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_bridge_sources::AddBridgeSourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_bridge_sources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddBridgeSources`.
///
/// Adds sources to an existing bridge.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddBridgeSourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_bridge_sources::builders::AddBridgeSourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_bridge_sources::AddBridgeSourcesOutput,
        crate::operation::add_bridge_sources::AddBridgeSourcesError,
    > for AddBridgeSourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_bridge_sources::AddBridgeSourcesOutput,
            crate::operation::add_bridge_sources::AddBridgeSourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddBridgeSourcesFluentBuilder {
    /// Creates a new `AddBridgeSources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddBridgeSources as a reference.
    pub fn as_input(&self) -> &crate::operation::add_bridge_sources::builders::AddBridgeSourcesInputBuilder {
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
        crate::operation::add_bridge_sources::AddBridgeSourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_bridge_sources::AddBridgeSourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_bridge_sources::AddBridgeSources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_bridge_sources::AddBridgeSources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_bridge_sources::AddBridgeSourcesOutput,
        crate::operation::add_bridge_sources::AddBridgeSourcesError,
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
    /// The ARN of the bridge that you want to update.
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bridge_arn(input.into());
        self
    }
    /// The ARN of the bridge that you want to update.
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bridge_arn(input);
        self
    }
    /// The ARN of the bridge that you want to update.
    pub fn get_bridge_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bridge_arn()
    }
    /// Appends an item to `Sources`.
    ///
    /// To override the contents of this collection use [`set_sources`](Self::set_sources).
    ///
    /// The sources that you want to add to this bridge.
    pub fn sources(mut self, input: crate::types::AddBridgeSourceRequest) -> Self {
        self.inner = self.inner.sources(input);
        self
    }
    /// The sources that you want to add to this bridge.
    pub fn set_sources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddBridgeSourceRequest>>) -> Self {
        self.inner = self.inner.set_sources(input);
        self
    }
    /// The sources that you want to add to this bridge.
    pub fn get_sources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddBridgeSourceRequest>> {
        self.inner.get_sources()
    }
}
