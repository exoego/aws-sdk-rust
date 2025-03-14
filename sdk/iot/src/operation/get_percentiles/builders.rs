// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_percentiles::_get_percentiles_output::GetPercentilesOutputBuilder;

pub use crate::operation::get_percentiles::_get_percentiles_input::GetPercentilesInputBuilder;

impl GetPercentilesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_percentiles::GetPercentilesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_percentiles::GetPercentilesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_percentiles();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPercentiles`.
///
/// <p>Groups the aggregated values that match the query into percentile groupings. The default percentile groupings are: 1,5,25,50,75,95,99, although you can specify your own when you call <code>GetPercentiles</code>. This function returns a value for each percentile group specified (or the default percentile groupings). The percentile group "1" contains the aggregated field value that occurs in approximately one percent of the values that match the query. The percentile group "5" contains the aggregated field value that occurs in approximately five percent of the values that match the query, and so on. The result is an approximation, the more values that match the query, the more accurate the percentile values.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">GetPercentiles</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPercentilesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_percentiles::builders::GetPercentilesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_percentiles::GetPercentilesOutput,
        crate::operation::get_percentiles::GetPercentilesError,
    > for GetPercentilesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_percentiles::GetPercentilesOutput,
            crate::operation::get_percentiles::GetPercentilesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPercentilesFluentBuilder {
    /// Creates a new `GetPercentiles`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPercentiles as a reference.
    pub fn as_input(&self) -> &crate::operation::get_percentiles::builders::GetPercentilesInputBuilder {
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
        crate::operation::get_percentiles::GetPercentilesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_percentiles::GetPercentilesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_percentiles::GetPercentiles::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_percentiles::GetPercentiles::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_percentiles::GetPercentilesOutput,
        crate::operation::get_percentiles::GetPercentilesError,
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
    /// <p>The name of the index to search.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_name(input.into());
        self
    }
    /// <p>The name of the index to search.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_name(input);
        self
    }
    /// <p>The name of the index to search.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_name()
    }
    /// <p>The search query string.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_string(input.into());
        self
    }
    /// <p>The search query string.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_string(input);
        self
    }
    /// <p>The search query string.</p>
    pub fn get_query_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_string()
    }
    /// <p>The field to aggregate.</p>
    pub fn aggregation_field(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aggregation_field(input.into());
        self
    }
    /// <p>The field to aggregate.</p>
    pub fn set_aggregation_field(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aggregation_field(input);
        self
    }
    /// <p>The field to aggregate.</p>
    pub fn get_aggregation_field(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aggregation_field()
    }
    /// <p>The query version.</p>
    pub fn query_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_version(input.into());
        self
    }
    /// <p>The query version.</p>
    pub fn set_query_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_version(input);
        self
    }
    /// <p>The query version.</p>
    pub fn get_query_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_version()
    }
    /// Appends an item to `percents`.
    ///
    /// To override the contents of this collection use [`set_percents`](Self::set_percents).
    ///
    /// <p>The percentile groups returned.</p>
    pub fn percents(mut self, input: f64) -> Self {
        self.inner = self.inner.percents(input);
        self
    }
    /// <p>The percentile groups returned.</p>
    pub fn set_percents(mut self, input: ::std::option::Option<::std::vec::Vec<f64>>) -> Self {
        self.inner = self.inner.set_percents(input);
        self
    }
    /// <p>The percentile groups returned.</p>
    pub fn get_percents(&self) -> &::std::option::Option<::std::vec::Vec<f64>> {
        self.inner.get_percents()
    }
}
