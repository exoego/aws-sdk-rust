// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_membership_datasources::_batch_get_membership_datasources_output::BatchGetMembershipDatasourcesOutputBuilder;

pub use crate::operation::batch_get_membership_datasources::_batch_get_membership_datasources_input::BatchGetMembershipDatasourcesInputBuilder;

impl BatchGetMembershipDatasourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_get_membership_datasources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchGetMembershipDatasources`.
///
/// <p>Gets information on the data source package history for an account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGetMembershipDatasourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_get_membership_datasources::builders::BatchGetMembershipDatasourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
    > for BatchGetMembershipDatasourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchGetMembershipDatasourcesFluentBuilder {
    /// Creates a new `BatchGetMembershipDatasources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchGetMembershipDatasources as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_get_membership_datasources::builders::BatchGetMembershipDatasourcesInputBuilder {
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
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesOutput,
        crate::operation::batch_get_membership_datasources::BatchGetMembershipDatasourcesError,
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
    /// Appends an item to `GraphArns`.
    ///
    /// To override the contents of this collection use [`set_graph_arns`](Self::set_graph_arns).
    ///
    /// <p>The ARN of the behavior graph.</p>
    pub fn graph_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_arns(input.into());
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn set_graph_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_graph_arns(input);
        self
    }
    /// <p>The ARN of the behavior graph.</p>
    pub fn get_graph_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_graph_arns()
    }
}
