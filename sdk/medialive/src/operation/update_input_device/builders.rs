// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_input_device::_update_input_device_output::UpdateInputDeviceOutputBuilder;

pub use crate::operation::update_input_device::_update_input_device_input::UpdateInputDeviceInputBuilder;

impl UpdateInputDeviceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_input_device::UpdateInputDeviceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_input_device::UpdateInputDeviceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_input_device();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateInputDevice`.
///
/// Updates the parameters for the input device.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateInputDeviceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_input_device::builders::UpdateInputDeviceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_input_device::UpdateInputDeviceOutput,
        crate::operation::update_input_device::UpdateInputDeviceError,
    > for UpdateInputDeviceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_input_device::UpdateInputDeviceOutput,
            crate::operation::update_input_device::UpdateInputDeviceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateInputDeviceFluentBuilder {
    /// Creates a new `UpdateInputDevice`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateInputDevice as a reference.
    pub fn as_input(&self) -> &crate::operation::update_input_device::builders::UpdateInputDeviceInputBuilder {
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
        crate::operation::update_input_device::UpdateInputDeviceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_input_device::UpdateInputDeviceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_input_device::UpdateInputDevice::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_input_device::UpdateInputDevice::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_input_device::UpdateInputDeviceOutput,
        crate::operation::update_input_device::UpdateInputDeviceError,
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
    /// The settings that you want to apply to the HD input device.
    pub fn hd_device_settings(mut self, input: crate::types::InputDeviceConfigurableSettings) -> Self {
        self.inner = self.inner.hd_device_settings(input);
        self
    }
    /// The settings that you want to apply to the HD input device.
    pub fn set_hd_device_settings(mut self, input: ::std::option::Option<crate::types::InputDeviceConfigurableSettings>) -> Self {
        self.inner = self.inner.set_hd_device_settings(input);
        self
    }
    /// The settings that you want to apply to the HD input device.
    pub fn get_hd_device_settings(&self) -> &::std::option::Option<crate::types::InputDeviceConfigurableSettings> {
        self.inner.get_hd_device_settings()
    }
    /// The unique ID of the input device. For example, hd-123456789abcdef.
    pub fn input_device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_device_id(input.into());
        self
    }
    /// The unique ID of the input device. For example, hd-123456789abcdef.
    pub fn set_input_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_device_id(input);
        self
    }
    /// The unique ID of the input device. For example, hd-123456789abcdef.
    pub fn get_input_device_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_device_id()
    }
    /// The name that you assigned to this input device (not the unique ID).
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// The name that you assigned to this input device (not the unique ID).
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// The name that you assigned to this input device (not the unique ID).
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// The settings that you want to apply to the UHD input device.
    pub fn uhd_device_settings(mut self, input: crate::types::InputDeviceConfigurableSettings) -> Self {
        self.inner = self.inner.uhd_device_settings(input);
        self
    }
    /// The settings that you want to apply to the UHD input device.
    pub fn set_uhd_device_settings(mut self, input: ::std::option::Option<crate::types::InputDeviceConfigurableSettings>) -> Self {
        self.inner = self.inner.set_uhd_device_settings(input);
        self
    }
    /// The settings that you want to apply to the UHD input device.
    pub fn get_uhd_device_settings(&self) -> &::std::option::Option<crate::types::InputDeviceConfigurableSettings> {
        self.inner.get_uhd_device_settings()
    }
    /// The Availability Zone you want associated with this input device.
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// The Availability Zone you want associated with this input device.
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// The Availability Zone you want associated with this input device.
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
}
