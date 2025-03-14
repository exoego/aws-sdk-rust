// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ops_metadata::_list_ops_metadata_output::ListOpsMetadataOutputBuilder;

pub use crate::operation::list_ops_metadata::_list_ops_metadata_input::ListOpsMetadataInputBuilder;

impl ListOpsMetadataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ops_metadata::ListOpsMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ops_metadata::ListOpsMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ops_metadata();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListOpsMetadata`.
///
/// <p>Amazon Web Services Systems Manager calls this API operation when displaying all Application Manager OpsMetadata objects or blobs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListOpsMetadataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ops_metadata::builders::ListOpsMetadataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ops_metadata::ListOpsMetadataOutput,
        crate::operation::list_ops_metadata::ListOpsMetadataError,
    > for ListOpsMetadataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ops_metadata::ListOpsMetadataOutput,
            crate::operation::list_ops_metadata::ListOpsMetadataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListOpsMetadataFluentBuilder {
    /// Creates a new `ListOpsMetadata`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListOpsMetadata as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ops_metadata::builders::ListOpsMetadataInputBuilder {
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
        crate::operation::list_ops_metadata::ListOpsMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ops_metadata::ListOpsMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ops_metadata::ListOpsMetadata::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ops_metadata::ListOpsMetadata::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_ops_metadata::ListOpsMetadataOutput,
        crate::operation::list_ops_metadata::ListOpsMetadataError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_ops_metadata::paginator::ListOpsMetadataPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_ops_metadata::paginator::ListOpsMetadataPaginator {
        crate::operation::list_ops_metadata::paginator::ListOpsMetadataPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters to limit the number of OpsMetadata objects returned by the call.</p>
    pub fn filters(mut self, input: crate::types::OpsMetadataFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters to limit the number of OpsMetadata objects returned by the call.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::OpsMetadataFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>One or more filters to limit the number of OpsMetadata objects returned by the call.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::OpsMetadataFilter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
