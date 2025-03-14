// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::configure_agent::_configure_agent_output::ConfigureAgentOutputBuilder;

pub use crate::operation::configure_agent::_configure_agent_input::ConfigureAgentInputBuilder;

impl ConfigureAgentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::configure_agent::ConfigureAgentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::configure_agent::ConfigureAgentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.configure_agent();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ConfigureAgent`.
///
/// <p>Used by profiler agents to report their current state and to receive remote configuration updates. For example, <code>ConfigureAgent</code> can be used to tell an agent whether to profile or not and for how long to return profiling data.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ConfigureAgentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::configure_agent::builders::ConfigureAgentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::configure_agent::ConfigureAgentOutput,
        crate::operation::configure_agent::ConfigureAgentError,
    > for ConfigureAgentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::configure_agent::ConfigureAgentOutput,
            crate::operation::configure_agent::ConfigureAgentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ConfigureAgentFluentBuilder {
    /// Creates a new `ConfigureAgent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ConfigureAgent as a reference.
    pub fn as_input(&self) -> &crate::operation::configure_agent::builders::ConfigureAgentInputBuilder {
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
        crate::operation::configure_agent::ConfigureAgentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::configure_agent::ConfigureAgentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::configure_agent::ConfigureAgent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::configure_agent::ConfigureAgent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::configure_agent::ConfigureAgentOutput,
        crate::operation::configure_agent::ConfigureAgentError,
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
    /// <p>The name of the profiling group for which the configured agent is collecting profiling data.</p>
    pub fn profiling_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profiling_group_name(input.into());
        self
    }
    /// <p>The name of the profiling group for which the configured agent is collecting profiling data.</p>
    pub fn set_profiling_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profiling_group_name(input);
        self
    }
    /// <p>The name of the profiling group for which the configured agent is collecting profiling data.</p>
    pub fn get_profiling_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profiling_group_name()
    }
    /// <p>A universally unique identifier (UUID) for a profiling instance. For example, if the profiling instance is an Amazon EC2 instance, it is the instance ID. If it is an AWS Fargate container, it is the container's task ID.</p>
    pub fn fleet_instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_instance_id(input.into());
        self
    }
    /// <p>A universally unique identifier (UUID) for a profiling instance. For example, if the profiling instance is an Amazon EC2 instance, it is the instance ID. If it is an AWS Fargate container, it is the container's task ID.</p>
    pub fn set_fleet_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_instance_id(input);
        self
    }
    /// <p>A universally unique identifier (UUID) for a profiling instance. For example, if the profiling instance is an Amazon EC2 instance, it is the instance ID. If it is an AWS Fargate container, it is the container's task ID.</p>
    pub fn get_fleet_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_instance_id()
    }
    /// Adds a key-value pair to `metadata`.
    ///
    /// To override the contents of this collection use [`set_metadata`](Self::set_metadata).
    ///
    /// <p>Metadata captured about the compute platform the agent is running on. It includes information about sampling and reporting. The valid fields are:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPUTE_PLATFORM</code> - The compute platform on which the agent is running</p></li>
    /// <li>
    /// <p><code>AGENT_ID</code> - The ID for an agent instance.</p></li>
    /// <li>
    /// <p><code>AWS_REQUEST_ID</code> - The AWS request ID of a Lambda invocation.</p></li>
    /// <li>
    /// <p><code>EXECUTION_ENVIRONMENT</code> - The execution environment a Lambda function is running on.</p></li>
    /// <li>
    /// <p><code>LAMBDA_FUNCTION_ARN</code> - The Amazon Resource Name (ARN) that is used to invoke a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_MEMORY_LIMIT_IN_MB</code> - The memory allocated to a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_REMAINING_TIME_IN_MILLISECONDS</code> - The time in milliseconds before execution of a Lambda function times out.</p></li>
    /// <li>
    /// <p><code>LAMBDA_TIME_GAP_BETWEEN_INVOKES_IN_MILLISECONDS</code> - The time in milliseconds between two invocations of a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_PREVIOUS_EXECUTION_TIME_IN_MILLISECONDS</code> - The time in milliseconds for the previous Lambda invocation.</p></li>
    /// </ul>
    pub fn metadata(mut self, k: crate::types::MetadataField, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metadata(k, v.into());
        self
    }
    /// <p>Metadata captured about the compute platform the agent is running on. It includes information about sampling and reporting. The valid fields are:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPUTE_PLATFORM</code> - The compute platform on which the agent is running</p></li>
    /// <li>
    /// <p><code>AGENT_ID</code> - The ID for an agent instance.</p></li>
    /// <li>
    /// <p><code>AWS_REQUEST_ID</code> - The AWS request ID of a Lambda invocation.</p></li>
    /// <li>
    /// <p><code>EXECUTION_ENVIRONMENT</code> - The execution environment a Lambda function is running on.</p></li>
    /// <li>
    /// <p><code>LAMBDA_FUNCTION_ARN</code> - The Amazon Resource Name (ARN) that is used to invoke a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_MEMORY_LIMIT_IN_MB</code> - The memory allocated to a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_REMAINING_TIME_IN_MILLISECONDS</code> - The time in milliseconds before execution of a Lambda function times out.</p></li>
    /// <li>
    /// <p><code>LAMBDA_TIME_GAP_BETWEEN_INVOKES_IN_MILLISECONDS</code> - The time in milliseconds between two invocations of a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_PREVIOUS_EXECUTION_TIME_IN_MILLISECONDS</code> - The time in milliseconds for the previous Lambda invocation.</p></li>
    /// </ul>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::MetadataField, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_metadata(input);
        self
    }
    /// <p>Metadata captured about the compute platform the agent is running on. It includes information about sampling and reporting. The valid fields are:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPUTE_PLATFORM</code> - The compute platform on which the agent is running</p></li>
    /// <li>
    /// <p><code>AGENT_ID</code> - The ID for an agent instance.</p></li>
    /// <li>
    /// <p><code>AWS_REQUEST_ID</code> - The AWS request ID of a Lambda invocation.</p></li>
    /// <li>
    /// <p><code>EXECUTION_ENVIRONMENT</code> - The execution environment a Lambda function is running on.</p></li>
    /// <li>
    /// <p><code>LAMBDA_FUNCTION_ARN</code> - The Amazon Resource Name (ARN) that is used to invoke a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_MEMORY_LIMIT_IN_MB</code> - The memory allocated to a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_REMAINING_TIME_IN_MILLISECONDS</code> - The time in milliseconds before execution of a Lambda function times out.</p></li>
    /// <li>
    /// <p><code>LAMBDA_TIME_GAP_BETWEEN_INVOKES_IN_MILLISECONDS</code> - The time in milliseconds between two invocations of a Lambda function.</p></li>
    /// <li>
    /// <p><code>LAMBDA_PREVIOUS_EXECUTION_TIME_IN_MILLISECONDS</code> - The time in milliseconds for the previous Lambda invocation.</p></li>
    /// </ul>
    pub fn get_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<crate::types::MetadataField, ::std::string::String>> {
        self.inner.get_metadata()
    }
}
