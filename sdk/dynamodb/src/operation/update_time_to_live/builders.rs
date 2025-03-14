// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_time_to_live::_update_time_to_live_output::UpdateTimeToLiveOutputBuilder;

pub use crate::operation::update_time_to_live::_update_time_to_live_input::UpdateTimeToLiveInputBuilder;

impl UpdateTimeToLiveInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_time_to_live::UpdateTimeToLiveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_time_to_live();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateTimeToLive`.
///
/// <p>The <code>UpdateTimeToLive</code> method enables or disables Time to Live (TTL) for the specified table. A successful <code>UpdateTimeToLive</code> call returns the current <code>TimeToLiveSpecification</code>. It can take up to one hour for the change to fully process. Any additional <code>UpdateTimeToLive</code> calls for the same table during this one hour duration result in a <code>ValidationException</code>.</p>
/// <p>TTL compares the current time in epoch time format to the time stored in the TTL attribute of an item. If the epoch time value stored in the attribute is less than the current time, the item is marked as expired and subsequently deleted.</p><note>
/// <p>The epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.</p>
/// </note>
/// <p>DynamoDB deletes expired items on a best-effort basis to ensure availability of throughput for other data operations.</p><important>
/// <p>DynamoDB typically deletes expired items within two days of expiration. The exact duration within which an item gets deleted after expiration is specific to the nature of the workload. Items that have expired and not been deleted will still show up in reads, queries, and scans.</p>
/// </important>
/// <p>As items are deleted, they are removed from any local secondary index and global secondary index immediately in the same eventually consistent way as a standard delete operation.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/TTL.html">Time To Live</a> in the Amazon DynamoDB Developer Guide.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTimeToLiveFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_time_to_live::builders::UpdateTimeToLiveInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
        crate::operation::update_time_to_live::UpdateTimeToLiveError,
    > for UpdateTimeToLiveFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
            crate::operation::update_time_to_live::UpdateTimeToLiveError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateTimeToLiveFluentBuilder {
    /// Creates a new `UpdateTimeToLive`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateTimeToLive as a reference.
    pub fn as_input(&self) -> &crate::operation::update_time_to_live::builders::UpdateTimeToLiveInputBuilder {
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
        crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_time_to_live::UpdateTimeToLiveError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_time_to_live::UpdateTimeToLive::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_time_to_live::UpdateTimeToLive::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
        crate::operation::update_time_to_live::UpdateTimeToLiveError,
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
    /// <p>The name of the table to be configured. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to be configured. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table to be configured. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// <p>Represents the settings used to enable or disable Time to Live for the specified table.</p>
    pub fn time_to_live_specification(mut self, input: crate::types::TimeToLiveSpecification) -> Self {
        self.inner = self.inner.time_to_live_specification(input);
        self
    }
    /// <p>Represents the settings used to enable or disable Time to Live for the specified table.</p>
    pub fn set_time_to_live_specification(mut self, input: ::std::option::Option<crate::types::TimeToLiveSpecification>) -> Self {
        self.inner = self.inner.set_time_to_live_specification(input);
        self
    }
    /// <p>Represents the settings used to enable or disable Time to Live for the specified table.</p>
    pub fn get_time_to_live_specification(&self) -> &::std::option::Option<crate::types::TimeToLiveSpecification> {
        self.inner.get_time_to_live_specification()
    }
}
