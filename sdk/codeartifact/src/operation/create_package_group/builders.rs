// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_package_group::_create_package_group_output::CreatePackageGroupOutputBuilder;

pub use crate::operation::create_package_group::_create_package_group_input::CreatePackageGroupInputBuilder;

impl CreatePackageGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_package_group::CreatePackageGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_package_group::CreatePackageGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_package_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePackageGroup`.
///
/// <p>Creates a package group. For more information about creating package groups, including example CLI commands, see <a href="https://docs.aws.amazon.com/codeartifact/latest/ug/create-package-group.html">Create a package group</a> in the <i>CodeArtifact User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePackageGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_package_group::builders::CreatePackageGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_package_group::CreatePackageGroupOutput,
        crate::operation::create_package_group::CreatePackageGroupError,
    > for CreatePackageGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_package_group::CreatePackageGroupOutput,
            crate::operation::create_package_group::CreatePackageGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePackageGroupFluentBuilder {
    /// Creates a new `CreatePackageGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePackageGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_package_group::builders::CreatePackageGroupInputBuilder {
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
        crate::operation::create_package_group::CreatePackageGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_package_group::CreatePackageGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_package_group::CreatePackageGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_package_group::CreatePackageGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_package_group::CreatePackageGroupOutput,
        crate::operation::create_package_group::CreatePackageGroupError,
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
    /// <p>The name of the domain in which you want to create a package group.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The name of the domain in which you want to create a package group.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The name of the domain in which you want to create a package group.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
    /// <p>The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces.</p>
    pub fn domain_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_owner(input.into());
        self
    }
    /// <p>The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces.</p>
    pub fn set_domain_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_owner(input);
        self
    }
    /// <p>The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces.</p>
    pub fn get_domain_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_owner()
    }
    /// <p>The pattern of the package group to create. The pattern is also the identifier of the package group.</p>
    pub fn package_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_group(input.into());
        self
    }
    /// <p>The pattern of the package group to create. The pattern is also the identifier of the package group.</p>
    pub fn set_package_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_group(input);
        self
    }
    /// <p>The pattern of the package group to create. The pattern is also the identifier of the package group.</p>
    pub fn get_package_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_group()
    }
    /// <p>The contact information for the created package group.</p>
    pub fn contact_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_info(input.into());
        self
    }
    /// <p>The contact information for the created package group.</p>
    pub fn set_contact_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_info(input);
        self
    }
    /// <p>The contact information for the created package group.</p>
    pub fn get_contact_info(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_info()
    }
    /// <p>A description of the package group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the package group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the package group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tag key-value pairs for the package group.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>One or more tag key-value pairs for the package group.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>One or more tag key-value pairs for the package group.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
