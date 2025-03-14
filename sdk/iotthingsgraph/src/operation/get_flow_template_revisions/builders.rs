// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_flow_template_revisions::_get_flow_template_revisions_output::GetFlowTemplateRevisionsOutputBuilder;

pub use crate::operation::get_flow_template_revisions::_get_flow_template_revisions_input::GetFlowTemplateRevisionsInputBuilder;

impl GetFlowTemplateRevisionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_flow_template_revisions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFlowTemplateRevisions`.
///
/// <p>Gets revisions of the specified workflow. Only the last 100 revisions are stored. If the workflow has been deprecated, this action will return revisions that occurred before the deprecation. This action won't work for workflows that have been deleted.</p>
#[deprecated(note = "since: 2022-08-30")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFlowTemplateRevisionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput,
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError,
    > for GetFlowTemplateRevisionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput,
            crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFlowTemplateRevisionsFluentBuilder {
    /// Creates a new `GetFlowTemplateRevisions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFlowTemplateRevisions as a reference.
    pub fn as_input(&self) -> &crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsInputBuilder {
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
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_flow_template_revisions::GetFlowTemplateRevisions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput,
        crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_flow_template_revisions::paginator::GetFlowTemplateRevisionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_flow_template_revisions::paginator::GetFlowTemplateRevisionsPaginator {
        crate::operation::get_flow_template_revisions::paginator::GetFlowTemplateRevisionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the workflow.</p>
    /// <p>The ID should be in the following format.</p>
    /// <p><code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code></p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the workflow.</p>
    /// <p>The ID should be in the following format.</p>
    /// <p><code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code></p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the workflow.</p>
    /// <p>The ID should be in the following format.</p>
    /// <p><code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code></p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
