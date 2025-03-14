// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_test_grid_url::_create_test_grid_url_output::CreateTestGridUrlOutputBuilder;

pub use crate::operation::create_test_grid_url::_create_test_grid_url_input::CreateTestGridUrlInputBuilder;

impl CreateTestGridUrlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_test_grid_url::CreateTestGridUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_test_grid_url::CreateTestGridUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_test_grid_url();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateTestGridUrl`.
///
/// <p>Creates a signed, short-term URL that can be passed to a Selenium <code>RemoteWebDriver</code> constructor.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTestGridUrlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_test_grid_url::builders::CreateTestGridUrlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_test_grid_url::CreateTestGridUrlOutput,
        crate::operation::create_test_grid_url::CreateTestGridUrlError,
    > for CreateTestGridUrlFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_test_grid_url::CreateTestGridUrlOutput,
            crate::operation::create_test_grid_url::CreateTestGridUrlError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateTestGridUrlFluentBuilder {
    /// Creates a new `CreateTestGridUrl`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateTestGridUrl as a reference.
    pub fn as_input(&self) -> &crate::operation::create_test_grid_url::builders::CreateTestGridUrlInputBuilder {
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
        crate::operation::create_test_grid_url::CreateTestGridUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_test_grid_url::CreateTestGridUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_test_grid_url::CreateTestGridUrl::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_test_grid_url::CreateTestGridUrl::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_test_grid_url::CreateTestGridUrlOutput,
        crate::operation::create_test_grid_url::CreateTestGridUrlError,
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
    /// <p>ARN (from <code>CreateTestGridProject</code> or <code>ListTestGridProjects</code>) to associate with the short-term URL.</p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_arn(input.into());
        self
    }
    /// <p>ARN (from <code>CreateTestGridProject</code> or <code>ListTestGridProjects</code>) to associate with the short-term URL.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_arn(input);
        self
    }
    /// <p>ARN (from <code>CreateTestGridProject</code> or <code>ListTestGridProjects</code>) to associate with the short-term URL.</p>
    pub fn get_project_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_arn()
    }
    /// <p>Lifetime, in seconds, of the URL.</p>
    pub fn expires_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.expires_in_seconds(input);
        self
    }
    /// <p>Lifetime, in seconds, of the URL.</p>
    pub fn set_expires_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_expires_in_seconds(input);
        self
    }
    /// <p>Lifetime, in seconds, of the URL.</p>
    pub fn get_expires_in_seconds(&self) -> &::std::option::Option<i32> {
        self.inner.get_expires_in_seconds()
    }
}
