// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_link_provider_for_user::_admin_link_provider_for_user_output::AdminLinkProviderForUserOutputBuilder;

pub use crate::operation::admin_link_provider_for_user::_admin_link_provider_for_user_input::AdminLinkProviderForUserInputBuilder;

impl AdminLinkProviderForUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_link_provider_for_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminLinkProviderForUser`.
///
/// <p>Links an existing user account in a user pool (<code>DestinationUser</code>) to an identity from an external IdP (<code>SourceUser</code>) based on a specified attribute name and value from the external IdP. This allows you to create a link from the existing user account to an external federated user identity that has not yet been used to sign in. You can then use the federated user identity to sign in as the existing user account.</p>
/// <p>For example, if there is an existing user with a username and password, this API links that user to a federated user identity. When the user signs in with a federated user identity, they sign in as the existing user account.</p><note>
/// <p>The maximum number of federated identities linked to a user is five.</p>
/// </note> <important>
/// <p>Because this API allows a user with an external federated identity to sign in as an existing user in the user pool, it is critical that it only be used with external IdPs and provider attributes that have been trusted by the application owner.</p>
/// </important> <note>
/// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
/// <p class="title"><b>Learn more</b></p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html">Signing Amazon Web Services API Requests</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a></p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminLinkProviderForUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_link_provider_for_user::builders::AdminLinkProviderForUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserOutput,
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserError,
    > for AdminLinkProviderForUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserOutput,
            crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AdminLinkProviderForUserFluentBuilder {
    /// Creates a new `AdminLinkProviderForUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminLinkProviderForUser as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_link_provider_for_user::builders::AdminLinkProviderForUserInputBuilder {
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
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_link_provider_for_user::AdminLinkProviderForUser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserOutput,
        crate::operation::admin_link_provider_for_user::AdminLinkProviderForUserError,
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
    /// <p>The user pool ID for the user pool.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for the user pool.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user pool ID for the user pool.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The existing user in the user pool that you want to assign to the external IdP user account. This user can be a local (Username + Password) Amazon Cognito user pools user or a federated user (for example, a SAML or Facebook user). If the user doesn't exist, Amazon Cognito generates an exception. Amazon Cognito returns this user when the new user (with the linked IdP attribute) signs in.</p>
    /// <p>For a native username + password user, the <code>ProviderAttributeValue</code> for the <code>DestinationUser</code> should be the username in the user pool. For a federated user, it should be the provider-specific <code>user_id</code>.</p>
    /// <p>The <code>ProviderAttributeName</code> of the <code>DestinationUser</code> is ignored.</p>
    /// <p>The <code>ProviderName</code> should be set to <code>Cognito</code> for users in Cognito user pools.</p><important>
    /// <p>All attributes in the DestinationUser profile must be mutable. If you have assigned the user any immutable custom attributes, the operation won't succeed.</p>
    /// </important>
    pub fn destination_user(mut self, input: crate::types::ProviderUserIdentifierType) -> Self {
        self.inner = self.inner.destination_user(input);
        self
    }
    /// <p>The existing user in the user pool that you want to assign to the external IdP user account. This user can be a local (Username + Password) Amazon Cognito user pools user or a federated user (for example, a SAML or Facebook user). If the user doesn't exist, Amazon Cognito generates an exception. Amazon Cognito returns this user when the new user (with the linked IdP attribute) signs in.</p>
    /// <p>For a native username + password user, the <code>ProviderAttributeValue</code> for the <code>DestinationUser</code> should be the username in the user pool. For a federated user, it should be the provider-specific <code>user_id</code>.</p>
    /// <p>The <code>ProviderAttributeName</code> of the <code>DestinationUser</code> is ignored.</p>
    /// <p>The <code>ProviderName</code> should be set to <code>Cognito</code> for users in Cognito user pools.</p><important>
    /// <p>All attributes in the DestinationUser profile must be mutable. If you have assigned the user any immutable custom attributes, the operation won't succeed.</p>
    /// </important>
    pub fn set_destination_user(mut self, input: ::std::option::Option<crate::types::ProviderUserIdentifierType>) -> Self {
        self.inner = self.inner.set_destination_user(input);
        self
    }
    /// <p>The existing user in the user pool that you want to assign to the external IdP user account. This user can be a local (Username + Password) Amazon Cognito user pools user or a federated user (for example, a SAML or Facebook user). If the user doesn't exist, Amazon Cognito generates an exception. Amazon Cognito returns this user when the new user (with the linked IdP attribute) signs in.</p>
    /// <p>For a native username + password user, the <code>ProviderAttributeValue</code> for the <code>DestinationUser</code> should be the username in the user pool. For a federated user, it should be the provider-specific <code>user_id</code>.</p>
    /// <p>The <code>ProviderAttributeName</code> of the <code>DestinationUser</code> is ignored.</p>
    /// <p>The <code>ProviderName</code> should be set to <code>Cognito</code> for users in Cognito user pools.</p><important>
    /// <p>All attributes in the DestinationUser profile must be mutable. If you have assigned the user any immutable custom attributes, the operation won't succeed.</p>
    /// </important>
    pub fn get_destination_user(&self) -> &::std::option::Option<crate::types::ProviderUserIdentifierType> {
        self.inner.get_destination_user()
    }
    /// <p>An external IdP account for a user who doesn't exist yet in the user pool. This user must be a federated user (for example, a SAML or Facebook user), not another native user.</p>
    /// <p>If the <code>SourceUser</code> is using a federated social IdP, such as Facebook, Google, or Login with Amazon, you must set the <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>. For social IdPs, the <code>ProviderName</code> will be <code>Facebook</code>, <code>Google</code>, or <code>LoginWithAmazon</code>, and Amazon Cognito will automatically parse the Facebook, Google, and Login with Amazon tokens for <code>id</code>, <code>sub</code>, and <code>user_id</code>, respectively. The <code>ProviderAttributeValue</code> for the user must be the same value as the <code>id</code>, <code>sub</code>, or <code>user_id</code> value found in the social IdP token.</p>
    /// <p></p>
    /// <p>For OIDC, the <code>ProviderAttributeName</code> can be any value that matches a claim in the ID token, or that your app retrieves from the <code>userInfo</code> endpoint. You must map the claim to a user pool attribute in your IdP configuration, and set the user pool attribute name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For SAML, the <code>ProviderAttributeName</code> can be any value that matches a claim in the SAML assertion. To link SAML users based on the subject of the SAML assertion, map the subject to a claim through the SAML IdP and set that claim name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For both OIDC and SAML users, when you set <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>, Amazon Cognito will automatically parse the default unique identifier found in the subject from the IdP token.</p>
    pub fn source_user(mut self, input: crate::types::ProviderUserIdentifierType) -> Self {
        self.inner = self.inner.source_user(input);
        self
    }
    /// <p>An external IdP account for a user who doesn't exist yet in the user pool. This user must be a federated user (for example, a SAML or Facebook user), not another native user.</p>
    /// <p>If the <code>SourceUser</code> is using a federated social IdP, such as Facebook, Google, or Login with Amazon, you must set the <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>. For social IdPs, the <code>ProviderName</code> will be <code>Facebook</code>, <code>Google</code>, or <code>LoginWithAmazon</code>, and Amazon Cognito will automatically parse the Facebook, Google, and Login with Amazon tokens for <code>id</code>, <code>sub</code>, and <code>user_id</code>, respectively. The <code>ProviderAttributeValue</code> for the user must be the same value as the <code>id</code>, <code>sub</code>, or <code>user_id</code> value found in the social IdP token.</p>
    /// <p></p>
    /// <p>For OIDC, the <code>ProviderAttributeName</code> can be any value that matches a claim in the ID token, or that your app retrieves from the <code>userInfo</code> endpoint. You must map the claim to a user pool attribute in your IdP configuration, and set the user pool attribute name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For SAML, the <code>ProviderAttributeName</code> can be any value that matches a claim in the SAML assertion. To link SAML users based on the subject of the SAML assertion, map the subject to a claim through the SAML IdP and set that claim name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For both OIDC and SAML users, when you set <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>, Amazon Cognito will automatically parse the default unique identifier found in the subject from the IdP token.</p>
    pub fn set_source_user(mut self, input: ::std::option::Option<crate::types::ProviderUserIdentifierType>) -> Self {
        self.inner = self.inner.set_source_user(input);
        self
    }
    /// <p>An external IdP account for a user who doesn't exist yet in the user pool. This user must be a federated user (for example, a SAML or Facebook user), not another native user.</p>
    /// <p>If the <code>SourceUser</code> is using a federated social IdP, such as Facebook, Google, or Login with Amazon, you must set the <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>. For social IdPs, the <code>ProviderName</code> will be <code>Facebook</code>, <code>Google</code>, or <code>LoginWithAmazon</code>, and Amazon Cognito will automatically parse the Facebook, Google, and Login with Amazon tokens for <code>id</code>, <code>sub</code>, and <code>user_id</code>, respectively. The <code>ProviderAttributeValue</code> for the user must be the same value as the <code>id</code>, <code>sub</code>, or <code>user_id</code> value found in the social IdP token.</p>
    /// <p></p>
    /// <p>For OIDC, the <code>ProviderAttributeName</code> can be any value that matches a claim in the ID token, or that your app retrieves from the <code>userInfo</code> endpoint. You must map the claim to a user pool attribute in your IdP configuration, and set the user pool attribute name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For SAML, the <code>ProviderAttributeName</code> can be any value that matches a claim in the SAML assertion. To link SAML users based on the subject of the SAML assertion, map the subject to a claim through the SAML IdP and set that claim name as the value of <code>ProviderAttributeName</code> in your <code>AdminLinkProviderForUser</code> request.</p>
    /// <p>For both OIDC and SAML users, when you set <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>, Amazon Cognito will automatically parse the default unique identifier found in the subject from the IdP token.</p>
    pub fn get_source_user(&self) -> &::std::option::Option<crate::types::ProviderUserIdentifierType> {
        self.inner.get_source_user()
    }
}
