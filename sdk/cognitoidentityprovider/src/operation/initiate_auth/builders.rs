// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::initiate_auth::_initiate_auth_output::InitiateAuthOutputBuilder;

pub use crate::operation::initiate_auth::_initiate_auth_input::InitiateAuthInputBuilder;

impl InitiateAuthInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::initiate_auth::InitiateAuthOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_auth::InitiateAuthError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.initiate_auth();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `InitiateAuth`.
///
/// <p>Initiates sign-in for a user in the Amazon Cognito user directory. You can't sign in a user with a federated IdP with <code>InitiateAuth</code>. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-identity-federation.html"> Adding user pool sign-in through a third party</a>.</p><note>
/// <p>Amazon Cognito doesn't evaluate Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you can't use IAM credentials to authorize requests, and you can't grant IAM permissions in policies. For more information about authorization models in Amazon Cognito, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a>.</p>
/// </note> <note>
/// <p>This action might generate an SMS text message. Starting June 1, 2021, US telecom carriers require you to register an origination phone number before you can send SMS messages to US phone numbers. If you use SMS text messages in Amazon Cognito, you must register a phone number with <a href="https://console.aws.amazon.com/pinpoint/home/">Amazon Pinpoint</a>. Amazon Cognito uses the registered number automatically. Otherwise, Amazon Cognito users who must receive SMS messages might not be able to sign up, activate their accounts, or sign in.</p>
/// <p>If you have never used SMS text messages with Amazon Cognito or any other Amazon Web Service, Amazon Simple Notification Service might place your account in the SMS sandbox. In <i> <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">sandbox mode</a> </i>, you can send messages only to verified phone numbers. After you test your app while in the sandbox environment, you can move out of the sandbox and into production. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html"> SMS message settings for Amazon Cognito user pools</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InitiateAuthFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::initiate_auth::builders::InitiateAuthInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::initiate_auth::InitiateAuthOutput,
        crate::operation::initiate_auth::InitiateAuthError,
    > for InitiateAuthFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::initiate_auth::InitiateAuthOutput,
            crate::operation::initiate_auth::InitiateAuthError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InitiateAuthFluentBuilder {
    /// Creates a new `InitiateAuth`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the InitiateAuth as a reference.
    pub fn as_input(&self) -> &crate::operation::initiate_auth::builders::InitiateAuthInputBuilder {
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
        crate::operation::initiate_auth::InitiateAuthOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_auth::InitiateAuthError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::initiate_auth::InitiateAuth::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::initiate_auth::InitiateAuth::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::initiate_auth::InitiateAuthOutput,
        crate::operation::initiate_auth::InitiateAuthError,
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
    /// <p>The authentication flow for this call to run. The API action will depend on this value. For example:</p>
    /// <ul>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code> takes in a valid refresh token and returns new tokens.</p></li>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code> takes in <code>USERNAME</code> and <code>SRP_A</code> and returns the SRP variables to be used for next challenge execution.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code> takes in <code>USERNAME</code> and <code>PASSWORD</code> and returns the next challenge or tokens.</p></li>
    /// </ul>
    /// <p>Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p></li>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p></li>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code>: Custom authentication flow.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code>: Non-SRP authentication flow; user name and password are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if it doesn't find the user name in the user pool.</p></li>
    /// </ul>
    /// <p><code>ADMIN_NO_SRP_AUTH</code> isn't a valid value.</p>
    pub fn auth_flow(mut self, input: crate::types::AuthFlowType) -> Self {
        self.inner = self.inner.auth_flow(input);
        self
    }
    /// <p>The authentication flow for this call to run. The API action will depend on this value. For example:</p>
    /// <ul>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code> takes in a valid refresh token and returns new tokens.</p></li>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code> takes in <code>USERNAME</code> and <code>SRP_A</code> and returns the SRP variables to be used for next challenge execution.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code> takes in <code>USERNAME</code> and <code>PASSWORD</code> and returns the next challenge or tokens.</p></li>
    /// </ul>
    /// <p>Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p></li>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p></li>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code>: Custom authentication flow.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code>: Non-SRP authentication flow; user name and password are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if it doesn't find the user name in the user pool.</p></li>
    /// </ul>
    /// <p><code>ADMIN_NO_SRP_AUTH</code> isn't a valid value.</p>
    pub fn set_auth_flow(mut self, input: ::std::option::Option<crate::types::AuthFlowType>) -> Self {
        self.inner = self.inner.set_auth_flow(input);
        self
    }
    /// <p>The authentication flow for this call to run. The API action will depend on this value. For example:</p>
    /// <ul>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code> takes in a valid refresh token and returns new tokens.</p></li>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code> takes in <code>USERNAME</code> and <code>SRP_A</code> and returns the SRP variables to be used for next challenge execution.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code> takes in <code>USERNAME</code> and <code>PASSWORD</code> and returns the next challenge or tokens.</p></li>
    /// </ul>
    /// <p>Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p></li>
    /// <li>
    /// <p><code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p></li>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code>: Custom authentication flow.</p></li>
    /// <li>
    /// <p><code>USER_PASSWORD_AUTH</code>: Non-SRP authentication flow; user name and password are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if it doesn't find the user name in the user pool.</p></li>
    /// </ul>
    /// <p><code>ADMIN_NO_SRP_AUTH</code> isn't a valid value.</p>
    pub fn get_auth_flow(&self) -> &::std::option::Option<crate::types::AuthFlowType> {
        self.inner.get_auth_flow()
    }
    /// Adds a key-value pair to `AuthParameters`.
    ///
    /// To override the contents of this collection use [`set_auth_parameters`](Self::set_auth_parameters).
    ///
    /// <p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking. The required values depend on the value of <code>AuthFlow</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For <code>USER_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SRP_A</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>USER_PASSWORD_AUTH</code>: <code>USERNAME</code> (required), <code>PASSWORD</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</code>: <code>REFRESH_TOKEN</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>CUSTOM_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code>. To start the authentication flow with password verification, include <code>ChallengeName: SRP_A</code> and <code>SRP_A: (The SRP_A Value)</code>.</p></li>
    /// </ul>
    /// <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p>
    pub fn auth_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.auth_parameters(k.into(), v.into());
        self
    }
    /// <p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking. The required values depend on the value of <code>AuthFlow</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For <code>USER_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SRP_A</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>USER_PASSWORD_AUTH</code>: <code>USERNAME</code> (required), <code>PASSWORD</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</code>: <code>REFRESH_TOKEN</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>CUSTOM_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code>. To start the authentication flow with password verification, include <code>ChallengeName: SRP_A</code> and <code>SRP_A: (The SRP_A Value)</code>.</p></li>
    /// </ul>
    /// <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p>
    pub fn set_auth_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_auth_parameters(input);
        self
    }
    /// <p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you're invoking. The required values depend on the value of <code>AuthFlow</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For <code>USER_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SRP_A</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>USER_PASSWORD_AUTH</code>: <code>USERNAME</code> (required), <code>PASSWORD</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</code>: <code>REFRESH_TOKEN</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code>.</p></li>
    /// <li>
    /// <p>For <code>CUSTOM_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code>. To start the authentication flow with password verification, include <code>ChallengeName: SRP_A</code> and <code>SRP_A: (The SRP_A Value)</code>.</p></li>
    /// </ul>
    /// <p>For more information about <code>SECRET_HASH</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/signing-up-users-in-your-app.html#cognito-user-pools-computing-secret-hash">Computing secret hash values</a>. For information about <code>DEVICE_KEY</code>, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with user devices in your user pool</a>.</p>
    pub fn get_auth_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_auth_parameters()
    }
    /// Adds a key-value pair to `ClientMetadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the InitiateAuth API action, Amazon Cognito invokes the Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p>
    /// <ul>
    /// <li>
    /// <p>Pre signup</p></li>
    /// <li>
    /// <p>Pre authentication</p></li>
    /// <li>
    /// <p>User migration</p></li>
    /// </ul>
    /// <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your InitiateAuth request. In your function code in Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p>
    /// <p>When you use the InitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it doesn't provide the ClientMetadata value as input:</p>
    /// <ul>
    /// <li>
    /// <p>Post authentication</p></li>
    /// <li>
    /// <p>Custom message</p></li>
    /// <li>
    /// <p>Pre token generation</p></li>
    /// <li>
    /// <p>Create auth challenge</p></li>
    /// <li>
    /// <p>Define auth challenge</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the ClientMetadata value.</p></li>
    /// <li>
    /// <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p></li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_metadata(k.into(), v.into());
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the InitiateAuth API action, Amazon Cognito invokes the Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p>
    /// <ul>
    /// <li>
    /// <p>Pre signup</p></li>
    /// <li>
    /// <p>Pre authentication</p></li>
    /// <li>
    /// <p>User migration</p></li>
    /// </ul>
    /// <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your InitiateAuth request. In your function code in Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p>
    /// <p>When you use the InitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it doesn't provide the ClientMetadata value as input:</p>
    /// <ul>
    /// <li>
    /// <p>Post authentication</p></li>
    /// <li>
    /// <p>Custom message</p></li>
    /// <li>
    /// <p>Pre token generation</p></li>
    /// <li>
    /// <p>Create auth challenge</p></li>
    /// <li>
    /// <p>Define auth challenge</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the ClientMetadata value.</p></li>
    /// <li>
    /// <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p></li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_metadata(input);
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the InitiateAuth API action, Amazon Cognito invokes the Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p>
    /// <ul>
    /// <li>
    /// <p>Pre signup</p></li>
    /// <li>
    /// <p>Pre authentication</p></li>
    /// <li>
    /// <p>User migration</p></li>
    /// </ul>
    /// <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your InitiateAuth request. In your function code in Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p>
    /// <p>When you use the InitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it doesn't provide the ClientMetadata value as input:</p>
    /// <ul>
    /// <li>
    /// <p>Post authentication</p></li>
    /// <li>
    /// <p>Custom message</p></li>
    /// <li>
    /// <p>Pre token generation</p></li>
    /// <li>
    /// <p>Create auth challenge</p></li>
    /// <li>
    /// <p>Define auth challenge</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the ClientMetadata value.</p></li>
    /// <li>
    /// <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p></li>
    /// </ul>
    /// </note>
    pub fn get_client_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_client_metadata()
    }
    /// <p>The app client ID.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The app client ID.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
    /// <p>The app client ID.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_id()
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>InitiateAuth</code> calls.</p>
    pub fn analytics_metadata(mut self, input: crate::types::AnalyticsMetadataType) -> Self {
        self.inner = self.inner.analytics_metadata(input);
        self
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>InitiateAuth</code> calls.</p>
    pub fn set_analytics_metadata(mut self, input: ::std::option::Option<crate::types::AnalyticsMetadataType>) -> Self {
        self.inner = self.inner.set_analytics_metadata(input);
        self
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>InitiateAuth</code> calls.</p>
    pub fn get_analytics_metadata(&self) -> &::std::option::Option<crate::types::AnalyticsMetadataType> {
        self.inner.get_analytics_metadata()
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn user_context_data(mut self, input: crate::types::UserContextDataType) -> Self {
        self.inner = self.inner.user_context_data(input);
        self
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn set_user_context_data(mut self, input: ::std::option::Option<crate::types::UserContextDataType>) -> Self {
        self.inner = self.inner.set_user_context_data(input);
        self
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn get_user_context_data(&self) -> &::std::option::Option<crate::types::UserContextDataType> {
        self.inner.get_user_context_data()
    }
}
