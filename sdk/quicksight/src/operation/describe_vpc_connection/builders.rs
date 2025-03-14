// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_vpc_connection::_describe_vpc_connection_output::DescribeVpcConnectionOutputBuilder;

pub use crate::operation::describe_vpc_connection::_describe_vpc_connection_input::DescribeVpcConnectionInputBuilder;

impl DescribeVpcConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_connection::DescribeVpcConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_connection::DescribeVPCConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_vpc_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeVPCConnection`.
///
/// <p>Describes a VPC connection.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeVPCConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_vpc_connection::DescribeVpcConnectionOutput,
        crate::operation::describe_vpc_connection::DescribeVPCConnectionError,
    > for DescribeVPCConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_vpc_connection::DescribeVpcConnectionOutput,
            crate::operation::describe_vpc_connection::DescribeVPCConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeVPCConnectionFluentBuilder {
    /// Creates a new `DescribeVPCConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeVPCConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder {
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
        crate::operation::describe_vpc_connection::DescribeVpcConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_connection::DescribeVPCConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_vpc_connection::DescribeVPCConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_vpc_connection::DescribeVPCConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_vpc_connection::DescribeVpcConnectionOutput,
        crate::operation::describe_vpc_connection::DescribeVPCConnectionError,
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
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn vpc_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_connection_id(input.into());
        self
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn set_vpc_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_connection_id(input);
        self
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn get_vpc_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_connection_id()
    }
}
