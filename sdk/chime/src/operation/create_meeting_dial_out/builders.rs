// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_meeting_dial_out::_create_meeting_dial_out_output::CreateMeetingDialOutOutputBuilder;

pub use crate::operation::create_meeting_dial_out::_create_meeting_dial_out_input::CreateMeetingDialOutInputBuilder;

impl CreateMeetingDialOutInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_meeting_dial_out::CreateMeetingDialOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_meeting_dial_out();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMeetingDialOut`.
///
/// <p>Uses the join token and call metadata in a meeting request (From number, To number, and so forth) to initiate an outbound call to a public switched telephone network (PSTN) and join them into a Chime meeting. Also ensures that the From number belongs to the customer.</p>
/// <p>To play welcome audio or implement an interactive voice response (IVR), use the <code>CreateSipMediaApplicationCall</code> action with the corresponding SIP media application ID.</p><important>
/// <p><b>This API is is not available in a dedicated namespace.</b></p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMeetingDialOutFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_meeting_dial_out::builders::CreateMeetingDialOutInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutOutput,
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutError,
    > for CreateMeetingDialOutFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_meeting_dial_out::CreateMeetingDialOutOutput,
            crate::operation::create_meeting_dial_out::CreateMeetingDialOutError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateMeetingDialOutFluentBuilder {
    /// Creates a new `CreateMeetingDialOut`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateMeetingDialOut as a reference.
    pub fn as_input(&self) -> &crate::operation::create_meeting_dial_out::builders::CreateMeetingDialOutInputBuilder {
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
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_meeting_dial_out::CreateMeetingDialOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_meeting_dial_out::CreateMeetingDialOut::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_meeting_dial_out::CreateMeetingDialOut::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutOutput,
        crate::operation::create_meeting_dial_out::CreateMeetingDialOutError,
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
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_id()
    }
    /// <p>Phone number used as the caller ID when the remote party receives a call.</p>
    pub fn from_phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.from_phone_number(input.into());
        self
    }
    /// <p>Phone number used as the caller ID when the remote party receives a call.</p>
    pub fn set_from_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_from_phone_number(input);
        self
    }
    /// <p>Phone number used as the caller ID when the remote party receives a call.</p>
    pub fn get_from_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_from_phone_number()
    }
    /// <p>Phone number called when inviting someone to a meeting.</p>
    pub fn to_phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.to_phone_number(input.into());
        self
    }
    /// <p>Phone number called when inviting someone to a meeting.</p>
    pub fn set_to_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_to_phone_number(input);
        self
    }
    /// <p>Phone number called when inviting someone to a meeting.</p>
    pub fn get_to_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_to_phone_number()
    }
    /// <p>Token used by the Amazon Chime SDK attendee. Call the <a href="https://docs.aws.amazon.com/chime/latest/APIReference/API_CreateAttendee.html">CreateAttendee</a> action to get a join token.</p>
    pub fn join_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.join_token(input.into());
        self
    }
    /// <p>Token used by the Amazon Chime SDK attendee. Call the <a href="https://docs.aws.amazon.com/chime/latest/APIReference/API_CreateAttendee.html">CreateAttendee</a> action to get a join token.</p>
    pub fn set_join_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_join_token(input);
        self
    }
    /// <p>Token used by the Amazon Chime SDK attendee. Call the <a href="https://docs.aws.amazon.com/chime/latest/APIReference/API_CreateAttendee.html">CreateAttendee</a> action to get a join token.</p>
    pub fn get_join_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_join_token()
    }
}
