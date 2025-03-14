// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::request_cancel_workflow_execution::_request_cancel_workflow_execution_output::RequestCancelWorkflowExecutionOutputBuilder;

pub use crate::operation::request_cancel_workflow_execution::_request_cancel_workflow_execution_input::RequestCancelWorkflowExecutionInputBuilder;

impl RequestCancelWorkflowExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.request_cancel_workflow_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RequestCancelWorkflowExecution`.
///
/// <p>Records a <code>WorkflowExecutionCancelRequested</code> event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution as a whole. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p><note>
/// <p>If the runId isn't specified, the <code>WorkflowExecutionCancelRequested</code> event is recorded in the history of the current open workflow execution with the specified workflowId in the domain.</p>
/// </note> <note>
/// <p>Because this action allows the workflow to properly clean up and gracefully close, it should be used instead of <code>TerminateWorkflowExecution</code> when possible.</p>
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
pub struct RequestCancelWorkflowExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::request_cancel_workflow_execution::builders::RequestCancelWorkflowExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionOutput,
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionError,
    > for RequestCancelWorkflowExecutionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionOutput,
            crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RequestCancelWorkflowExecutionFluentBuilder {
    /// Creates a new `RequestCancelWorkflowExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RequestCancelWorkflowExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::request_cancel_workflow_execution::builders::RequestCancelWorkflowExecutionInputBuilder {
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
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionOutput,
        crate::operation::request_cancel_workflow_execution::RequestCancelWorkflowExecutionError,
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
    /// <p>The name of the domain containing the workflow execution to cancel.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The name of the domain containing the workflow execution to cancel.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The name of the domain containing the workflow execution to cancel.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
    /// <p>The workflowId of the workflow execution to cancel.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_id(input.into());
        self
    }
    /// <p>The workflowId of the workflow execution to cancel.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_id(input);
        self
    }
    /// <p>The workflowId of the workflow execution to cancel.</p>
    pub fn get_workflow_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_id()
    }
    /// <p>The runId of the workflow execution to cancel.</p>
    pub fn run_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.run_id(input.into());
        self
    }
    /// <p>The runId of the workflow execution to cancel.</p>
    pub fn set_run_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_run_id(input);
        self
    }
    /// <p>The runId of the workflow execution to cancel.</p>
    pub fn get_run_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_run_id()
    }
}
