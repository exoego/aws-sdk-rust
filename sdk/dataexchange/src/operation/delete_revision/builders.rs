// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_revision::_delete_revision_output::DeleteRevisionOutputBuilder;

pub use crate::operation::delete_revision::_delete_revision_input::DeleteRevisionInputBuilder;

impl DeleteRevisionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_revision::DeleteRevisionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_revision::DeleteRevisionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_revision();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteRevision`.
///
/// <p>This operation deletes a revision.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRevisionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_revision::builders::DeleteRevisionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_revision::DeleteRevisionOutput,
        crate::operation::delete_revision::DeleteRevisionError,
    > for DeleteRevisionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_revision::DeleteRevisionOutput,
            crate::operation::delete_revision::DeleteRevisionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteRevisionFluentBuilder {
    /// Creates a new `DeleteRevision`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteRevision as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_revision::builders::DeleteRevisionInputBuilder {
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
        crate::operation::delete_revision::DeleteRevisionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_revision::DeleteRevisionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_revision::DeleteRevision::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_revision::DeleteRevision::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_revision::DeleteRevisionOutput,
        crate::operation::delete_revision::DeleteRevisionError,
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
    /// <p>The unique identifier for a data set.</p>
    pub fn data_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_set_id(input.into());
        self
    }
    /// <p>The unique identifier for a data set.</p>
    pub fn set_data_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_set_id(input);
        self
    }
    /// <p>The unique identifier for a data set.</p>
    pub fn get_data_set_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_set_id()
    }
    /// <p>The unique identifier for a revision.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>The unique identifier for a revision.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
    /// <p>The unique identifier for a revision.</p>
    pub fn get_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_revision_id()
    }
}
