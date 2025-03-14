// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_access_entry::_describe_access_entry_output::DescribeAccessEntryOutputBuilder;

pub use crate::operation::describe_access_entry::_describe_access_entry_input::DescribeAccessEntryInputBuilder;

impl DescribeAccessEntryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_access_entry::DescribeAccessEntryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access_entry::DescribeAccessEntryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_access_entry();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAccessEntry`.
///
/// <p>Describes an access entry.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAccessEntryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_access_entry::builders::DescribeAccessEntryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_access_entry::DescribeAccessEntryOutput,
        crate::operation::describe_access_entry::DescribeAccessEntryError,
    > for DescribeAccessEntryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_access_entry::DescribeAccessEntryOutput,
            crate::operation::describe_access_entry::DescribeAccessEntryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAccessEntryFluentBuilder {
    /// Creates a new `DescribeAccessEntry`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAccessEntry as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_access_entry::builders::DescribeAccessEntryInputBuilder {
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
        crate::operation::describe_access_entry::DescribeAccessEntryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access_entry::DescribeAccessEntryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_access_entry::DescribeAccessEntry::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_access_entry::DescribeAccessEntry::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_access_entry::DescribeAccessEntryOutput,
        crate::operation::describe_access_entry::DescribeAccessEntryError,
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
    /// <p>The name of your cluster.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_name(input.into());
        self
    }
    /// <p>The name of your cluster.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_name(input);
        self
    }
    /// <p>The name of your cluster.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_name()
    }
    /// <p>The ARN of the IAM principal for the <code>AccessEntry</code>.</p>
    pub fn principal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM principal for the <code>AccessEntry</code>.</p>
    pub fn set_principal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
    /// <p>The ARN of the IAM principal for the <code>AccessEntry</code>.</p>
    pub fn get_principal_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_arn()
    }
}
