// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_retraining_scheduler::_create_retraining_scheduler_output::CreateRetrainingSchedulerOutputBuilder;

pub use crate::operation::create_retraining_scheduler::_create_retraining_scheduler_input::CreateRetrainingSchedulerInputBuilder;

impl CreateRetrainingSchedulerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_retraining_scheduler();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRetrainingScheduler`.
///
/// <p>Creates a retraining scheduler on the specified model.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRetrainingSchedulerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_retraining_scheduler::builders::CreateRetrainingSchedulerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerOutput,
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerError,
    > for CreateRetrainingSchedulerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerOutput,
            crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRetrainingSchedulerFluentBuilder {
    /// Creates a new `CreateRetrainingScheduler`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRetrainingScheduler as a reference.
    pub fn as_input(&self) -> &crate::operation::create_retraining_scheduler::builders::CreateRetrainingSchedulerInputBuilder {
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
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_retraining_scheduler::CreateRetrainingScheduler::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_retraining_scheduler::CreateRetrainingScheduler::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerOutput,
        crate::operation::create_retraining_scheduler::CreateRetrainingSchedulerError,
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
    /// <p>The name of the model to add the retraining scheduler to.</p>
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_name(input.into());
        self
    }
    /// <p>The name of the model to add the retraining scheduler to.</p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_name(input);
        self
    }
    /// <p>The name of the model to add the retraining scheduler to.</p>
    pub fn get_model_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_name()
    }
    /// <p>The start date for the retraining scheduler. Lookout for Equipment truncates the time you provide to the nearest UTC day.</p>
    pub fn retraining_start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.retraining_start_date(input);
        self
    }
    /// <p>The start date for the retraining scheduler. Lookout for Equipment truncates the time you provide to the nearest UTC day.</p>
    pub fn set_retraining_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_retraining_start_date(input);
        self
    }
    /// <p>The start date for the retraining scheduler. Lookout for Equipment truncates the time you provide to the nearest UTC day.</p>
    pub fn get_retraining_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_retraining_start_date()
    }
    /// <p>This parameter uses the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a> standard to set the frequency at which you want retraining to occur in terms of Years, Months, and/or Days (note: other parameters like Time are not currently supported). The minimum value is 30 days (P30D) and the maximum value is 1 year (P1Y). For example, the following values are valid:</p>
    /// <ul>
    /// <li>
    /// <p>P3M15D – Every 3 months and 15 days</p></li>
    /// <li>
    /// <p>P2M – Every 2 months</p></li>
    /// <li>
    /// <p>P150D – Every 150 days</p></li>
    /// </ul>
    pub fn retraining_frequency(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.retraining_frequency(input.into());
        self
    }
    /// <p>This parameter uses the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a> standard to set the frequency at which you want retraining to occur in terms of Years, Months, and/or Days (note: other parameters like Time are not currently supported). The minimum value is 30 days (P30D) and the maximum value is 1 year (P1Y). For example, the following values are valid:</p>
    /// <ul>
    /// <li>
    /// <p>P3M15D – Every 3 months and 15 days</p></li>
    /// <li>
    /// <p>P2M – Every 2 months</p></li>
    /// <li>
    /// <p>P150D – Every 150 days</p></li>
    /// </ul>
    pub fn set_retraining_frequency(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_retraining_frequency(input);
        self
    }
    /// <p>This parameter uses the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a> standard to set the frequency at which you want retraining to occur in terms of Years, Months, and/or Days (note: other parameters like Time are not currently supported). The minimum value is 30 days (P30D) and the maximum value is 1 year (P1Y). For example, the following values are valid:</p>
    /// <ul>
    /// <li>
    /// <p>P3M15D – Every 3 months and 15 days</p></li>
    /// <li>
    /// <p>P2M – Every 2 months</p></li>
    /// <li>
    /// <p>P150D – Every 150 days</p></li>
    /// </ul>
    pub fn get_retraining_frequency(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_retraining_frequency()
    }
    /// <p>The number of past days of data that will be used for retraining.</p>
    pub fn lookback_window(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lookback_window(input.into());
        self
    }
    /// <p>The number of past days of data that will be used for retraining.</p>
    pub fn set_lookback_window(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lookback_window(input);
        self
    }
    /// <p>The number of past days of data that will be used for retraining.</p>
    pub fn get_lookback_window(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lookback_window()
    }
    /// <p>Indicates how the service will use new models. In <code>MANAGED</code> mode, new models will automatically be used for inference if they have better performance than the current model. In <code>MANUAL</code> mode, the new models will not be used <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/versioning-model.html#model-activation">until they are manually activated</a>.</p>
    pub fn promote_mode(mut self, input: crate::types::ModelPromoteMode) -> Self {
        self.inner = self.inner.promote_mode(input);
        self
    }
    /// <p>Indicates how the service will use new models. In <code>MANAGED</code> mode, new models will automatically be used for inference if they have better performance than the current model. In <code>MANUAL</code> mode, the new models will not be used <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/versioning-model.html#model-activation">until they are manually activated</a>.</p>
    pub fn set_promote_mode(mut self, input: ::std::option::Option<crate::types::ModelPromoteMode>) -> Self {
        self.inner = self.inner.set_promote_mode(input);
        self
    }
    /// <p>Indicates how the service will use new models. In <code>MANAGED</code> mode, new models will automatically be used for inference if they have better performance than the current model. In <code>MANUAL</code> mode, the new models will not be used <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/versioning-model.html#model-activation">until they are manually activated</a>.</p>
    pub fn get_promote_mode(&self) -> &::std::option::Option<crate::types::ModelPromoteMode> {
        self.inner.get_promote_mode()
    }
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
