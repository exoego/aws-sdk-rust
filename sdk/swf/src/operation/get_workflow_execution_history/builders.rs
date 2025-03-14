// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_workflow_execution_history::_get_workflow_execution_history_output::GetWorkflowExecutionHistoryOutputBuilder;

pub use crate::operation::get_workflow_execution_history::_get_workflow_execution_history_input::GetWorkflowExecutionHistoryInputBuilder;

impl GetWorkflowExecutionHistoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_workflow_execution_history();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetWorkflowExecutionHistory`.
///
/// <p>Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p><note>
/// <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p>
/// </note>
/// <p><b>Access Control</b></p>
/// <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p>
/// <ul>
/// <li>
/// <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p></li>
/// <li>
/// <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p></li>
/// <li>
/// <p>You cannot use an IAM policy to constrain this action's parameters.</p></li>
/// </ul>
/// <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetWorkflowExecutionHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_workflow_execution_history::builders::GetWorkflowExecutionHistoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryOutput,
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryError,
    > for GetWorkflowExecutionHistoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryOutput,
            crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetWorkflowExecutionHistoryFluentBuilder {
    /// Creates a new `GetWorkflowExecutionHistory`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetWorkflowExecutionHistory as a reference.
    pub fn as_input(&self) -> &crate::operation::get_workflow_execution_history::builders::GetWorkflowExecutionHistoryInputBuilder {
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
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryOutput,
        crate::operation::get_workflow_execution_history::GetWorkflowExecutionHistoryError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_workflow_execution_history::paginator::GetWorkflowExecutionHistoryPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_workflow_execution_history::paginator::GetWorkflowExecutionHistoryPaginator {
        crate::operation::get_workflow_execution_history::paginator::GetWorkflowExecutionHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the domain containing the workflow execution.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The name of the domain containing the workflow execution.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The name of the domain containing the workflow execution.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
    /// <p>Specifies the workflow execution for which to return the history.</p>
    pub fn execution(mut self, input: crate::types::WorkflowExecution) -> Self {
        self.inner = self.inner.execution(input);
        self
    }
    /// <p>Specifies the workflow execution for which to return the history.</p>
    pub fn set_execution(mut self, input: ::std::option::Option<crate::types::WorkflowExecution>) -> Self {
        self.inner = self.inner.set_execution(input);
        self
    }
    /// <p>Specifies the workflow execution for which to return the history.</p>
    pub fn get_execution(&self) -> &::std::option::Option<crate::types::WorkflowExecution> {
        self.inner.get_execution()
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>".</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn next_page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_page_token(input.into());
        self
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>".</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn set_next_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_page_token(input);
        self
    }
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>".</p>
    /// <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    pub fn get_next_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_page_token()
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results.</p>
    pub fn maximum_page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.maximum_page_size(input);
        self
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results.</p>
    pub fn set_maximum_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_maximum_page_size(input);
        self
    }
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results.</p>
    pub fn get_maximum_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_maximum_page_size()
    }
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimeStamp</code> of the events.</p>
    pub fn reverse_order(mut self, input: bool) -> Self {
        self.inner = self.inner.reverse_order(input);
        self
    }
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimeStamp</code> of the events.</p>
    pub fn set_reverse_order(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_reverse_order(input);
        self
    }
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimeStamp</code> of the events.</p>
    pub fn get_reverse_order(&self) -> &::std::option::Option<bool> {
        self.inner.get_reverse_order()
    }
}
