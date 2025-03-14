// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_message_move_task::_start_message_move_task_output::StartMessageMoveTaskOutputBuilder;

pub use crate::operation::start_message_move_task::_start_message_move_task_input::StartMessageMoveTaskInputBuilder;

impl StartMessageMoveTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_message_move_task::StartMessageMoveTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_message_move_task::StartMessageMoveTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_message_move_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartMessageMoveTask`.
///
/// <p>Starts an asynchronous task to move messages from a specified source queue to a specified destination queue.</p><note>
/// <ul>
/// <li>
/// <p>This action is currently limited to supporting message redrive from queues that are configured as <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">dead-letter queues (DLQs)</a> of other Amazon SQS queues only. Non-SQS queue sources of dead-letter queues, such as Lambda or Amazon SNS topics, are currently not supported.</p></li>
/// <li>
/// <p>In dead-letter queues redrive context, the <code>StartMessageMoveTask</code> the source queue is the DLQ, while the destination queue can be the original source queue (from which the messages were driven to the dead-letter-queue), or a custom destination queue.</p></li>
/// <li>
/// <p>Currently, only standard queues support redrive. FIFO queues don't support redrive.</p></li>
/// <li>
/// <p>Only one active message movement task is supported per queue at any given time.</p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartMessageMoveTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_message_move_task::builders::StartMessageMoveTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_message_move_task::StartMessageMoveTaskOutput,
        crate::operation::start_message_move_task::StartMessageMoveTaskError,
    > for StartMessageMoveTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_message_move_task::StartMessageMoveTaskOutput,
            crate::operation::start_message_move_task::StartMessageMoveTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartMessageMoveTaskFluentBuilder {
    /// Creates a new `StartMessageMoveTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartMessageMoveTask as a reference.
    pub fn as_input(&self) -> &crate::operation::start_message_move_task::builders::StartMessageMoveTaskInputBuilder {
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
        crate::operation::start_message_move_task::StartMessageMoveTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_message_move_task::StartMessageMoveTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_message_move_task::StartMessageMoveTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_message_move_task::StartMessageMoveTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_message_move_task::StartMessageMoveTaskOutput,
        crate::operation::start_message_move_task::StartMessageMoveTaskError,
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
    /// <p>The ARN of the queue that contains the messages to be moved to another queue. Currently, only ARNs of dead-letter queues (DLQs) whose sources are other Amazon SQS queues are accepted. DLQs whose sources are non-SQS queues, such as Lambda or Amazon SNS topics, are not currently supported.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_arn(input.into());
        self
    }
    /// <p>The ARN of the queue that contains the messages to be moved to another queue. Currently, only ARNs of dead-letter queues (DLQs) whose sources are other Amazon SQS queues are accepted. DLQs whose sources are non-SQS queues, such as Lambda or Amazon SNS topics, are not currently supported.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_arn(input);
        self
    }
    /// <p>The ARN of the queue that contains the messages to be moved to another queue. Currently, only ARNs of dead-letter queues (DLQs) whose sources are other Amazon SQS queues are accepted. DLQs whose sources are non-SQS queues, such as Lambda or Amazon SNS topics, are not currently supported.</p>
    pub fn get_source_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_arn()
    }
    /// <p>The ARN of the queue that receives the moved messages. You can use this field to specify the destination queue where you would like to redrive messages. If this field is left blank, the messages will be redriven back to their respective original source queues.</p>
    pub fn destination_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_arn(input.into());
        self
    }
    /// <p>The ARN of the queue that receives the moved messages. You can use this field to specify the destination queue where you would like to redrive messages. If this field is left blank, the messages will be redriven back to their respective original source queues.</p>
    pub fn set_destination_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_arn(input);
        self
    }
    /// <p>The ARN of the queue that receives the moved messages. You can use this field to specify the destination queue where you would like to redrive messages. If this field is left blank, the messages will be redriven back to their respective original source queues.</p>
    pub fn get_destination_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_arn()
    }
    /// <p>The number of messages to be moved per second (the message movement rate). You can use this field to define a fixed message movement rate. The maximum value for messages per second is 500. If this field is left blank, the system will optimize the rate based on the queue message backlog size, which may vary throughout the duration of the message movement task.</p>
    pub fn max_number_of_messages_per_second(mut self, input: i32) -> Self {
        self.inner = self.inner.max_number_of_messages_per_second(input);
        self
    }
    /// <p>The number of messages to be moved per second (the message movement rate). You can use this field to define a fixed message movement rate. The maximum value for messages per second is 500. If this field is left blank, the system will optimize the rate based on the queue message backlog size, which may vary throughout the duration of the message movement task.</p>
    pub fn set_max_number_of_messages_per_second(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_number_of_messages_per_second(input);
        self
    }
    /// <p>The number of messages to be moved per second (the message movement rate). You can use this field to define a fixed message movement rate. The maximum value for messages per second is 500. If this field is left blank, the system will optimize the rate based on the queue message backlog size, which may vary throughout the duration of the message movement task.</p>
    pub fn get_max_number_of_messages_per_second(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_number_of_messages_per_second()
    }
}
