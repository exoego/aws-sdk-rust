// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_identity_source::_update_identity_source_output::UpdateIdentitySourceOutputBuilder;

pub use crate::operation::update_identity_source::_update_identity_source_input::UpdateIdentitySourceInputBuilder;

impl UpdateIdentitySourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_identity_source::UpdateIdentitySourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_identity_source::UpdateIdentitySourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_identity_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateIdentitySource`.
///
/// <p>Updates the specified identity source to use a new identity provider (IdP) source, or to change the mapping of identities from the IdP to a different principal entity type.</p><note>
/// <p>Verified Permissions is <i> <a href="https://wikipedia.org/wiki/Eventual_consistency">eventually consistent</a> </i>. It can take a few seconds for a new or changed element to propagate through the service and be visible in the results of other Verified Permissions operations.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIdentitySourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_identity_source::builders::UpdateIdentitySourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_identity_source::UpdateIdentitySourceOutput,
        crate::operation::update_identity_source::UpdateIdentitySourceError,
    > for UpdateIdentitySourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_identity_source::UpdateIdentitySourceOutput,
            crate::operation::update_identity_source::UpdateIdentitySourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateIdentitySourceFluentBuilder {
    /// Creates a new `UpdateIdentitySource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateIdentitySource as a reference.
    pub fn as_input(&self) -> &crate::operation::update_identity_source::builders::UpdateIdentitySourceInputBuilder {
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
        crate::operation::update_identity_source::UpdateIdentitySourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_identity_source::UpdateIdentitySourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_identity_source::UpdateIdentitySource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_identity_source::UpdateIdentitySource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_identity_source::UpdateIdentitySourceOutput,
        crate::operation::update_identity_source::UpdateIdentitySourceError,
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
    /// <p>Specifies the ID of the policy store that contains the identity source that you want to update.</p>
    pub fn policy_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_store_id(input.into());
        self
    }
    /// <p>Specifies the ID of the policy store that contains the identity source that you want to update.</p>
    pub fn set_policy_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_store_id(input);
        self
    }
    /// <p>Specifies the ID of the policy store that contains the identity source that you want to update.</p>
    pub fn get_policy_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_store_id()
    }
    /// <p>Specifies the ID of the identity source that you want to update.</p>
    pub fn identity_source_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_source_id(input.into());
        self
    }
    /// <p>Specifies the ID of the identity source that you want to update.</p>
    pub fn set_identity_source_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_source_id(input);
        self
    }
    /// <p>Specifies the ID of the identity source that you want to update.</p>
    pub fn get_identity_source_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_source_id()
    }
    /// <p>Specifies the details required to communicate with the identity provider (IdP) associated with this identity source.</p><note>
    /// <p>At this time, the only valid member of this structure is a Amazon Cognito user pool configuration.</p>
    /// <p>You must specify a <code>userPoolArn</code>, and optionally, a <code>ClientId</code>.</p>
    /// </note>
    pub fn update_configuration(mut self, input: crate::types::UpdateConfiguration) -> Self {
        self.inner = self.inner.update_configuration(input);
        self
    }
    /// <p>Specifies the details required to communicate with the identity provider (IdP) associated with this identity source.</p><note>
    /// <p>At this time, the only valid member of this structure is a Amazon Cognito user pool configuration.</p>
    /// <p>You must specify a <code>userPoolArn</code>, and optionally, a <code>ClientId</code>.</p>
    /// </note>
    pub fn set_update_configuration(mut self, input: ::std::option::Option<crate::types::UpdateConfiguration>) -> Self {
        self.inner = self.inner.set_update_configuration(input);
        self
    }
    /// <p>Specifies the details required to communicate with the identity provider (IdP) associated with this identity source.</p><note>
    /// <p>At this time, the only valid member of this structure is a Amazon Cognito user pool configuration.</p>
    /// <p>You must specify a <code>userPoolArn</code>, and optionally, a <code>ClientId</code>.</p>
    /// </note>
    pub fn get_update_configuration(&self) -> &::std::option::Option<crate::types::UpdateConfiguration> {
        self.inner.get_update_configuration()
    }
    /// <p>Specifies the data type of principals generated for identities authenticated by the identity source.</p>
    pub fn principal_entity_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_entity_type(input.into());
        self
    }
    /// <p>Specifies the data type of principals generated for identities authenticated by the identity source.</p>
    pub fn set_principal_entity_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_entity_type(input);
        self
    }
    /// <p>Specifies the data type of principals generated for identities authenticated by the identity source.</p>
    pub fn get_principal_entity_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_entity_type()
    }
}
