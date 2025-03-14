// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_service_instance_deployment::_cancel_service_instance_deployment_output::CancelServiceInstanceDeploymentOutputBuilder;

pub use crate::operation::cancel_service_instance_deployment::_cancel_service_instance_deployment_input::CancelServiceInstanceDeploymentInputBuilder;

impl CancelServiceInstanceDeploymentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_service_instance_deployment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelServiceInstanceDeployment`.
///
/// <p>Attempts to cancel a service instance deployment on an <code>UpdateServiceInstance</code> action, if the deployment is <code>IN_PROGRESS</code>. For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-svc-instance-update.html">Update a service instance</a> in the <i>Proton User guide</i>.</p>
/// <p>The following list includes potential cancellation scenarios.</p>
/// <ul>
/// <li>
/// <p>If the cancellation attempt succeeds, the resulting deployment state is <code>CANCELLED</code>.</p></li>
/// <li>
/// <p>If the cancellation attempt fails, the resulting deployment state is <code>FAILED</code>.</p></li>
/// <li>
/// <p>If the current <code>UpdateServiceInstance</code> action succeeds before the cancellation attempt starts, the resulting deployment state is <code>SUCCEEDED</code> and the cancellation attempt has no effect.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelServiceInstanceDeploymentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput,
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError,
    > for CancelServiceInstanceDeploymentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput,
            crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelServiceInstanceDeploymentFluentBuilder {
    /// Creates a new `CancelServiceInstanceDeployment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelServiceInstanceDeployment as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentInputBuilder {
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
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeployment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeployment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput,
        crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError,
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
    /// <p>The name of the service instance with the deployment to cancel.</p>
    pub fn service_instance_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_instance_name(input.into());
        self
    }
    /// <p>The name of the service instance with the deployment to cancel.</p>
    pub fn set_service_instance_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_instance_name(input);
        self
    }
    /// <p>The name of the service instance with the deployment to cancel.</p>
    pub fn get_service_instance_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_instance_name()
    }
    /// <p>The name of the service with the service instance deployment to cancel.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the service with the service instance deployment to cancel.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The name of the service with the service instance deployment to cancel.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_name()
    }
}
