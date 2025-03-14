// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_user::_delete_user_output::DeleteUserOutputBuilder;

pub use crate::operation::delete_user::_delete_user_input::DeleteUserInputBuilder;

impl DeleteUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_user::DeleteUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user::DeleteUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteUser`.
///
/// <p>Deletes the specified UserID within the collection. Faces that are associated with the UserID are disassociated from the UserID before deleting the specified UserID. If the specified <code>Collection</code> or <code>UserID</code> is already deleted or not found, a <code>ResourceNotFoundException</code> will be thrown. If the action is successful with a 200 response, an empty HTTP body is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_user::builders::DeleteUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_user::DeleteUserOutput,
        crate::operation::delete_user::DeleteUserError,
    > for DeleteUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_user::DeleteUserOutput,
            crate::operation::delete_user::DeleteUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteUserFluentBuilder {
    /// Creates a new `DeleteUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteUser as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_user::builders::DeleteUserInputBuilder {
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
        crate::operation::delete_user::DeleteUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user::DeleteUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_user::DeleteUser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_user::DeleteUser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_user::DeleteUserOutput,
        crate::operation::delete_user::DeleteUserError,
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
    /// <p>The ID of an existing collection from which the UserID needs to be deleted.</p>
    pub fn collection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.collection_id(input.into());
        self
    }
    /// <p>The ID of an existing collection from which the UserID needs to be deleted.</p>
    pub fn set_collection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_collection_id(input);
        self
    }
    /// <p>The ID of an existing collection from which the UserID needs to be deleted.</p>
    pub fn get_collection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_collection_id()
    }
    /// <p>ID for the UserID to be deleted.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>ID for the UserID to be deleted.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>ID for the UserID to be deleted.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>Idempotent token used to identify the request to <code>DeleteUser</code>. If you use the same token with multiple <code>DeleteUser </code>requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Idempotent token used to identify the request to <code>DeleteUser</code>. If you use the same token with multiple <code>DeleteUser </code>requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Idempotent token used to identify the request to <code>DeleteUser</code>. If you use the same token with multiple <code>DeleteUser </code>requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
}
