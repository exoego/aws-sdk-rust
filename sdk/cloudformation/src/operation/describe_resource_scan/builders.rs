// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_resource_scan::_describe_resource_scan_output::DescribeResourceScanOutputBuilder;

pub use crate::operation::describe_resource_scan::_describe_resource_scan_input::DescribeResourceScanInputBuilder;

impl DescribeResourceScanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_resource_scan::DescribeResourceScanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_resource_scan::DescribeResourceScanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_resource_scan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeResourceScan`.
///
/// <p>Describes details of a resource scan.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeResourceScanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_resource_scan::builders::DescribeResourceScanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_resource_scan::DescribeResourceScanOutput,
        crate::operation::describe_resource_scan::DescribeResourceScanError,
    > for DescribeResourceScanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_resource_scan::DescribeResourceScanOutput,
            crate::operation::describe_resource_scan::DescribeResourceScanError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeResourceScanFluentBuilder {
    /// Creates a new `DescribeResourceScan`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeResourceScan as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_resource_scan::builders::DescribeResourceScanInputBuilder {
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
        crate::operation::describe_resource_scan::DescribeResourceScanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_resource_scan::DescribeResourceScanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_resource_scan::DescribeResourceScan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_resource_scan::DescribeResourceScan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_resource_scan::DescribeResourceScanOutput,
        crate::operation::describe_resource_scan::DescribeResourceScanError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource scan.</p>
    pub fn resource_scan_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_scan_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource scan.</p>
    pub fn set_resource_scan_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_scan_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource scan.</p>
    pub fn get_resource_scan_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_scan_id()
    }
}
