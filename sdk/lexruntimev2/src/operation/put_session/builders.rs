// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_session::_put_session_output::PutSessionOutputBuilder;

pub use crate::operation::put_session::_put_session_input::PutSessionInputBuilder;

impl PutSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_session::PutSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_session::PutSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutSession`.
///
/// <p>Creates a new session or modifies an existing session with an Amazon Lex V2 bot. Use this operation to enable your application to set the state of the bot.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_session::builders::PutSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_session::PutSessionOutput,
        crate::operation::put_session::PutSessionError,
    > for PutSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_session::PutSessionOutput,
            crate::operation::put_session::PutSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutSessionFluentBuilder {
    /// Creates a new `PutSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutSession as a reference.
    pub fn as_input(&self) -> &crate::operation::put_session::builders::PutSessionInputBuilder {
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
        crate::operation::put_session::PutSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_session::PutSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_session::PutSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_session::PutSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_session::PutSessionOutput,
        crate::operation::put_session::PutSessionError,
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
    /// <p>The identifier of the bot that receives the session data.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot that receives the session data.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The identifier of the bot that receives the session data.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The alias identifier of the bot that receives the session data.</p>
    pub fn bot_alias_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_alias_id(input.into());
        self
    }
    /// <p>The alias identifier of the bot that receives the session data.</p>
    pub fn set_bot_alias_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_alias_id(input);
        self
    }
    /// <p>The alias identifier of the bot that receives the session data.</p>
    pub fn get_bot_alias_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_alias_id()
    }
    /// <p>The locale where the session is in use.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The identifier of the session that receives the session data.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The identifier of the session that receives the session data.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The identifier of the session that receives the session data.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// Appends an item to `messages`.
    ///
    /// To override the contents of this collection use [`set_messages`](Self::set_messages).
    ///
    /// <p>A list of messages to send to the user. Messages are sent in the order that they are defined in the list.</p>
    pub fn messages(mut self, input: crate::types::Message) -> Self {
        self.inner = self.inner.messages(input);
        self
    }
    /// <p>A list of messages to send to the user. Messages are sent in the order that they are defined in the list.</p>
    pub fn set_messages(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Message>>) -> Self {
        self.inner = self.inner.set_messages(input);
        self
    }
    /// <p>A list of messages to send to the user. Messages are sent in the order that they are defined in the list.</p>
    pub fn get_messages(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Message>> {
        self.inner.get_messages()
    }
    /// <p>Sets the state of the session with the user. You can use this to set the current intent, attributes, context, and dialog action. Use the dialog action to determine the next step that Amazon Lex V2 should use in the conversation with the user.</p>
    pub fn session_state(mut self, input: crate::types::SessionState) -> Self {
        self.inner = self.inner.session_state(input);
        self
    }
    /// <p>Sets the state of the session with the user. You can use this to set the current intent, attributes, context, and dialog action. Use the dialog action to determine the next step that Amazon Lex V2 should use in the conversation with the user.</p>
    pub fn set_session_state(mut self, input: ::std::option::Option<crate::types::SessionState>) -> Self {
        self.inner = self.inner.set_session_state(input);
        self
    }
    /// <p>Sets the state of the session with the user. You can use this to set the current intent, attributes, context, and dialog action. Use the dialog action to determine the next step that Amazon Lex V2 should use in the conversation with the user.</p>
    pub fn get_session_state(&self) -> &::std::option::Option<crate::types::SessionState> {
        self.inner.get_session_state()
    }
    /// Adds a key-value pair to `requestAttributes`.
    ///
    /// To override the contents of this collection use [`set_request_attributes`](Self::set_request_attributes).
    ///
    /// <p>Request-specific information passed between Amazon Lex V2 and the client application.</p>
    /// <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p>
    pub fn request_attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.request_attributes(k.into(), v.into());
        self
    }
    /// <p>Request-specific information passed between Amazon Lex V2 and the client application.</p>
    /// <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p>
    pub fn set_request_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_request_attributes(input);
        self
    }
    /// <p>Request-specific information passed between Amazon Lex V2 and the client application.</p>
    /// <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p>
    pub fn get_request_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_request_attributes()
    }
    /// <p>The message that Amazon Lex V2 returns in the response can be either text or speech depending on the value of this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex V2 returns text in the response.</p></li>
    /// </ul>
    pub fn response_content_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.response_content_type(input.into());
        self
    }
    /// <p>The message that Amazon Lex V2 returns in the response can be either text or speech depending on the value of this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex V2 returns text in the response.</p></li>
    /// </ul>
    pub fn set_response_content_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_response_content_type(input);
        self
    }
    /// <p>The message that Amazon Lex V2 returns in the response can be either text or speech depending on the value of this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex V2 returns text in the response.</p></li>
    /// </ul>
    pub fn get_response_content_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_response_content_type()
    }
}
