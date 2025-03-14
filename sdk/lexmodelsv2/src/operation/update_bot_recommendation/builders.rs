// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bot_recommendation::_update_bot_recommendation_output::UpdateBotRecommendationOutputBuilder;

pub use crate::operation::update_bot_recommendation::_update_bot_recommendation_input::UpdateBotRecommendationInputBuilder;

impl UpdateBotRecommendationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_bot_recommendation::UpdateBotRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bot_recommendation::UpdateBotRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_bot_recommendation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBotRecommendation`.
///
/// <p>Updates an existing bot recommendation request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBotRecommendationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bot_recommendation::builders::UpdateBotRecommendationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_bot_recommendation::UpdateBotRecommendationOutput,
        crate::operation::update_bot_recommendation::UpdateBotRecommendationError,
    > for UpdateBotRecommendationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_bot_recommendation::UpdateBotRecommendationOutput,
            crate::operation::update_bot_recommendation::UpdateBotRecommendationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBotRecommendationFluentBuilder {
    /// Creates a new `UpdateBotRecommendation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBotRecommendation as a reference.
    pub fn as_input(&self) -> &crate::operation::update_bot_recommendation::builders::UpdateBotRecommendationInputBuilder {
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
        crate::operation::update_bot_recommendation::UpdateBotRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bot_recommendation::UpdateBotRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_bot_recommendation::UpdateBotRecommendation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_bot_recommendation::UpdateBotRecommendation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_bot_recommendation::UpdateBotRecommendationOutput,
        crate::operation::update_bot_recommendation::UpdateBotRecommendationError,
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
    /// <p>The unique identifier of the bot containing the bot recommendation to be updated.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot containing the bot recommendation to be updated.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The unique identifier of the bot containing the bot recommendation to be updated.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The version of the bot containing the bot recommendation to be updated.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot containing the bot recommendation to be updated.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The version of the bot containing the bot recommendation to be updated.</p>
    pub fn get_bot_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_version()
    }
    /// <p>The identifier of the language and locale of the bot recommendation to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a></p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale of the bot recommendation to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a></p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The identifier of the language and locale of the bot recommendation to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a></p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The unique identifier of the bot recommendation to be updated.</p>
    pub fn bot_recommendation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_recommendation_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot recommendation to be updated.</p>
    pub fn set_bot_recommendation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_recommendation_id(input);
        self
    }
    /// <p>The unique identifier of the bot recommendation to be updated.</p>
    pub fn get_bot_recommendation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_recommendation_id()
    }
    /// <p>The object representing the passwords that will be used to encrypt the data related to the bot recommendation results, as well as the KMS key ARN used to encrypt the associated metadata.</p>
    pub fn encryption_setting(mut self, input: crate::types::EncryptionSetting) -> Self {
        self.inner = self.inner.encryption_setting(input);
        self
    }
    /// <p>The object representing the passwords that will be used to encrypt the data related to the bot recommendation results, as well as the KMS key ARN used to encrypt the associated metadata.</p>
    pub fn set_encryption_setting(mut self, input: ::std::option::Option<crate::types::EncryptionSetting>) -> Self {
        self.inner = self.inner.set_encryption_setting(input);
        self
    }
    /// <p>The object representing the passwords that will be used to encrypt the data related to the bot recommendation results, as well as the KMS key ARN used to encrypt the associated metadata.</p>
    pub fn get_encryption_setting(&self) -> &::std::option::Option<crate::types::EncryptionSetting> {
        self.inner.get_encryption_setting()
    }
}
