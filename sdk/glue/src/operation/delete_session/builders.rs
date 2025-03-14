// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_session::_delete_session_output::DeleteSessionOutputBuilder;

pub use crate::operation::delete_session::_delete_session_input::DeleteSessionInputBuilder;

impl DeleteSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_session::DeleteSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_session::DeleteSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteSession`.
///
/// <p>Deletes the session.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_session::builders::DeleteSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_session::DeleteSessionOutput,
        crate::operation::delete_session::DeleteSessionError,
    > for DeleteSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_session::DeleteSessionOutput,
            crate::operation::delete_session::DeleteSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteSessionFluentBuilder {
    /// Creates a new `DeleteSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteSession as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_session::builders::DeleteSessionInputBuilder {
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
        crate::operation::delete_session::DeleteSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_session::DeleteSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_session::DeleteSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_session::DeleteSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_session::DeleteSessionOutput,
        crate::operation::delete_session::DeleteSessionError,
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
    /// <p>The ID of the session to be deleted.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the session to be deleted.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the session to be deleted.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The name of the origin of the delete session request.</p>
    pub fn request_origin(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.request_origin(input.into());
        self
    }
    /// <p>The name of the origin of the delete session request.</p>
    pub fn set_request_origin(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_request_origin(input);
        self
    }
    /// <p>The name of the origin of the delete session request.</p>
    pub fn get_request_origin(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_request_origin()
    }
}
