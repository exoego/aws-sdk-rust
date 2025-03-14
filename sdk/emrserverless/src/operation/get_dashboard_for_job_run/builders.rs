// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_dashboard_for_job_run::_get_dashboard_for_job_run_output::GetDashboardForJobRunOutputBuilder;

pub use crate::operation::get_dashboard_for_job_run::_get_dashboard_for_job_run_input::GetDashboardForJobRunInputBuilder;

impl GetDashboardForJobRunInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_dashboard_for_job_run();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDashboardForJobRun`.
///
/// <p>Creates and returns a URL that you can use to access the application UIs for a job run.</p>
/// <p>For jobs in a running state, the application UI is a live user interface such as the Spark or Tez web UI. For completed jobs, the application UI is a persistent application user interface such as the Spark History Server or persistent Tez UI.</p><note>
/// <p>The URL is valid for one hour after you generate it. To access the application UI after that hour elapses, you must invoke the API again to generate a new URL.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDashboardForJobRunFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_dashboard_for_job_run::builders::GetDashboardForJobRunInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunOutput,
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunError,
    > for GetDashboardForJobRunFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunOutput,
            crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetDashboardForJobRunFluentBuilder {
    /// Creates a new `GetDashboardForJobRun`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDashboardForJobRun as a reference.
    pub fn as_input(&self) -> &crate::operation::get_dashboard_for_job_run::builders::GetDashboardForJobRunInputBuilder {
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
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_dashboard_for_job_run::GetDashboardForJobRun::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRun::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunOutput,
        crate::operation::get_dashboard_for_job_run::GetDashboardForJobRunError,
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
    /// <p>The ID of the application.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The ID of the application.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The ID of the job run.</p>
    pub fn job_run_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_run_id(input.into());
        self
    }
    /// <p>The ID of the job run.</p>
    pub fn set_job_run_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_run_id(input);
        self
    }
    /// <p>The ID of the job run.</p>
    pub fn get_job_run_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_run_id()
    }
}
