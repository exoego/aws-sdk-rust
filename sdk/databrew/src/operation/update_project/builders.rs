// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_project::_update_project_output::UpdateProjectOutputBuilder;

pub use crate::operation::update_project::_update_project_input::UpdateProjectInputBuilder;

impl UpdateProjectInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_project();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateProject`.
///
/// <p>Modifies the definition of an existing DataBrew project.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProjectFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_project::builders::UpdateProjectInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_project::UpdateProjectOutput,
        crate::operation::update_project::UpdateProjectError,
    > for UpdateProjectFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_project::UpdateProjectOutput,
            crate::operation::update_project::UpdateProjectError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateProjectFluentBuilder {
    /// Creates a new `UpdateProject`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateProject as a reference.
    pub fn as_input(&self) -> &crate::operation::update_project::builders::UpdateProjectInputBuilder {
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
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_project::UpdateProject::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_project::UpdateProject::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_project::UpdateProjectOutput,
        crate::operation::update_project::UpdateProjectError,
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
    /// <p>Represents the sample size and sampling type for DataBrew to use for interactive data analysis.</p>
    pub fn sample(mut self, input: crate::types::Sample) -> Self {
        self.inner = self.inner.sample(input);
        self
    }
    /// <p>Represents the sample size and sampling type for DataBrew to use for interactive data analysis.</p>
    pub fn set_sample(mut self, input: ::std::option::Option<crate::types::Sample>) -> Self {
        self.inner = self.inner.set_sample(input);
        self
    }
    /// <p>Represents the sample size and sampling type for DataBrew to use for interactive data analysis.</p>
    pub fn get_sample(&self) -> &::std::option::Option<crate::types::Sample> {
        self.inner.get_sample()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed for this request.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed for this request.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed for this request.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>The name of the project to be updated.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the project to be updated.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the project to be updated.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
