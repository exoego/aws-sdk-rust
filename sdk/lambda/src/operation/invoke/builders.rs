// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::invoke::_invoke_output::InvokeOutputBuilder;

pub use crate::operation::invoke::_invoke_input::InvokeInputBuilder;

impl InvokeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::invoke::InvokeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke::InvokeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.invoke();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Invoke`.
///
/// <p>Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. By default, Lambda invokes your function synchronously (i.e. the<code>InvocationType</code> is <code>RequestResponse</code>). To invoke a function asynchronously, set <code>InvocationType</code> to <code>Event</code>. Lambda passes the <code>ClientContext</code> object to your function for synchronous invocations only.</p>
/// <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-sync.html">synchronous invocation</a>, details about the function response, including errors, are included in the response body and headers. For either invocation type, you can find more information in the <a href="https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions.html">execution log</a> and <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-x-ray.html">trace</a>.</p>
/// <p>When an error occurs, your function may be invoked multiple times. Retry behavior varies by error type, client, event source, and invocation type. For example, if you invoke a function asynchronously and it returns an error, Lambda executes the function up to two more times. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-retries.html">Error handling and automatic retries in Lambda</a>.</p>
/// <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html">asynchronous invocation</a>, Lambda adds events to a queue before sending them to your function. If your function does not have enough capacity to keep up with the queue, events may be lost. Occasionally, your function may receive the same event multiple times, even if no error occurs. To retain events that were not processed, configure your function with a <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-dlq">dead-letter queue</a>.</p>
/// <p>The status code in the API response doesn't reflect function errors. Error codes are reserved for errors that prevent your function from executing, such as permissions errors, <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-limits.html">quota</a> errors, or issues with your function's code and configuration. For example, Lambda returns <code>TooManyRequestsException</code> if running the function would cause you to exceed a concurrency limit at either the account level (<code>ConcurrentInvocationLimitExceeded</code>) or function level (<code>ReservedFunctionConcurrentInvocationLimitExceeded</code>).</p>
/// <p>For functions with a long timeout, your client might disconnect during synchronous invocation while it waits for a response. Configure your HTTP client, SDK, firewall, proxy, or operating system to allow for long connections with timeout or keep-alive settings.</p>
/// <p>This operation requires permission for the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html">lambda:InvokeFunction</a> action. For details on how to set up permissions for cross-account invocations, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html#permissions-resource-xaccountinvoke">Granting function access to other accounts</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InvokeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::invoke::builders::InvokeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::invoke::InvokeOutput, crate::operation::invoke::InvokeError>
    for InvokeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::invoke::InvokeOutput, crate::operation::invoke::InvokeError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InvokeFluentBuilder {
    /// Creates a new `Invoke`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Invoke as a reference.
    pub fn as_input(&self) -> &crate::operation::invoke::builders::InvokeInputBuilder {
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
        crate::operation::invoke::InvokeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke::InvokeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::invoke::Invoke::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
        crate::operation::invoke::Invoke::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::invoke::InvokeOutput, crate::operation::invoke::InvokeError, Self> {
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
    /// <p>The name or ARN of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name or ARN of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name or ARN of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
    /// <p>Choose from the following options.</p>
    /// <ul>
    /// <li>
    /// <p><code>RequestResponse</code> (default) – Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API response includes the function response and additional data.</p></li>
    /// <li>
    /// <p><code>Event</code> – Invoke the function asynchronously. Send events that fail multiple times to the function's dead-letter queue (if one is configured). The API response only includes a status code.</p></li>
    /// <li>
    /// <p><code>DryRun</code> – Validate parameter values and verify that the user or role has permission to invoke the function.</p></li>
    /// </ul>
    pub fn invocation_type(mut self, input: crate::types::InvocationType) -> Self {
        self.inner = self.inner.invocation_type(input);
        self
    }
    /// <p>Choose from the following options.</p>
    /// <ul>
    /// <li>
    /// <p><code>RequestResponse</code> (default) – Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API response includes the function response and additional data.</p></li>
    /// <li>
    /// <p><code>Event</code> – Invoke the function asynchronously. Send events that fail multiple times to the function's dead-letter queue (if one is configured). The API response only includes a status code.</p></li>
    /// <li>
    /// <p><code>DryRun</code> – Validate parameter values and verify that the user or role has permission to invoke the function.</p></li>
    /// </ul>
    pub fn set_invocation_type(mut self, input: ::std::option::Option<crate::types::InvocationType>) -> Self {
        self.inner = self.inner.set_invocation_type(input);
        self
    }
    /// <p>Choose from the following options.</p>
    /// <ul>
    /// <li>
    /// <p><code>RequestResponse</code> (default) – Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API response includes the function response and additional data.</p></li>
    /// <li>
    /// <p><code>Event</code> – Invoke the function asynchronously. Send events that fail multiple times to the function's dead-letter queue (if one is configured). The API response only includes a status code.</p></li>
    /// <li>
    /// <p><code>DryRun</code> – Validate parameter values and verify that the user or role has permission to invoke the function.</p></li>
    /// </ul>
    pub fn get_invocation_type(&self) -> &::std::option::Option<crate::types::InvocationType> {
        self.inner.get_invocation_type()
    }
    /// <p>Set to <code>Tail</code> to include the execution log in the response. Applies to synchronously invoked functions only.</p>
    pub fn log_type(mut self, input: crate::types::LogType) -> Self {
        self.inner = self.inner.log_type(input);
        self
    }
    /// <p>Set to <code>Tail</code> to include the execution log in the response. Applies to synchronously invoked functions only.</p>
    pub fn set_log_type(mut self, input: ::std::option::Option<crate::types::LogType>) -> Self {
        self.inner = self.inner.set_log_type(input);
        self
    }
    /// <p>Set to <code>Tail</code> to include the execution log in the response. Applies to synchronously invoked functions only.</p>
    pub fn get_log_type(&self) -> &::std::option::Option<crate::types::LogType> {
        self.inner.get_log_type()
    }
    /// <p>Up to 3,583 bytes of base64-encoded data about the invoking client to pass to the function in the context object. Lambda passes the <code>ClientContext</code> object to your function for synchronous invocations only.</p>
    pub fn client_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_context(input.into());
        self
    }
    /// <p>Up to 3,583 bytes of base64-encoded data about the invoking client to pass to the function in the context object. Lambda passes the <code>ClientContext</code> object to your function for synchronous invocations only.</p>
    pub fn set_client_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_context(input);
        self
    }
    /// <p>Up to 3,583 bytes of base64-encoded data about the invoking client to pass to the function in the context object. Lambda passes the <code>ClientContext</code> object to your function for synchronous invocations only.</p>
    pub fn get_client_context(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_context()
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.payload(input);
        self
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_payload(input);
        self
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    /// <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p>
    pub fn get_payload(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_payload()
    }
    /// <p>Specify a version or alias to invoke a published version of the function.</p>
    pub fn qualifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.qualifier(input.into());
        self
    }
    /// <p>Specify a version or alias to invoke a published version of the function.</p>
    pub fn set_qualifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_qualifier(input);
        self
    }
    /// <p>Specify a version or alias to invoke a published version of the function.</p>
    pub fn get_qualifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_qualifier()
    }
}
