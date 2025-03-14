// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_theme_permissions::_describe_theme_permissions_output::DescribeThemePermissionsOutputBuilder;

pub use crate::operation::describe_theme_permissions::_describe_theme_permissions_input::DescribeThemePermissionsInputBuilder;

impl DescribeThemePermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_theme_permissions::DescribeThemePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_theme_permissions::DescribeThemePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_theme_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeThemePermissions`.
///
/// <p>Describes the read and write permissions for a theme.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeThemePermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_theme_permissions::builders::DescribeThemePermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_theme_permissions::DescribeThemePermissionsOutput,
        crate::operation::describe_theme_permissions::DescribeThemePermissionsError,
    > for DescribeThemePermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_theme_permissions::DescribeThemePermissionsOutput,
            crate::operation::describe_theme_permissions::DescribeThemePermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeThemePermissionsFluentBuilder {
    /// Creates a new `DescribeThemePermissions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeThemePermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_theme_permissions::builders::DescribeThemePermissionsInputBuilder {
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
        crate::operation::describe_theme_permissions::DescribeThemePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_theme_permissions::DescribeThemePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_theme_permissions::DescribeThemePermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_theme_permissions::DescribeThemePermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_theme_permissions::DescribeThemePermissionsOutput,
        crate::operation::describe_theme_permissions::DescribeThemePermissionsError,
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
    /// <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the theme that you want to describe permissions for.</p>
    pub fn theme_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.theme_id(input.into());
        self
    }
    /// <p>The ID for the theme that you want to describe permissions for.</p>
    pub fn set_theme_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_theme_id(input);
        self
    }
    /// <p>The ID for the theme that you want to describe permissions for.</p>
    pub fn get_theme_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_theme_id()
    }
}
