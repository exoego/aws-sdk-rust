// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_mapping::_get_mapping_output::GetMappingOutputBuilder;

pub use crate::operation::get_mapping::_get_mapping_input::GetMappingInputBuilder;

impl GetMappingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_mapping::GetMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_mapping::GetMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_mapping();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMapping`.
///
/// <p>Creates mappings.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMappingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_mapping::builders::GetMappingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_mapping::GetMappingOutput,
        crate::operation::get_mapping::GetMappingError,
    > for GetMappingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_mapping::GetMappingOutput,
            crate::operation::get_mapping::GetMappingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMappingFluentBuilder {
    /// Creates a new `GetMapping`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMapping as a reference.
    pub fn as_input(&self) -> &crate::operation::get_mapping::builders::GetMappingInputBuilder {
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
        crate::operation::get_mapping::GetMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_mapping::GetMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_mapping::GetMapping::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_mapping::GetMapping::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_mapping::GetMappingOutput,
        crate::operation::get_mapping::GetMappingError,
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
    /// <p>Specifies the source table.</p>
    pub fn source(mut self, input: crate::types::CatalogEntry) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>Specifies the source table.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::CatalogEntry>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>Specifies the source table.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::CatalogEntry> {
        self.inner.get_source()
    }
    /// Appends an item to `Sinks`.
    ///
    /// To override the contents of this collection use [`set_sinks`](Self::set_sinks).
    ///
    /// <p>A list of target tables.</p>
    pub fn sinks(mut self, input: crate::types::CatalogEntry) -> Self {
        self.inner = self.inner.sinks(input);
        self
    }
    /// <p>A list of target tables.</p>
    pub fn set_sinks(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CatalogEntry>>) -> Self {
        self.inner = self.inner.set_sinks(input);
        self
    }
    /// <p>A list of target tables.</p>
    pub fn get_sinks(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CatalogEntry>> {
        self.inner.get_sinks()
    }
    /// <p>Parameters for the mapping.</p>
    pub fn location(mut self, input: crate::types::Location) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>Parameters for the mapping.</p>
    pub fn set_location(mut self, input: ::std::option::Option<crate::types::Location>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>Parameters for the mapping.</p>
    pub fn get_location(&self) -> &::std::option::Option<crate::types::Location> {
        self.inner.get_location()
    }
}
