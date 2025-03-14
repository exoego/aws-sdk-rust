// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_compilation_jobs::_list_compilation_jobs_output::ListCompilationJobsOutputBuilder;

pub use crate::operation::list_compilation_jobs::_list_compilation_jobs_input::ListCompilationJobsInputBuilder;

impl ListCompilationJobsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_compilation_jobs::ListCompilationJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_compilation_jobs::ListCompilationJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_compilation_jobs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCompilationJobs`.
///
/// <p>Lists model compilation jobs that satisfy various filters.</p>
/// <p>To create a model compilation job, use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateCompilationJob.html">CreateCompilationJob</a>. To get information about a particular model compilation job you have created, use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DescribeCompilationJob.html">DescribeCompilationJob</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCompilationJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_compilation_jobs::builders::ListCompilationJobsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_compilation_jobs::ListCompilationJobsOutput,
        crate::operation::list_compilation_jobs::ListCompilationJobsError,
    > for ListCompilationJobsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_compilation_jobs::ListCompilationJobsOutput,
            crate::operation::list_compilation_jobs::ListCompilationJobsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCompilationJobsFluentBuilder {
    /// Creates a new `ListCompilationJobs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCompilationJobs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_compilation_jobs::builders::ListCompilationJobsInputBuilder {
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
        crate::operation::list_compilation_jobs::ListCompilationJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_compilation_jobs::ListCompilationJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_compilation_jobs::ListCompilationJobs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_compilation_jobs::ListCompilationJobs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_compilation_jobs::ListCompilationJobsOutput,
        crate::operation::list_compilation_jobs::ListCompilationJobsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_compilation_jobs::paginator::ListCompilationJobsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_compilation_jobs::paginator::ListCompilationJobsPaginator {
        crate::operation::list_compilation_jobs::paginator::ListCompilationJobsPaginator::new(self.handle, self.inner)
    }
    /// <p>If the result of the previous <code>ListCompilationJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model compilation jobs, use the token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous <code>ListCompilationJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model compilation jobs, use the token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the result of the previous <code>ListCompilationJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model compilation jobs, use the token in the next request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of model compilation jobs to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of model compilation jobs to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of model compilation jobs to return in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A filter that returns the model compilation jobs that were created after a specified time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were created after a specified time.</p>
    pub fn set_creation_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were created after a specified time.</p>
    pub fn get_creation_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_after()
    }
    /// <p>A filter that returns the model compilation jobs that were created before a specified time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were created before a specified time.</p>
    pub fn set_creation_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were created before a specified time.</p>
    pub fn get_creation_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_before()
    }
    /// <p>A filter that returns the model compilation jobs that were modified after a specified time.</p>
    pub fn last_modified_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_after(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were modified after a specified time.</p>
    pub fn set_last_modified_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_modified_time_after(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were modified after a specified time.</p>
    pub fn get_last_modified_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_modified_time_after()
    }
    /// <p>A filter that returns the model compilation jobs that were modified before a specified time.</p>
    pub fn last_modified_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_before(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were modified before a specified time.</p>
    pub fn set_last_modified_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_modified_time_before(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs that were modified before a specified time.</p>
    pub fn get_last_modified_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_modified_time_before()
    }
    /// <p>A filter that returns the model compilation jobs whose name contains a specified string.</p>
    pub fn name_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>A filter that returns the model compilation jobs whose name contains a specified string.</p>
    pub fn set_name_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>A filter that returns the model compilation jobs whose name contains a specified string.</p>
    pub fn get_name_contains(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_contains()
    }
    /// <p>A filter that retrieves model compilation jobs with a specific <code>CompilationJobStatus</code> status.</p>
    pub fn status_equals(mut self, input: crate::types::CompilationJobStatus) -> Self {
        self.inner = self.inner.status_equals(input);
        self
    }
    /// <p>A filter that retrieves model compilation jobs with a specific <code>CompilationJobStatus</code> status.</p>
    pub fn set_status_equals(mut self, input: ::std::option::Option<crate::types::CompilationJobStatus>) -> Self {
        self.inner = self.inner.set_status_equals(input);
        self
    }
    /// <p>A filter that retrieves model compilation jobs with a specific <code>CompilationJobStatus</code> status.</p>
    pub fn get_status_equals(&self) -> &::std::option::Option<crate::types::CompilationJobStatus> {
        self.inner.get_status_equals()
    }
    /// <p>The field by which to sort results. The default is <code>CreationTime</code>.</p>
    pub fn sort_by(mut self, input: crate::types::ListCompilationJobsSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The field by which to sort results. The default is <code>CreationTime</code>.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::ListCompilationJobsSortBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The field by which to sort results. The default is <code>CreationTime</code>.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::ListCompilationJobsSortBy> {
        self.inner.get_sort_by()
    }
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        self.inner.get_sort_order()
    }
}
