// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_connection_status::_get_connection_status_output::GetConnectionStatusOutputBuilder;

pub use crate::operation::get_connection_status::_get_connection_status_input::GetConnectionStatusInputBuilder;

impl GetConnectionStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_connection_status::GetConnectionStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_connection_status::GetConnectionStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_connection_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetConnectionStatus`.
///
/// <p>Retrieves the Session Manager connection status for a managed node to determine whether it is running and ready to receive Session Manager connections.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetConnectionStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_connection_status::builders::GetConnectionStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_connection_status::GetConnectionStatusOutput,
        crate::operation::get_connection_status::GetConnectionStatusError,
    > for GetConnectionStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_connection_status::GetConnectionStatusOutput,
            crate::operation::get_connection_status::GetConnectionStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetConnectionStatusFluentBuilder {
    /// Creates a new `GetConnectionStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetConnectionStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_connection_status::builders::GetConnectionStatusInputBuilder {
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
        crate::operation::get_connection_status::GetConnectionStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_connection_status::GetConnectionStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_connection_status::GetConnectionStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_connection_status::GetConnectionStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_connection_status::GetConnectionStatusOutput,
        crate::operation::get_connection_status::GetConnectionStatusError,
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
    /// <p>The managed node ID.</p>
    pub fn target(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target(input.into());
        self
    }
    /// <p>The managed node ID.</p>
    pub fn set_target(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>The managed node ID.</p>
    pub fn get_target(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target()
    }
}
