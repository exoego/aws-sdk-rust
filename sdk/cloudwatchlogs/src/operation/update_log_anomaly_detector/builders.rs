// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_log_anomaly_detector::_update_log_anomaly_detector_output::UpdateLogAnomalyDetectorOutputBuilder;

pub use crate::operation::update_log_anomaly_detector::_update_log_anomaly_detector_input::UpdateLogAnomalyDetectorInputBuilder;

impl UpdateLogAnomalyDetectorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_log_anomaly_detector();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLogAnomalyDetector`.
///
/// <p>Updates an existing log anomaly detector.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLogAnomalyDetectorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_log_anomaly_detector::builders::UpdateLogAnomalyDetectorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorOutput,
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorError,
    > for UpdateLogAnomalyDetectorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorOutput,
            crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLogAnomalyDetectorFluentBuilder {
    /// Creates a new `UpdateLogAnomalyDetector`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLogAnomalyDetector as a reference.
    pub fn as_input(&self) -> &crate::operation::update_log_anomaly_detector::builders::UpdateLogAnomalyDetectorInputBuilder {
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
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetector::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetector::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorOutput,
        crate::operation::update_log_anomaly_detector::UpdateLogAnomalyDetectorError,
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
    /// <p>The ARN of the anomaly detector that you want to update.</p>
    pub fn anomaly_detector_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.anomaly_detector_arn(input.into());
        self
    }
    /// <p>The ARN of the anomaly detector that you want to update.</p>
    pub fn set_anomaly_detector_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_anomaly_detector_arn(input);
        self
    }
    /// <p>The ARN of the anomaly detector that you want to update.</p>
    pub fn get_anomaly_detector_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_anomaly_detector_arn()
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn evaluation_frequency(mut self, input: crate::types::EvaluationFrequency) -> Self {
        self.inner = self.inner.evaluation_frequency(input);
        self
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn set_evaluation_frequency(mut self, input: ::std::option::Option<crate::types::EvaluationFrequency>) -> Self {
        self.inner = self.inner.set_evaluation_frequency(input);
        self
    }
    /// <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to <code>FIFTEEN_MIN</code> might be appropriate.</p>
    pub fn get_evaluation_frequency(&self) -> &::std::option::Option<crate::types::EvaluationFrequency> {
        self.inner.get_evaluation_frequency()
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn filter_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_pattern(input.into());
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn set_filter_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_filter_pattern(input);
        self
    }
    /// <p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event can contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>
    pub fn get_filter_pattern(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_filter_pattern()
    }
    /// <p>The number of days to use as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal. Therefore, if you do not correct the cause of an anomaly during this time, it will be considered normal going forward and will not be detected.</p>
    pub fn anomaly_visibility_time(mut self, input: i64) -> Self {
        self.inner = self.inner.anomaly_visibility_time(input);
        self
    }
    /// <p>The number of days to use as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal. Therefore, if you do not correct the cause of an anomaly during this time, it will be considered normal going forward and will not be detected.</p>
    pub fn set_anomaly_visibility_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_anomaly_visibility_time(input);
        self
    }
    /// <p>The number of days to use as the life cycle of anomalies. After this time, anomalies are automatically baselined and the anomaly detector model will treat new occurrences of similar event as normal. Therefore, if you do not correct the cause of an anomaly during this time, it will be considered normal going forward and will not be detected.</p>
    pub fn get_anomaly_visibility_time(&self) -> &::std::option::Option<i64> {
        self.inner.get_anomaly_visibility_time()
    }
    /// <p>Use this parameter to pause or restart the anomaly detector.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Use this parameter to pause or restart the anomaly detector.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// <p>Use this parameter to pause or restart the anomaly detector.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_enabled()
    }
}
