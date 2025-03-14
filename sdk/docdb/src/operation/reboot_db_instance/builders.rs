// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reboot_db_instance::_reboot_db_instance_output::RebootDbInstanceOutputBuilder;

pub use crate::operation::reboot_db_instance::_reboot_db_instance_input::RebootDbInstanceInputBuilder;

impl RebootDbInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::reboot_db_instance::RebootDbInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reboot_db_instance::RebootDBInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.reboot_db_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RebootDBInstance`.
///
/// <p>You might need to reboot your instance, usually for maintenance reasons. For example, if you make certain changes, or if you change the cluster parameter group that is associated with the instance, you must reboot the instance for the changes to take effect.</p>
/// <p>Rebooting an instance restarts the database engine service. Rebooting an instance results in a momentary outage, during which the instance status is set to <i>rebooting</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RebootDBInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::reboot_db_instance::builders::RebootDbInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::reboot_db_instance::RebootDbInstanceOutput,
        crate::operation::reboot_db_instance::RebootDBInstanceError,
    > for RebootDBInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::reboot_db_instance::RebootDbInstanceOutput,
            crate::operation::reboot_db_instance::RebootDBInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RebootDBInstanceFluentBuilder {
    /// Creates a new `RebootDBInstance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RebootDBInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::reboot_db_instance::builders::RebootDbInstanceInputBuilder {
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
        crate::operation::reboot_db_instance::RebootDbInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reboot_db_instance::RebootDBInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::reboot_db_instance::RebootDBInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::reboot_db_instance::RebootDBInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::reboot_db_instance::RebootDbInstanceOutput,
        crate::operation::reboot_db_instance::RebootDBInstanceError,
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
    /// <p>The instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn db_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn set_db_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
    /// <p>The instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn get_db_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_instance_identifier()
    }
    /// <p>When <code>true</code>, the reboot is conducted through a Multi-AZ failover.</p>
    /// <p>Constraint: You can't specify <code>true</code> if the instance is not configured for Multi-AZ.</p>
    pub fn force_failover(mut self, input: bool) -> Self {
        self.inner = self.inner.force_failover(input);
        self
    }
    /// <p>When <code>true</code>, the reboot is conducted through a Multi-AZ failover.</p>
    /// <p>Constraint: You can't specify <code>true</code> if the instance is not configured for Multi-AZ.</p>
    pub fn set_force_failover(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_failover(input);
        self
    }
    /// <p>When <code>true</code>, the reboot is conducted through a Multi-AZ failover.</p>
    /// <p>Constraint: You can't specify <code>true</code> if the instance is not configured for Multi-AZ.</p>
    pub fn get_force_failover(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_failover()
    }
}
