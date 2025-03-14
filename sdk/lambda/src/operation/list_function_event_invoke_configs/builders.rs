// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_function_event_invoke_configs::_list_function_event_invoke_configs_output::ListFunctionEventInvokeConfigsOutputBuilder;

pub use crate::operation::list_function_event_invoke_configs::_list_function_event_invoke_configs_input::ListFunctionEventInvokeConfigsInputBuilder;

impl ListFunctionEventInvokeConfigsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_function_event_invoke_configs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListFunctionEventInvokeConfigs`.
///
/// <p>Retrieves a list of configurations for asynchronous invocation for a function.</p>
/// <p>To configure options for asynchronous invocation, use <code>PutFunctionEventInvokeConfig</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListFunctionEventInvokeConfigsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput,
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
    > for ListFunctionEventInvokeConfigsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput,
            crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListFunctionEventInvokeConfigsFluentBuilder {
    /// Creates a new `ListFunctionEventInvokeConfigs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListFunctionEventInvokeConfigs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsInputBuilder {
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
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput,
        crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_function_event_invoke_configs::paginator::ListFunctionEventInvokeConfigsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_function_event_invoke_configs::paginator::ListFunctionEventInvokeConfigsPaginator {
        crate::operation::list_function_event_invoke_configs::paginator::ListFunctionEventInvokeConfigsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name or ARN of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name or ARN of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name or ARN of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of configurations to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of configurations to return.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of configurations to return.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
}
