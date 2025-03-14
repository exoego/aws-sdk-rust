// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_images::_describe_images_output::DescribeImagesOutputBuilder;

pub use crate::operation::describe_images::_describe_images_input::DescribeImagesInputBuilder;

impl DescribeImagesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_images::DescribeImagesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_images::DescribeImagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_images();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeImages`.
///
/// <p>Retrieves a list that describes one or more specified images, if the image names or image ARNs are provided. Otherwise, all images in the account are described.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeImagesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_images::builders::DescribeImagesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_images::DescribeImagesOutput,
        crate::operation::describe_images::DescribeImagesError,
    > for DescribeImagesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_images::DescribeImagesOutput,
            crate::operation::describe_images::DescribeImagesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeImagesFluentBuilder {
    /// Creates a new `DescribeImages`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeImages as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_images::builders::DescribeImagesInputBuilder {
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
        crate::operation::describe_images::DescribeImagesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_images::DescribeImagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_images::DescribeImages::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_images::DescribeImages::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_images::DescribeImagesOutput,
        crate::operation::describe_images::DescribeImagesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_images::paginator::DescribeImagesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_images::paginator::DescribeImagesPaginator {
        crate::operation::describe_images::paginator::DescribeImagesPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Names`.
    ///
    /// To override the contents of this collection use [`set_names`](Self::set_names).
    ///
    /// <p>The names of the public or private images to describe.</p>
    pub fn names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.names(input.into());
        self
    }
    /// <p>The names of the public or private images to describe.</p>
    pub fn set_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_names(input);
        self
    }
    /// <p>The names of the public or private images to describe.</p>
    pub fn get_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_names()
    }
    /// Appends an item to `Arns`.
    ///
    /// To override the contents of this collection use [`set_arns`](Self::set_arns).
    ///
    /// <p>The ARNs of the public, private, and shared images to describe.</p>
    pub fn arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arns(input.into());
        self
    }
    /// <p>The ARNs of the public, private, and shared images to describe.</p>
    pub fn set_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_arns(input);
        self
    }
    /// <p>The ARNs of the public, private, and shared images to describe.</p>
    pub fn get_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_arns()
    }
    /// <p>The type of image (public, private, or shared) to describe.</p>
    pub fn r#type(mut self, input: crate::types::VisibilityType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of image (public, private, or shared) to describe.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::VisibilityType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of image (public, private, or shared) to describe.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::VisibilityType> {
        self.inner.get_type()
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
