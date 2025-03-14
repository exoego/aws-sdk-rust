// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_objects::_describe_objects_output::DescribeObjectsOutputBuilder;

pub use crate::operation::describe_objects::_describe_objects_input::DescribeObjectsInputBuilder;

impl DescribeObjectsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_objects::DescribeObjectsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_objects::DescribeObjectsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_objects();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeObjects`.
///
/// <p>Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object.</p><examples>
/// <request>
/// POST / HTTP/1.1 Content-Type: application/x-amz-json-1.1 X-Amz-Target: DataPipeline.DescribeObjects Content-Length: 98 Host: datapipeline.us-east-1.amazonaws.com X-Amz-Date: Mon, 12 Nov 2012 17:49:52 GMT Authorization: AuthParams {"pipelineId": "df-06372391ZG65EXAMPLE", "objectIds": ["Schedule"], "evaluateExpressions": true}
/// </request>
/// <response>
/// x-amzn-RequestId: 4c18ea5d-0777-11e2-8a14-21bb8a1f50ef Content-Type: application/x-amz-json-1.1 Content-Length: 1488 Date: Mon, 12 Nov 2012 17:50:53 GMT {"hasMoreResults": false, "pipelineObjects": [ {"fields": [ {"key": "startDateTime", "stringValue": "2012-12-12T00:00:00"}, {"key": "parent", "refValue": "Default"}, {"key": "@sphere", "stringValue": "COMPONENT"}, {"key": "type", "stringValue": "Schedule"}, {"key": "period", "stringValue": "1 hour"}, {"key": "endDateTime", "stringValue": "2012-12-21T18:00:00"}, {"key": "@version", "stringValue": "1"}, {"key": "@status", "stringValue": "PENDING"}, {"key": "@pipelineId", "stringValue": "df-06372391ZG65EXAMPLE"} ], "id": "Schedule", "name": "Schedule"} ] }
/// </response>
/// </examples>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeObjectsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_objects::builders::DescribeObjectsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_objects::DescribeObjectsOutput,
        crate::operation::describe_objects::DescribeObjectsError,
    > for DescribeObjectsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_objects::DescribeObjectsOutput,
            crate::operation::describe_objects::DescribeObjectsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeObjectsFluentBuilder {
    /// Creates a new `DescribeObjects`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeObjects as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_objects::builders::DescribeObjectsInputBuilder {
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
        crate::operation::describe_objects::DescribeObjectsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_objects::DescribeObjectsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_objects::DescribeObjects::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_objects::DescribeObjects::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_objects::DescribeObjectsOutput,
        crate::operation::describe_objects::DescribeObjectsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_objects::paginator::DescribeObjectsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_objects::paginator::DescribeObjectsPaginator {
        crate::operation::describe_objects::paginator::DescribeObjectsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the pipeline that contains the object definitions.</p>
    pub fn pipeline_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pipeline_id(input.into());
        self
    }
    /// <p>The ID of the pipeline that contains the object definitions.</p>
    pub fn set_pipeline_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pipeline_id(input);
        self
    }
    /// <p>The ID of the pipeline that contains the object definitions.</p>
    pub fn get_pipeline_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pipeline_id()
    }
    /// Appends an item to `objectIds`.
    ///
    /// To override the contents of this collection use [`set_object_ids`](Self::set_object_ids).
    ///
    /// <p>The IDs of the pipeline objects that contain the definitions to be described. You can pass as many as 25 identifiers in a single call to <code>DescribeObjects</code>.</p>
    pub fn object_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.object_ids(input.into());
        self
    }
    /// <p>The IDs of the pipeline objects that contain the definitions to be described. You can pass as many as 25 identifiers in a single call to <code>DescribeObjects</code>.</p>
    pub fn set_object_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_object_ids(input);
        self
    }
    /// <p>The IDs of the pipeline objects that contain the definitions to be described. You can pass as many as 25 identifiers in a single call to <code>DescribeObjects</code>.</p>
    pub fn get_object_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_object_ids()
    }
    /// <p>Indicates whether any expressions in the object should be evaluated when the object descriptions are returned.</p>
    pub fn evaluate_expressions(mut self, input: bool) -> Self {
        self.inner = self.inner.evaluate_expressions(input);
        self
    }
    /// <p>Indicates whether any expressions in the object should be evaluated when the object descriptions are returned.</p>
    pub fn set_evaluate_expressions(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_evaluate_expressions(input);
        self
    }
    /// <p>Indicates whether any expressions in the object should be evaluated when the object descriptions are returned.</p>
    pub fn get_evaluate_expressions(&self) -> &::std::option::Option<bool> {
        self.inner.get_evaluate_expressions()
    }
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>DescribeObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>DescribeObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>DescribeObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
