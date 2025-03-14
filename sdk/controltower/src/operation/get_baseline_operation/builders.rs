// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_baseline_operation::_get_baseline_operation_output::GetBaselineOperationOutputBuilder;

pub use crate::operation::get_baseline_operation::_get_baseline_operation_input::GetBaselineOperationInputBuilder;

impl GetBaselineOperationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_baseline_operation::GetBaselineOperationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_baseline_operation::GetBaselineOperationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_baseline_operation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBaselineOperation`.
///
/// <p>Returns the details of an asynchronous baseline operation, as initiated by any of these APIs: <code>EnableBaseline</code>, <code>DisableBaseline</code>, <code>UpdateEnabledBaseline</code>, <code>ResetEnabledBaseline</code>. A status message is displayed in case of operation failure.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBaselineOperationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_baseline_operation::builders::GetBaselineOperationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_baseline_operation::GetBaselineOperationOutput,
        crate::operation::get_baseline_operation::GetBaselineOperationError,
    > for GetBaselineOperationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_baseline_operation::GetBaselineOperationOutput,
            crate::operation::get_baseline_operation::GetBaselineOperationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetBaselineOperationFluentBuilder {
    /// Creates a new `GetBaselineOperation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBaselineOperation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_baseline_operation::builders::GetBaselineOperationInputBuilder {
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
        crate::operation::get_baseline_operation::GetBaselineOperationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_baseline_operation::GetBaselineOperationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_baseline_operation::GetBaselineOperation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_baseline_operation::GetBaselineOperation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_baseline_operation::GetBaselineOperationOutput,
        crate::operation::get_baseline_operation::GetBaselineOperationError,
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
    /// <p>The operation ID returned from mutating asynchronous APIs (Enable, Disable, Update, Reset).</p>
    pub fn operation_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operation_identifier(input.into());
        self
    }
    /// <p>The operation ID returned from mutating asynchronous APIs (Enable, Disable, Update, Reset).</p>
    pub fn set_operation_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_operation_identifier(input);
        self
    }
    /// <p>The operation ID returned from mutating asynchronous APIs (Enable, Disable, Update, Reset).</p>
    pub fn get_operation_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_operation_identifier()
    }
}
