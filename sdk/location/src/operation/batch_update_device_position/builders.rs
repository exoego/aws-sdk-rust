// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_device_position::_batch_update_device_position_output::BatchUpdateDevicePositionOutputBuilder;

pub use crate::operation::batch_update_device_position::_batch_update_device_position_input::BatchUpdateDevicePositionInputBuilder;

impl BatchUpdateDevicePositionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_device_position::BatchUpdateDevicePositionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_update_device_position();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchUpdateDevicePosition`.
///
/// <p>Uploads position update data for one or more devices to a tracker resource (up to 10 devices per batch). Amazon Location uses the data when it reports the last known device position and position history. Amazon Location retains location data for 30 days.</p><note>
/// <p>Position updates are handled based on the <code>PositionFiltering</code> property of the tracker. When <code>PositionFiltering</code> is set to <code>TimeBased</code>, updates are evaluated against linked geofence collections, and location data is stored at a maximum of one position per 30 second interval. If your update frequency is more often than every 30 seconds, only one update per 30 seconds is stored for each unique device ID.</p>
/// <p>When <code>PositionFiltering</code> is set to <code>DistanceBased</code> filtering, location data is stored and evaluated against linked geofence collections only if the device has moved more than 30 m (98.4 ft).</p>
/// <p>When <code>PositionFiltering</code> is set to <code>AccuracyBased</code> filtering, location data is stored and evaluated against linked geofence collections only if the device has moved more than the measured accuracy. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is neither stored or evaluated if the device has moved less than 15 m. If <code>PositionFiltering</code> is set to <code>AccuracyBased</code> filtering, Amazon Location uses the default value <code>{ "Horizontal": 0}</code> when accuracy is not provided on a <code>DevicePositionUpdate</code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchUpdateDevicePositionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_update_device_position::builders::BatchUpdateDevicePositionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionOutput,
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionError,
    > for BatchUpdateDevicePositionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_update_device_position::BatchUpdateDevicePositionOutput,
            crate::operation::batch_update_device_position::BatchUpdateDevicePositionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchUpdateDevicePositionFluentBuilder {
    /// Creates a new `BatchUpdateDevicePosition`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchUpdateDevicePosition as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_update_device_position::builders::BatchUpdateDevicePositionInputBuilder {
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
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_device_position::BatchUpdateDevicePositionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_update_device_position::BatchUpdateDevicePosition::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_update_device_position::BatchUpdateDevicePosition::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionOutput,
        crate::operation::batch_update_device_position::BatchUpdateDevicePositionError,
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
    /// <p>The name of the tracker resource to update.</p>
    pub fn tracker_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tracker_name(input.into());
        self
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn set_tracker_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tracker_name(input);
        self
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn get_tracker_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_tracker_name()
    }
    /// Appends an item to `Updates`.
    ///
    /// To override the contents of this collection use [`set_updates`](Self::set_updates).
    ///
    /// <p>Contains the position update details for each device, up to 10 devices.</p>
    pub fn updates(mut self, input: crate::types::DevicePositionUpdate) -> Self {
        self.inner = self.inner.updates(input);
        self
    }
    /// <p>Contains the position update details for each device, up to 10 devices.</p>
    pub fn set_updates(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DevicePositionUpdate>>) -> Self {
        self.inner = self.inner.set_updates(input);
        self
    }
    /// <p>Contains the position update details for each device, up to 10 devices.</p>
    pub fn get_updates(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DevicePositionUpdate>> {
        self.inner.get_updates()
    }
}
