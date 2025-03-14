// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::retrieve::_retrieve_output::RetrieveOutputBuilder;

pub use crate::operation::retrieve::_retrieve_input::RetrieveInputBuilder;

impl RetrieveInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::retrieve::RetrieveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::retrieve::RetrieveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.retrieve();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Retrieve`.
///
/// <p>Queries a knowledge base and retrieves information from it.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RetrieveFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::retrieve::builders::RetrieveInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::retrieve::RetrieveOutput, crate::operation::retrieve::RetrieveError>
    for RetrieveFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::retrieve::RetrieveOutput, crate::operation::retrieve::RetrieveError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RetrieveFluentBuilder {
    /// Creates a new `Retrieve`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Retrieve as a reference.
    pub fn as_input(&self) -> &crate::operation::retrieve::builders::RetrieveInputBuilder {
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
        crate::operation::retrieve::RetrieveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::retrieve::RetrieveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::retrieve::Retrieve::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::retrieve::Retrieve::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::retrieve::RetrieveOutput, crate::operation::retrieve::RetrieveError, Self>
    {
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
    /// Paginators are used by calling [`send().await`](crate::operation::retrieve::paginator::RetrievePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::retrieve::paginator::RetrievePaginator {
        crate::operation::retrieve::paginator::RetrievePaginator::new(self.handle, self.inner)
    }
    /// <p>The unique identifier of the knowledge base to query.</p>
    pub fn knowledge_base_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.knowledge_base_id(input.into());
        self
    }
    /// <p>The unique identifier of the knowledge base to query.</p>
    pub fn set_knowledge_base_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_knowledge_base_id(input);
        self
    }
    /// <p>The unique identifier of the knowledge base to query.</p>
    pub fn get_knowledge_base_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_knowledge_base_id()
    }
    /// <p>The query to send the knowledge base.</p>
    pub fn retrieval_query(mut self, input: crate::types::KnowledgeBaseQuery) -> Self {
        self.inner = self.inner.retrieval_query(input);
        self
    }
    /// <p>The query to send the knowledge base.</p>
    pub fn set_retrieval_query(mut self, input: ::std::option::Option<crate::types::KnowledgeBaseQuery>) -> Self {
        self.inner = self.inner.set_retrieval_query(input);
        self
    }
    /// <p>The query to send the knowledge base.</p>
    pub fn get_retrieval_query(&self) -> &::std::option::Option<crate::types::KnowledgeBaseQuery> {
        self.inner.get_retrieval_query()
    }
    /// <p>Contains details about how the results should be returned.</p>
    pub fn retrieval_configuration(mut self, input: crate::types::KnowledgeBaseRetrievalConfiguration) -> Self {
        self.inner = self.inner.retrieval_configuration(input);
        self
    }
    /// <p>Contains details about how the results should be returned.</p>
    pub fn set_retrieval_configuration(mut self, input: ::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration>) -> Self {
        self.inner = self.inner.set_retrieval_configuration(input);
        self
    }
    /// <p>Contains details about how the results should be returned.</p>
    pub fn get_retrieval_configuration(&self) -> &::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration> {
        self.inner.get_retrieval_configuration()
    }
    /// <p>If there are more results than can fit in the response, the response returns a <code>nextToken</code>. Use this token in the <code>nextToken</code> field of another request to retrieve the next batch of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If there are more results than can fit in the response, the response returns a <code>nextToken</code>. Use this token in the <code>nextToken</code> field of another request to retrieve the next batch of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If there are more results than can fit in the response, the response returns a <code>nextToken</code>. Use this token in the <code>nextToken</code> field of another request to retrieve the next batch of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
