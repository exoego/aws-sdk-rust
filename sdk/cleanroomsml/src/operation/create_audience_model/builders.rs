// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_audience_model::_create_audience_model_output::CreateAudienceModelOutputBuilder;

pub use crate::operation::create_audience_model::_create_audience_model_input::CreateAudienceModelInputBuilder;

impl CreateAudienceModelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_audience_model::CreateAudienceModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_audience_model::CreateAudienceModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_audience_model();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAudienceModel`.
///
/// <p>Defines the information necessary to create an audience model. An audience model is a machine learning model that Clean Rooms ML trains to measure similarity between users. Clean Rooms ML manages training and storing the audience model. The audience model can be used in multiple calls to the <code>StartAudienceGenerationJob</code> API.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAudienceModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_audience_model::builders::CreateAudienceModelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_audience_model::CreateAudienceModelOutput,
        crate::operation::create_audience_model::CreateAudienceModelError,
    > for CreateAudienceModelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_audience_model::CreateAudienceModelOutput,
            crate::operation::create_audience_model::CreateAudienceModelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAudienceModelFluentBuilder {
    /// Creates a new `CreateAudienceModel`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAudienceModel as a reference.
    pub fn as_input(&self) -> &crate::operation::create_audience_model::builders::CreateAudienceModelInputBuilder {
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
        crate::operation::create_audience_model::CreateAudienceModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_audience_model::CreateAudienceModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_audience_model::CreateAudienceModel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_audience_model::CreateAudienceModel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_audience_model::CreateAudienceModelOutput,
        crate::operation::create_audience_model::CreateAudienceModelError,
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
    /// <p>The start date and time of the training window.</p>
    pub fn training_data_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.training_data_start_time(input);
        self
    }
    /// <p>The start date and time of the training window.</p>
    pub fn set_training_data_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_training_data_start_time(input);
        self
    }
    /// <p>The start date and time of the training window.</p>
    pub fn get_training_data_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_training_data_start_time()
    }
    /// <p>The end date and time of the training window.</p>
    pub fn training_data_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.training_data_end_time(input);
        self
    }
    /// <p>The end date and time of the training window.</p>
    pub fn set_training_data_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_training_data_end_time(input);
        self
    }
    /// <p>The end date and time of the training window.</p>
    pub fn get_training_data_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_training_data_end_time()
    }
    /// <p>The name of the audience model resource.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the audience model resource.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the audience model resource.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The Amazon Resource Name (ARN) of the training dataset for this audience model.</p>
    pub fn training_dataset_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.training_dataset_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the training dataset for this audience model.</p>
    pub fn set_training_dataset_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_training_dataset_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the training dataset for this audience model.</p>
    pub fn get_training_dataset_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_training_dataset_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key. This key is used to encrypt and decrypt customer-owned data in the trained ML model and the associated data.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key. This key is used to encrypt and decrypt customer-owned data in the trained ML model and the associated data.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key. This key is used to encrypt and decrypt customer-owned data in the trained ML model and the associated data.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_arn()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The optional metadata that you apply to the resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum number of tags per resource - 50.</p></li>
    /// <li>
    /// <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p></li>
    /// <li>
    /// <p>Maximum key length - 128 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>Maximum value length - 256 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Tag keys and values are case sensitive.</p></li>
    /// <li>
    /// <p>Do not use aws:, AWS:, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has aws as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of aws do not count against your tags per resource limit.</p></li>
    /// </ul>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The optional metadata that you apply to the resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum number of tags per resource - 50.</p></li>
    /// <li>
    /// <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p></li>
    /// <li>
    /// <p>Maximum key length - 128 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>Maximum value length - 256 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Tag keys and values are case sensitive.</p></li>
    /// <li>
    /// <p>Do not use aws:, AWS:, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has aws as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of aws do not count against your tags per resource limit.</p></li>
    /// </ul>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The optional metadata that you apply to the resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li>
    /// <p>Maximum number of tags per resource - 50.</p></li>
    /// <li>
    /// <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p></li>
    /// <li>
    /// <p>Maximum key length - 128 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>Maximum value length - 256 Unicode characters in UTF-8.</p></li>
    /// <li>
    /// <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p></li>
    /// <li>
    /// <p>Tag keys and values are case sensitive.</p></li>
    /// <li>
    /// <p>Do not use aws:, AWS:, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has aws as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of aws do not count against your tags per resource limit.</p></li>
    /// </ul>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>The description of the audience model.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the audience model.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the audience model.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}
