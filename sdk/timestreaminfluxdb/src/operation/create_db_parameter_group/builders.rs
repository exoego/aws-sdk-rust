// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_db_parameter_group::_create_db_parameter_group_output::CreateDbParameterGroupOutputBuilder;

pub use crate::operation::create_db_parameter_group::_create_db_parameter_group_input::CreateDbParameterGroupInputBuilder;

impl CreateDbParameterGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_db_parameter_group::CreateDbParameterGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_db_parameter_group::CreateDbParameterGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_db_parameter_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDbParameterGroup`.
///
/// <p>Creates a new Timestream for InfluxDB DB parameter group to associate with DB instances.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDbParameterGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_db_parameter_group::builders::CreateDbParameterGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_db_parameter_group::CreateDbParameterGroupOutput,
        crate::operation::create_db_parameter_group::CreateDbParameterGroupError,
    > for CreateDbParameterGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_db_parameter_group::CreateDbParameterGroupOutput,
            crate::operation::create_db_parameter_group::CreateDbParameterGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDbParameterGroupFluentBuilder {
    /// Creates a new `CreateDbParameterGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDbParameterGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_db_parameter_group::builders::CreateDbParameterGroupInputBuilder {
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
        crate::operation::create_db_parameter_group::CreateDbParameterGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_db_parameter_group::CreateDbParameterGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_db_parameter_group::CreateDbParameterGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_db_parameter_group::CreateDbParameterGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_db_parameter_group::CreateDbParameterGroupOutput,
        crate::operation::create_db_parameter_group::CreateDbParameterGroupError,
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
    /// <p>The name of the DB parameter group. The name must be unique per customer and per region.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the DB parameter group. The name must be unique per customer and per region.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the DB parameter group. The name must be unique per customer and per region.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description of the DB parameter group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the DB parameter group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the DB parameter group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>A list of the parameters that comprise the DB parameter group.</p>
    pub fn parameters(mut self, input: crate::types::Parameters) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>A list of the parameters that comprise the DB parameter group.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<crate::types::Parameters>) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>A list of the parameters that comprise the DB parameter group.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<crate::types::Parameters> {
        self.inner.get_parameters()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pairs to associate with the DB parameter group.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A list of key-value pairs to associate with the DB parameter group.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of key-value pairs to associate with the DB parameter group.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
