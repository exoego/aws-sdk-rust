// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_environment::_create_environment_output::CreateEnvironmentOutputBuilder;

pub use crate::operation::create_environment::_create_environment_input::CreateEnvironmentInputBuilder;

impl CreateEnvironmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_environment::CreateEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_environment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateEnvironment`.
///
/// <p>Creates an environment for your thin client devices.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_environment::builders::CreateEnvironmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_environment::CreateEnvironmentOutput,
        crate::operation::create_environment::CreateEnvironmentError,
    > for CreateEnvironmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_environment::CreateEnvironmentOutput,
            crate::operation::create_environment::CreateEnvironmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateEnvironmentFluentBuilder {
    /// Creates a new `CreateEnvironment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateEnvironment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_environment::builders::CreateEnvironmentInputBuilder {
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
        crate::operation::create_environment::CreateEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_environment::CreateEnvironment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_environment::CreateEnvironment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_environment::CreateEnvironmentOutput,
        crate::operation::create_environment::CreateEnvironmentError,
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
    /// <p>The name for the environment.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the environment.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name for the environment.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The Amazon Resource Name (ARN) of the desktop to stream from Amazon WorkSpaces, WorkSpaces Web, or AppStream 2.0.</p>
    pub fn desktop_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.desktop_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the desktop to stream from Amazon WorkSpaces, WorkSpaces Web, or AppStream 2.0.</p>
    pub fn set_desktop_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_desktop_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the desktop to stream from Amazon WorkSpaces, WorkSpaces Web, or AppStream 2.0.</p>
    pub fn get_desktop_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_desktop_arn()
    }
    /// <p>The URL for the identity provider login (only for environments that use AppStream 2.0).</p>
    pub fn desktop_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.desktop_endpoint(input.into());
        self
    }
    /// <p>The URL for the identity provider login (only for environments that use AppStream 2.0).</p>
    pub fn set_desktop_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_desktop_endpoint(input);
        self
    }
    /// <p>The URL for the identity provider login (only for environments that use AppStream 2.0).</p>
    pub fn get_desktop_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_desktop_endpoint()
    }
    /// <p>An option to define if software updates should be applied within a maintenance window.</p>
    pub fn software_set_update_schedule(mut self, input: crate::types::SoftwareSetUpdateSchedule) -> Self {
        self.inner = self.inner.software_set_update_schedule(input);
        self
    }
    /// <p>An option to define if software updates should be applied within a maintenance window.</p>
    pub fn set_software_set_update_schedule(mut self, input: ::std::option::Option<crate::types::SoftwareSetUpdateSchedule>) -> Self {
        self.inner = self.inner.set_software_set_update_schedule(input);
        self
    }
    /// <p>An option to define if software updates should be applied within a maintenance window.</p>
    pub fn get_software_set_update_schedule(&self) -> &::std::option::Option<crate::types::SoftwareSetUpdateSchedule> {
        self.inner.get_software_set_update_schedule()
    }
    /// <p>A specification for a time window to apply software updates.</p>
    pub fn maintenance_window(mut self, input: crate::types::MaintenanceWindow) -> Self {
        self.inner = self.inner.maintenance_window(input);
        self
    }
    /// <p>A specification for a time window to apply software updates.</p>
    pub fn set_maintenance_window(mut self, input: ::std::option::Option<crate::types::MaintenanceWindow>) -> Self {
        self.inner = self.inner.set_maintenance_window(input);
        self
    }
    /// <p>A specification for a time window to apply software updates.</p>
    pub fn get_maintenance_window(&self) -> &::std::option::Option<crate::types::MaintenanceWindow> {
        self.inner.get_maintenance_window()
    }
    /// <p>An option to define which software updates to apply.</p>
    pub fn software_set_update_mode(mut self, input: crate::types::SoftwareSetUpdateMode) -> Self {
        self.inner = self.inner.software_set_update_mode(input);
        self
    }
    /// <p>An option to define which software updates to apply.</p>
    pub fn set_software_set_update_mode(mut self, input: ::std::option::Option<crate::types::SoftwareSetUpdateMode>) -> Self {
        self.inner = self.inner.set_software_set_update_mode(input);
        self
    }
    /// <p>An option to define which software updates to apply.</p>
    pub fn get_software_set_update_mode(&self) -> &::std::option::Option<crate::types::SoftwareSetUpdateMode> {
        self.inner.get_software_set_update_mode()
    }
    /// <p>The ID of the software set to apply.</p>
    pub fn desired_software_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.desired_software_set_id(input.into());
        self
    }
    /// <p>The ID of the software set to apply.</p>
    pub fn set_desired_software_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_desired_software_set_id(input);
        self
    }
    /// <p>The ID of the software set to apply.</p>
    pub fn get_desired_software_set_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_desired_software_set_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service key to use to encrypt the environment.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service key to use to encrypt the environment.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service key to use to encrypt the environment.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_arn()
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
