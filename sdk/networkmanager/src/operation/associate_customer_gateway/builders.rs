// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_customer_gateway::_associate_customer_gateway_output::AssociateCustomerGatewayOutputBuilder;

pub use crate::operation::associate_customer_gateway::_associate_customer_gateway_input::AssociateCustomerGatewayInputBuilder;

impl AssociateCustomerGatewayInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_customer_gateway();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateCustomerGateway`.
///
/// <p>Associates a customer gateway with a device and optionally, with a link. If you specify a link, it must be associated with the specified device.</p>
/// <p>You can only associate customer gateways that are connected to a VPN attachment on a transit gateway or core network registered in your global network. When you register a transit gateway or core network, customer gateways that are connected to the transit gateway are automatically included in the global network. To list customer gateways that are connected to a transit gateway, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpnConnections.html">DescribeVpnConnections</a> EC2 API and filter by <code>transit-gateway-id</code>.</p>
/// <p>You cannot associate a customer gateway with more than one device and link.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateCustomerGatewayFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_customer_gateway::builders::AssociateCustomerGatewayInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
    > for AssociateCustomerGatewayFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateCustomerGatewayFluentBuilder {
    /// Creates a new `AssociateCustomerGateway`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateCustomerGateway as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_customer_gateway::builders::AssociateCustomerGatewayInputBuilder {
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
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_customer_gateway::AssociateCustomerGateway::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_customer_gateway::AssociateCustomerGateway::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
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
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn customer_gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.customer_gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn set_customer_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_customer_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn get_customer_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_customer_gateway_arn()
    }
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_network_id()
    }
    /// <p>The ID of the device.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The ID of the device.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
    /// <p>The ID of the device.</p>
    pub fn get_device_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_id()
    }
    /// <p>The ID of the link.</p>
    pub fn link_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.link_id(input.into());
        self
    }
    /// <p>The ID of the link.</p>
    pub fn set_link_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_link_id(input);
        self
    }
    /// <p>The ID of the link.</p>
    pub fn get_link_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_link_id()
    }
}
