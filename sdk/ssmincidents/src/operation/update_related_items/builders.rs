// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_related_items::_update_related_items_output::UpdateRelatedItemsOutputBuilder;

pub use crate::operation::update_related_items::_update_related_items_input::UpdateRelatedItemsInputBuilder;

impl UpdateRelatedItemsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_related_items::UpdateRelatedItemsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_related_items::UpdateRelatedItemsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_related_items();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRelatedItems`.
///
/// <p>Add or remove related items from the related items tab of an incident record.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRelatedItemsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_related_items::builders::UpdateRelatedItemsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_related_items::UpdateRelatedItemsOutput,
        crate::operation::update_related_items::UpdateRelatedItemsError,
    > for UpdateRelatedItemsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_related_items::UpdateRelatedItemsOutput,
            crate::operation::update_related_items::UpdateRelatedItemsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRelatedItemsFluentBuilder {
    /// Creates a new `UpdateRelatedItems`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRelatedItems as a reference.
    pub fn as_input(&self) -> &crate::operation::update_related_items::builders::UpdateRelatedItemsInputBuilder {
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
        crate::operation::update_related_items::UpdateRelatedItemsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_related_items::UpdateRelatedItemsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_related_items::UpdateRelatedItems::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_related_items::UpdateRelatedItems::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_related_items::UpdateRelatedItemsOutput,
        crate::operation::update_related_items::UpdateRelatedItemsError,
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
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A token that ensures that a client calls the operation only once with the specified details.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The Amazon Resource Name (ARN) of the incident record that contains the related items that you update.</p>
    pub fn incident_record_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.incident_record_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident record that contains the related items that you update.</p>
    pub fn set_incident_record_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_incident_record_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the incident record that contains the related items that you update.</p>
    pub fn get_incident_record_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_incident_record_arn()
    }
    /// <p>Details about the item that you are add to, or delete from, an incident.</p>
    pub fn related_items_update(mut self, input: crate::types::RelatedItemsUpdate) -> Self {
        self.inner = self.inner.related_items_update(input);
        self
    }
    /// <p>Details about the item that you are add to, or delete from, an incident.</p>
    pub fn set_related_items_update(mut self, input: ::std::option::Option<crate::types::RelatedItemsUpdate>) -> Self {
        self.inner = self.inner.set_related_items_update(input);
        self
    }
    /// <p>Details about the item that you are add to, or delete from, an incident.</p>
    pub fn get_related_items_update(&self) -> &::std::option::Option<crate::types::RelatedItemsUpdate> {
        self.inner.get_related_items_update()
    }
}
