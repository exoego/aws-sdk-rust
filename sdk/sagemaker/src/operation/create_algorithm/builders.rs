// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_algorithm::_create_algorithm_output::CreateAlgorithmOutputBuilder;

pub use crate::operation::create_algorithm::_create_algorithm_input::CreateAlgorithmInputBuilder;

impl CreateAlgorithmInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_algorithm::CreateAlgorithmOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_algorithm::CreateAlgorithmError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_algorithm();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAlgorithm`.
///
/// <p>Create a machine learning algorithm that you can use in SageMaker and list in the Amazon Web Services Marketplace.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAlgorithmFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_algorithm::builders::CreateAlgorithmInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_algorithm::CreateAlgorithmOutput,
        crate::operation::create_algorithm::CreateAlgorithmError,
    > for CreateAlgorithmFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_algorithm::CreateAlgorithmOutput,
            crate::operation::create_algorithm::CreateAlgorithmError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAlgorithmFluentBuilder {
    /// Creates a new `CreateAlgorithm`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAlgorithm as a reference.
    pub fn as_input(&self) -> &crate::operation::create_algorithm::builders::CreateAlgorithmInputBuilder {
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
        crate::operation::create_algorithm::CreateAlgorithmOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_algorithm::CreateAlgorithmError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_algorithm::CreateAlgorithm::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_algorithm::CreateAlgorithm::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_algorithm::CreateAlgorithmOutput,
        crate::operation::create_algorithm::CreateAlgorithmError,
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
    /// <p>The name of the algorithm.</p>
    pub fn algorithm_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.algorithm_name(input.into());
        self
    }
    /// <p>The name of the algorithm.</p>
    pub fn set_algorithm_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_algorithm_name(input);
        self
    }
    /// <p>The name of the algorithm.</p>
    pub fn get_algorithm_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_algorithm_name()
    }
    /// <p>A description of the algorithm.</p>
    pub fn algorithm_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.algorithm_description(input.into());
        self
    }
    /// <p>A description of the algorithm.</p>
    pub fn set_algorithm_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_algorithm_description(input);
        self
    }
    /// <p>A description of the algorithm.</p>
    pub fn get_algorithm_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_algorithm_description()
    }
    /// <p>Specifies details about training jobs run by this algorithm, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR path of the container and the version digest of the algorithm.</p></li>
    /// <li>
    /// <p>The hyperparameters that the algorithm supports.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for training.</p></li>
    /// <li>
    /// <p>Whether the algorithm supports distributed training.</p></li>
    /// <li>
    /// <p>The metrics that the algorithm emits to Amazon CloudWatch.</p></li>
    /// <li>
    /// <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p></li>
    /// <li>
    /// <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p></li>
    /// </ul>
    pub fn training_specification(mut self, input: crate::types::TrainingSpecification) -> Self {
        self.inner = self.inner.training_specification(input);
        self
    }
    /// <p>Specifies details about training jobs run by this algorithm, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR path of the container and the version digest of the algorithm.</p></li>
    /// <li>
    /// <p>The hyperparameters that the algorithm supports.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for training.</p></li>
    /// <li>
    /// <p>Whether the algorithm supports distributed training.</p></li>
    /// <li>
    /// <p>The metrics that the algorithm emits to Amazon CloudWatch.</p></li>
    /// <li>
    /// <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p></li>
    /// <li>
    /// <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p></li>
    /// </ul>
    pub fn set_training_specification(mut self, input: ::std::option::Option<crate::types::TrainingSpecification>) -> Self {
        self.inner = self.inner.set_training_specification(input);
        self
    }
    /// <p>Specifies details about training jobs run by this algorithm, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR path of the container and the version digest of the algorithm.</p></li>
    /// <li>
    /// <p>The hyperparameters that the algorithm supports.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for training.</p></li>
    /// <li>
    /// <p>Whether the algorithm supports distributed training.</p></li>
    /// <li>
    /// <p>The metrics that the algorithm emits to Amazon CloudWatch.</p></li>
    /// <li>
    /// <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p></li>
    /// <li>
    /// <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p></li>
    /// </ul>
    pub fn get_training_specification(&self) -> &::std::option::Option<crate::types::TrainingSpecification> {
        self.inner.get_training_specification()
    }
    /// <p>Specifies details about inference jobs that the algorithm runs, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for transform jobs and real-time endpoints used for inference.</p></li>
    /// <li>
    /// <p>The input and output content formats that the algorithm supports for inference.</p></li>
    /// </ul>
    pub fn inference_specification(mut self, input: crate::types::InferenceSpecification) -> Self {
        self.inner = self.inner.inference_specification(input);
        self
    }
    /// <p>Specifies details about inference jobs that the algorithm runs, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for transform jobs and real-time endpoints used for inference.</p></li>
    /// <li>
    /// <p>The input and output content formats that the algorithm supports for inference.</p></li>
    /// </ul>
    pub fn set_inference_specification(mut self, input: ::std::option::Option<crate::types::InferenceSpecification>) -> Self {
        self.inner = self.inner.set_inference_specification(input);
        self
    }
    /// <p>Specifies details about inference jobs that the algorithm runs, including the following:</p>
    /// <ul>
    /// <li>
    /// <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p></li>
    /// <li>
    /// <p>The instance types that the algorithm supports for transform jobs and real-time endpoints used for inference.</p></li>
    /// <li>
    /// <p>The input and output content formats that the algorithm supports for inference.</p></li>
    /// </ul>
    pub fn get_inference_specification(&self) -> &::std::option::Option<crate::types::InferenceSpecification> {
        self.inner.get_inference_specification()
    }
    /// <p>Specifies configurations for one or more training jobs and that SageMaker runs to test the algorithm's training code and, optionally, one or more batch transform jobs that SageMaker runs to test the algorithm's inference code.</p>
    pub fn validation_specification(mut self, input: crate::types::AlgorithmValidationSpecification) -> Self {
        self.inner = self.inner.validation_specification(input);
        self
    }
    /// <p>Specifies configurations for one or more training jobs and that SageMaker runs to test the algorithm's training code and, optionally, one or more batch transform jobs that SageMaker runs to test the algorithm's inference code.</p>
    pub fn set_validation_specification(mut self, input: ::std::option::Option<crate::types::AlgorithmValidationSpecification>) -> Self {
        self.inner = self.inner.set_validation_specification(input);
        self
    }
    /// <p>Specifies configurations for one or more training jobs and that SageMaker runs to test the algorithm's training code and, optionally, one or more batch transform jobs that SageMaker runs to test the algorithm's inference code.</p>
    pub fn get_validation_specification(&self) -> &::std::option::Option<crate::types::AlgorithmValidationSpecification> {
        self.inner.get_validation_specification()
    }
    /// <p>Whether to certify the algorithm so that it can be listed in Amazon Web Services Marketplace.</p>
    pub fn certify_for_marketplace(mut self, input: bool) -> Self {
        self.inner = self.inner.certify_for_marketplace(input);
        self
    }
    /// <p>Whether to certify the algorithm so that it can be listed in Amazon Web Services Marketplace.</p>
    pub fn set_certify_for_marketplace(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_certify_for_marketplace(input);
        self
    }
    /// <p>Whether to certify the algorithm so that it can be listed in Amazon Web Services Marketplace.</p>
    pub fn get_certify_for_marketplace(&self) -> &::std::option::Option<bool> {
        self.inner.get_certify_for_marketplace()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
