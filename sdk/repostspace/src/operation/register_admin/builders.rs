// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_admin::_register_admin_output::RegisterAdminOutputBuilder;

pub use crate::operation::register_admin::_register_admin_input::RegisterAdminInputBuilder;

impl RegisterAdminInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_admin::RegisterAdminOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_admin::RegisterAdminError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_admin();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterAdmin`.
///
/// <p>Adds a user or group to the list of administrators of the private re:Post.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterAdminFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_admin::builders::RegisterAdminInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_admin::RegisterAdminOutput,
        crate::operation::register_admin::RegisterAdminError,
    > for RegisterAdminFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_admin::RegisterAdminOutput,
            crate::operation::register_admin::RegisterAdminError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterAdminFluentBuilder {
    /// Creates a new `RegisterAdmin`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterAdmin as a reference.
    pub fn as_input(&self) -> &crate::operation::register_admin::builders::RegisterAdminInputBuilder {
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
        crate::operation::register_admin::RegisterAdminOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_admin::RegisterAdminError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_admin::RegisterAdmin::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_admin::RegisterAdmin::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_admin::RegisterAdminOutput,
        crate::operation::register_admin::RegisterAdminError,
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
    /// <p>The ID of the private re:Post.</p>
    pub fn space_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_id(input.into());
        self
    }
    /// <p>The ID of the private re:Post.</p>
    pub fn set_space_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_id(input);
        self
    }
    /// <p>The ID of the private re:Post.</p>
    pub fn get_space_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_id()
    }
    /// <p>The ID of the administrator.</p>
    pub fn admin_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.admin_id(input.into());
        self
    }
    /// <p>The ID of the administrator.</p>
    pub fn set_admin_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_admin_id(input);
        self
    }
    /// <p>The ID of the administrator.</p>
    pub fn get_admin_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_admin_id()
    }
}
