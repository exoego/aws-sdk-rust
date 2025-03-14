// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::initiate_device_claim::_initiate_device_claim_output::InitiateDeviceClaimOutputBuilder;

pub use crate::operation::initiate_device_claim::_initiate_device_claim_input::InitiateDeviceClaimInputBuilder;

impl InitiateDeviceClaimInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::initiate_device_claim::InitiateDeviceClaimOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_device_claim::InitiateDeviceClaimError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.initiate_device_claim();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `InitiateDeviceClaim`.
///
/// <p>Given a device ID, initiates a claim request for the associated device.</p><note>
/// <p>Claiming a device consists of initiating a claim, then publishing a device event, and finalizing the claim. For a device of type button, a device event can be published by simply clicking the device.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InitiateDeviceClaimFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::initiate_device_claim::builders::InitiateDeviceClaimInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::initiate_device_claim::InitiateDeviceClaimOutput,
        crate::operation::initiate_device_claim::InitiateDeviceClaimError,
    > for InitiateDeviceClaimFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::initiate_device_claim::InitiateDeviceClaimOutput,
            crate::operation::initiate_device_claim::InitiateDeviceClaimError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InitiateDeviceClaimFluentBuilder {
    /// Creates a new `InitiateDeviceClaim`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the InitiateDeviceClaim as a reference.
    pub fn as_input(&self) -> &crate::operation::initiate_device_claim::builders::InitiateDeviceClaimInputBuilder {
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
        crate::operation::initiate_device_claim::InitiateDeviceClaimOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_device_claim::InitiateDeviceClaimError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::initiate_device_claim::InitiateDeviceClaim::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::initiate_device_claim::InitiateDeviceClaim::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::initiate_device_claim::InitiateDeviceClaimOutput,
        crate::operation::initiate_device_claim::InitiateDeviceClaimError,
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
    /// <p>The unique identifier of the device.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The unique identifier of the device.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
    /// <p>The unique identifier of the device.</p>
    pub fn get_device_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_id()
    }
}
