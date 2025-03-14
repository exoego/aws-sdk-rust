// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_key_group::_update_key_group_output::UpdateKeyGroupOutputBuilder;

pub use crate::operation::update_key_group::_update_key_group_input::UpdateKeyGroupInputBuilder;

impl UpdateKeyGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_key_group::UpdateKeyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_key_group::UpdateKeyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_key_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateKeyGroup`.
///
/// <p>Updates a key group.</p>
/// <p>When you update a key group, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a key group:</p>
/// <ol>
/// <li>
/// <p>Get the current key group with <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p></li>
/// <li>
/// <p>Locally modify the fields in the key group that you want to update. For example, add or remove public key IDs.</p></li>
/// <li>
/// <p>Call <code>UpdateKeyGroup</code> with the entire key group object, including the fields that you modified and those that you didn't.</p></li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateKeyGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_key_group::builders::UpdateKeyGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_key_group::UpdateKeyGroupOutput,
        crate::operation::update_key_group::UpdateKeyGroupError,
    > for UpdateKeyGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_key_group::UpdateKeyGroupOutput,
            crate::operation::update_key_group::UpdateKeyGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateKeyGroupFluentBuilder {
    /// Creates a new `UpdateKeyGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateKeyGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_key_group::builders::UpdateKeyGroupInputBuilder {
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
        crate::operation::update_key_group::UpdateKeyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_key_group::UpdateKeyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_key_group::UpdateKeyGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_key_group::UpdateKeyGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_key_group::UpdateKeyGroupOutput,
        crate::operation::update_key_group::UpdateKeyGroupError,
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
    /// <p>The key group configuration.</p>
    pub fn key_group_config(mut self, input: crate::types::KeyGroupConfig) -> Self {
        self.inner = self.inner.key_group_config(input);
        self
    }
    /// <p>The key group configuration.</p>
    pub fn set_key_group_config(mut self, input: ::std::option::Option<crate::types::KeyGroupConfig>) -> Self {
        self.inner = self.inner.set_key_group_config(input);
        self
    }
    /// <p>The key group configuration.</p>
    pub fn get_key_group_config(&self) -> &::std::option::Option<crate::types::KeyGroupConfig> {
        self.inner.get_key_group_config()
    }
    /// <p>The identifier of the key group that you are updating.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of the key group that you are updating.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The identifier of the key group that you are updating.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The version of the key group that you are updating. The version is the key group's <code>ETag</code> value.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version of the key group that you are updating. The version is the key group's <code>ETag</code> value.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The version of the key group that you are updating. The version is the key group's <code>ETag</code> value.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
}
