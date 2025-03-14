// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_core_network::_create_core_network_output::CreateCoreNetworkOutputBuilder;

pub use crate::operation::create_core_network::_create_core_network_input::CreateCoreNetworkInputBuilder;

impl CreateCoreNetworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_core_network::CreateCoreNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_core_network::CreateCoreNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_core_network();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCoreNetwork`.
///
/// <p>Creates a core network as part of your global network, and optionally, with a core network policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCoreNetworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_core_network::builders::CreateCoreNetworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_core_network::CreateCoreNetworkOutput,
        crate::operation::create_core_network::CreateCoreNetworkError,
    > for CreateCoreNetworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_core_network::CreateCoreNetworkOutput,
            crate::operation::create_core_network::CreateCoreNetworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCoreNetworkFluentBuilder {
    /// Creates a new `CreateCoreNetwork`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCoreNetwork as a reference.
    pub fn as_input(&self) -> &crate::operation::create_core_network::builders::CreateCoreNetworkInputBuilder {
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
        crate::operation::create_core_network::CreateCoreNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_core_network::CreateCoreNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_core_network::CreateCoreNetwork::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_core_network::CreateCoreNetwork::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_core_network::CreateCoreNetworkOutput,
        crate::operation::create_core_network::CreateCoreNetworkError,
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
    /// <p>The ID of the global network that a core network will be a part of.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network that a core network will be a part of.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the global network that a core network will be a part of.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_network_id()
    }
    /// <p>The description of a core network.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of a core network.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of a core network.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Key-value tags associated with a core network request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Key-value tags associated with a core network request.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Key-value tags associated with a core network request.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>The policy document for creating a core network.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_document(input.into());
        self
    }
    /// <p>The policy document for creating a core network.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_document(input);
        self
    }
    /// <p>The policy document for creating a core network.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_document()
    }
    /// <p>The client token associated with a core network request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client token associated with a core network request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The client token associated with a core network request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
