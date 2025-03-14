// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_experiment::_start_experiment_output::StartExperimentOutputBuilder;

pub use crate::operation::start_experiment::_start_experiment_input::StartExperimentInputBuilder;

impl StartExperimentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_experiment::StartExperimentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_experiment::StartExperimentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_experiment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartExperiment`.
///
/// <p>Starts running an experiment from the specified experiment template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartExperimentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_experiment::builders::StartExperimentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_experiment::StartExperimentOutput,
        crate::operation::start_experiment::StartExperimentError,
    > for StartExperimentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_experiment::StartExperimentOutput,
            crate::operation::start_experiment::StartExperimentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartExperimentFluentBuilder {
    /// Creates a new `StartExperiment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartExperiment as a reference.
    pub fn as_input(&self) -> &crate::operation::start_experiment::builders::StartExperimentInputBuilder {
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
        crate::operation::start_experiment::StartExperimentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_experiment::StartExperimentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_experiment::StartExperiment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_experiment::StartExperiment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_experiment::StartExperimentOutput,
        crate::operation::start_experiment::StartExperimentError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The ID of the experiment template.</p>
    pub fn experiment_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.experiment_template_id(input.into());
        self
    }
    /// <p>The ID of the experiment template.</p>
    pub fn set_experiment_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_experiment_template_id(input);
        self
    }
    /// <p>The ID of the experiment template.</p>
    pub fn get_experiment_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_experiment_template_id()
    }
    /// <p>The experiment options for running the experiment.</p>
    pub fn experiment_options(mut self, input: crate::types::StartExperimentExperimentOptionsInput) -> Self {
        self.inner = self.inner.experiment_options(input);
        self
    }
    /// <p>The experiment options for running the experiment.</p>
    pub fn set_experiment_options(mut self, input: ::std::option::Option<crate::types::StartExperimentExperimentOptionsInput>) -> Self {
        self.inner = self.inner.set_experiment_options(input);
        self
    }
    /// <p>The experiment options for running the experiment.</p>
    pub fn get_experiment_options(&self) -> &::std::option::Option<crate::types::StartExperimentExperimentOptionsInput> {
        self.inner.get_experiment_options()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the experiment.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to apply to the experiment.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to apply to the experiment.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
