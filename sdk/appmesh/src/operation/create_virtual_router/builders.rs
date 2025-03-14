// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_virtual_router::_create_virtual_router_output::CreateVirtualRouterOutputBuilder;

pub use crate::operation::create_virtual_router::_create_virtual_router_input::CreateVirtualRouterInputBuilder;

impl CreateVirtualRouterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_virtual_router::CreateVirtualRouterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_virtual_router::CreateVirtualRouterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_virtual_router();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateVirtualRouter`.
///
/// <p>Creates a virtual router within a service mesh.</p>
/// <p>Specify a <code>listener</code> for any inbound traffic that your virtual router receives. Create a virtual router for each protocol and port that you need to route. Virtual routers handle traffic for one or more virtual services within your mesh. After you create your virtual router, create and associate routes for your virtual router that direct incoming requests to different virtual nodes.</p>
/// <p>For more information about virtual routers, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_routers.html">Virtual routers</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVirtualRouterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_virtual_router::builders::CreateVirtualRouterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_virtual_router::CreateVirtualRouterOutput,
        crate::operation::create_virtual_router::CreateVirtualRouterError,
    > for CreateVirtualRouterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_virtual_router::CreateVirtualRouterOutput,
            crate::operation::create_virtual_router::CreateVirtualRouterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateVirtualRouterFluentBuilder {
    /// Creates a new `CreateVirtualRouter`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateVirtualRouter as a reference.
    pub fn as_input(&self) -> &crate::operation::create_virtual_router::builders::CreateVirtualRouterInputBuilder {
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
        crate::operation::create_virtual_router::CreateVirtualRouterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_virtual_router::CreateVirtualRouterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_virtual_router::CreateVirtualRouter::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_virtual_router::CreateVirtualRouter::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_virtual_router::CreateVirtualRouterOutput,
        crate::operation::create_virtual_router::CreateVirtualRouterError,
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
    /// <p>The name to use for the virtual router.</p>
    pub fn virtual_router_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.virtual_router_name(input.into());
        self
    }
    /// <p>The name to use for the virtual router.</p>
    pub fn set_virtual_router_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_virtual_router_name(input);
        self
    }
    /// <p>The name to use for the virtual router.</p>
    pub fn get_virtual_router_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_virtual_router_name()
    }
    /// <p>The name of the service mesh to create the virtual router in.</p>
    pub fn mesh_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mesh_name(input.into());
        self
    }
    /// <p>The name of the service mesh to create the virtual router in.</p>
    pub fn set_mesh_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mesh_name(input);
        self
    }
    /// <p>The name of the service mesh to create the virtual router in.</p>
    pub fn get_mesh_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mesh_name()
    }
    /// <p>The virtual router specification to apply.</p>
    pub fn spec(mut self, input: crate::types::VirtualRouterSpec) -> Self {
        self.inner = self.inner.spec(input);
        self
    }
    /// <p>The virtual router specification to apply.</p>
    pub fn set_spec(mut self, input: ::std::option::Option<crate::types::VirtualRouterSpec>) -> Self {
        self.inner = self.inner.set_spec(input);
        self
    }
    /// <p>The virtual router specification to apply.</p>
    pub fn get_spec(&self) -> &::std::option::Option<crate::types::VirtualRouterSpec> {
        self.inner.get_spec()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Optional metadata that you can apply to the virtual router to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn tags(mut self, input: crate::types::TagRef) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Optional metadata that you can apply to the virtual router to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagRef>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Optional metadata that you can apply to the virtual router to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagRef>> {
        self.inner.get_tags()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn mesh_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mesh_owner(input.into());
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn set_mesh_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mesh_owner(input);
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn get_mesh_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mesh_owner()
    }
}
