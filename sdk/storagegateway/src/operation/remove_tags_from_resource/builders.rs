// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_tags_from_resource::_remove_tags_from_resource_output::RemoveTagsFromResourceOutputBuilder;

pub use crate::operation::remove_tags_from_resource::_remove_tags_from_resource_input::RemoveTagsFromResourceInputBuilder;

impl RemoveTagsFromResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_tags_from_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveTagsFromResource`.
///
/// <p>Removes one or more tags from the specified resource. This operation is supported in storage gateways of all types.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveTagsFromResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
    > for RemoveTagsFromResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
            crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveTagsFromResourceFluentBuilder {
    /// Creates a new `RemoveTagsFromResource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveTagsFromResource as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceInputBuilder {
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
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_tags_from_resource::RemoveTagsFromResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_tags_from_resource::RemoveTagsFromResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource you want to remove the tags from.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource you want to remove the tags from.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource you want to remove the tags from.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// Appends an item to `TagKeys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>The keys of the tags you want to remove from the specified resource. A tag is composed of a key-value pair.</p>
    pub fn tag_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tag_keys(input.into());
        self
    }
    /// <p>The keys of the tags you want to remove from the specified resource. A tag is composed of a key-value pair.</p>
    pub fn set_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_tag_keys(input);
        self
    }
    /// <p>The keys of the tags you want to remove from the specified resource. A tag is composed of a key-value pair.</p>
    pub fn get_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_tag_keys()
    }
}
