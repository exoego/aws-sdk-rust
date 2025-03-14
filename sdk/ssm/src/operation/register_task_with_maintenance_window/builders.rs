// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_task_with_maintenance_window::_register_task_with_maintenance_window_output::RegisterTaskWithMaintenanceWindowOutputBuilder;

pub use crate::operation::register_task_with_maintenance_window::_register_task_with_maintenance_window_input::RegisterTaskWithMaintenanceWindowInputBuilder;

impl RegisterTaskWithMaintenanceWindowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_task_with_maintenance_window();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterTaskWithMaintenanceWindow`.
///
/// <p>Adds a new task to a maintenance window.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterTaskWithMaintenanceWindowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_task_with_maintenance_window::builders::RegisterTaskWithMaintenanceWindowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowOutput,
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowError,
    > for RegisterTaskWithMaintenanceWindowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowOutput,
            crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterTaskWithMaintenanceWindowFluentBuilder {
    /// Creates a new `RegisterTaskWithMaintenanceWindow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterTaskWithMaintenanceWindow as a reference.
    pub fn as_input(&self) -> &crate::operation::register_task_with_maintenance_window::builders::RegisterTaskWithMaintenanceWindowInputBuilder {
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
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowOutput,
        crate::operation::register_task_with_maintenance_window::RegisterTaskWithMaintenanceWindowError,
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
    /// <p>The ID of the maintenance window the task should be added to.</p>
    pub fn window_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.window_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window the task should be added to.</p>
    pub fn set_window_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_window_id(input);
        self
    }
    /// <p>The ID of the maintenance window the task should be added to.</p>
    pub fn get_window_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_window_id()
    }
    /// Appends an item to `Targets`.
    ///
    /// To override the contents of this collection use [`set_targets`](Self::set_targets).
    ///
    /// <p>The targets (either managed nodes or maintenance window targets).</p><note>
    /// <p>One or more targets must be specified for maintenance window Run Command-type tasks. Depending on the task, targets are optional for other maintenance window task types (Automation, Lambda, and Step Functions). For more information about running tasks that don't specify targets, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">Registering maintenance window tasks without targets</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
    /// </note>
    /// <p>Specify managed nodes using the following format:</p>
    /// <p><code>Key=InstanceIds,Values=<instance-id-1>
    /// ,
    /// <instance-id-2></instance-id-2>
    /// </instance-id-1></code></p>
    /// <p>Specify maintenance window targets using the following format:</p>
    /// <p><code>Key=WindowTargetIds,Values=<window-target-id-1>
    /// ,
    /// <window-target-id-2></window-target-id-2>
    /// </window-target-id-1></code></p>
    pub fn targets(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.targets(input);
        self
    }
    /// <p>The targets (either managed nodes or maintenance window targets).</p><note>
    /// <p>One or more targets must be specified for maintenance window Run Command-type tasks. Depending on the task, targets are optional for other maintenance window task types (Automation, Lambda, and Step Functions). For more information about running tasks that don't specify targets, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">Registering maintenance window tasks without targets</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
    /// </note>
    /// <p>Specify managed nodes using the following format:</p>
    /// <p><code>Key=InstanceIds,Values=<instance-id-1>
    /// ,
    /// <instance-id-2></instance-id-2>
    /// </instance-id-1></code></p>
    /// <p>Specify maintenance window targets using the following format:</p>
    /// <p><code>Key=WindowTargetIds,Values=<window-target-id-1>
    /// ,
    /// <window-target-id-2></window-target-id-2>
    /// </window-target-id-1></code></p>
    pub fn set_targets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Target>>) -> Self {
        self.inner = self.inner.set_targets(input);
        self
    }
    /// <p>The targets (either managed nodes or maintenance window targets).</p><note>
    /// <p>One or more targets must be specified for maintenance window Run Command-type tasks. Depending on the task, targets are optional for other maintenance window task types (Automation, Lambda, and Step Functions). For more information about running tasks that don't specify targets, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">Registering maintenance window tasks without targets</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
    /// </note>
    /// <p>Specify managed nodes using the following format:</p>
    /// <p><code>Key=InstanceIds,Values=<instance-id-1>
    /// ,
    /// <instance-id-2></instance-id-2>
    /// </instance-id-1></code></p>
    /// <p>Specify maintenance window targets using the following format:</p>
    /// <p><code>Key=WindowTargetIds,Values=<window-target-id-1>
    /// ,
    /// <window-target-id-2></window-target-id-2>
    /// </window-target-id-1></code></p>
    pub fn get_targets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Target>> {
        self.inner.get_targets()
    }
    /// <p>The ARN of the task to run.</p>
    pub fn task_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_arn(input.into());
        self
    }
    /// <p>The ARN of the task to run.</p>
    pub fn set_task_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task_arn(input);
        self
    }
    /// <p>The ARN of the task to run.</p>
    pub fn get_task_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_task_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role for Amazon Web Services Systems Manager to assume when running a maintenance window task. If you do not specify a service role ARN, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created when you run <code>RegisterTaskWithMaintenanceWindow</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/using-service-linked-roles.html#slr-permissions">Using service-linked roles for Systems Manager</a> in the in the <i>Amazon Web Services Systems Manager User Guide</i>:</p>
    pub fn service_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role for Amazon Web Services Systems Manager to assume when running a maintenance window task. If you do not specify a service role ARN, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created when you run <code>RegisterTaskWithMaintenanceWindow</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/using-service-linked-roles.html#slr-permissions">Using service-linked roles for Systems Manager</a> in the in the <i>Amazon Web Services Systems Manager User Guide</i>:</p>
    pub fn set_service_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role for Amazon Web Services Systems Manager to assume when running a maintenance window task. If you do not specify a service role ARN, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created when you run <code>RegisterTaskWithMaintenanceWindow</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/using-service-linked-roles.html#slr-permissions">Using service-linked roles for Systems Manager</a> in the in the <i>Amazon Web Services Systems Manager User Guide</i>:</p>
    pub fn get_service_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_role_arn()
    }
    /// <p>The type of task being registered.</p>
    pub fn task_type(mut self, input: crate::types::MaintenanceWindowTaskType) -> Self {
        self.inner = self.inner.task_type(input);
        self
    }
    /// <p>The type of task being registered.</p>
    pub fn set_task_type(mut self, input: ::std::option::Option<crate::types::MaintenanceWindowTaskType>) -> Self {
        self.inner = self.inner.set_task_type(input);
        self
    }
    /// <p>The type of task being registered.</p>
    pub fn get_task_type(&self) -> &::std::option::Option<crate::types::MaintenanceWindowTaskType> {
        self.inner.get_task_type()
    }
    /// Adds a key-value pair to `TaskParameters`.
    ///
    /// To override the contents of this collection use [`set_task_parameters`](Self::set_task_parameters).
    ///
    /// <p>The parameters that should be passed to the task when it is run.</p><note>
    /// <p><code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn task_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::MaintenanceWindowTaskParameterValueExpression,
    ) -> Self {
        self.inner = self.inner.task_parameters(k.into(), v);
        self
    }
    /// <p>The parameters that should be passed to the task when it is run.</p><note>
    /// <p><code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn set_task_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MaintenanceWindowTaskParameterValueExpression>>,
    ) -> Self {
        self.inner = self.inner.set_task_parameters(input);
        self
    }
    /// <p>The parameters that should be passed to the task when it is run.</p><note>
    /// <p><code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn get_task_parameters(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MaintenanceWindowTaskParameterValueExpression>> {
        self.inner.get_task_parameters()
    }
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty.</p>
    pub fn task_invocation_parameters(mut self, input: crate::types::MaintenanceWindowTaskInvocationParameters) -> Self {
        self.inner = self.inner.task_invocation_parameters(input);
        self
    }
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty.</p>
    pub fn set_task_invocation_parameters(mut self, input: ::std::option::Option<crate::types::MaintenanceWindowTaskInvocationParameters>) -> Self {
        self.inner = self.inner.set_task_invocation_parameters(input);
        self
    }
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty.</p>
    pub fn get_task_invocation_parameters(&self) -> &::std::option::Option<crate::types::MaintenanceWindowTaskInvocationParameters> {
        self.inner.get_task_invocation_parameters()
    }
    /// <p>The priority of the task in the maintenance window, the lower the number the higher the priority. Tasks in a maintenance window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.inner = self.inner.priority(input);
        self
    }
    /// <p>The priority of the task in the maintenance window, the lower the number the higher the priority. Tasks in a maintenance window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_priority(input);
        self
    }
    /// <p>The priority of the task in the maintenance window, the lower the number the higher the priority. Tasks in a maintenance window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>
    pub fn get_priority(&self) -> &::std::option::Option<i32> {
        self.inner.get_priority()
    }
    /// <p>The maximum number of targets this task can be run for, in parallel.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn max_concurrency(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.max_concurrency(input.into());
        self
    }
    /// <p>The maximum number of targets this task can be run for, in parallel.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn set_max_concurrency(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_max_concurrency(input);
        self
    }
    /// <p>The maximum number of targets this task can be run for, in parallel.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn get_max_concurrency(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_max_concurrency()
    }
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn max_errors(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.max_errors(input.into());
        self
    }
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn set_max_errors(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_max_errors(input);
        self
    }
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p><note>
    /// <p>Although this element is listed as "Required: No", a value can be omitted only when you are registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless task</a> You must provide a value in all other cases.</p>
    /// <p>For maintenance window tasks without a target specified, you can't supply a value for this option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't affect the running of your task.</p>
    /// </note>
    pub fn get_max_errors(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_max_errors()
    }
    /// <p>A structure containing information about an Amazon Simple Storage Service (Amazon S3) bucket to write managed node-level logs to.</p><note>
    /// <p><code>LoggingInfo</code> has been deprecated. To specify an Amazon Simple Storage Service (Amazon S3) bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Amazon Web Services Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn logging_info(mut self, input: crate::types::LoggingInfo) -> Self {
        self.inner = self.inner.logging_info(input);
        self
    }
    /// <p>A structure containing information about an Amazon Simple Storage Service (Amazon S3) bucket to write managed node-level logs to.</p><note>
    /// <p><code>LoggingInfo</code> has been deprecated. To specify an Amazon Simple Storage Service (Amazon S3) bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Amazon Web Services Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn set_logging_info(mut self, input: ::std::option::Option<crate::types::LoggingInfo>) -> Self {
        self.inner = self.inner.set_logging_info(input);
        self
    }
    /// <p>A structure containing information about an Amazon Simple Storage Service (Amazon S3) bucket to write managed node-level logs to.</p><note>
    /// <p><code>LoggingInfo</code> has been deprecated. To specify an Amazon Simple Storage Service (Amazon S3) bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Amazon Web Services Systems Manager handles these options for the supported maintenance window task types, see <code>MaintenanceWindowTaskInvocationParameters</code>.</p>
    /// </note>
    pub fn get_logging_info(&self) -> &::std::option::Option<crate::types::LoggingInfo> {
        self.inner.get_logging_info()
    }
    /// <p>An optional name for the task.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>An optional name for the task.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>An optional name for the task.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>An optional description for the task.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description for the task.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description for the task.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>User-provided idempotency token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>User-provided idempotency token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>User-provided idempotency token.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached.</p>
    /// <ul>
    /// <li>
    /// <p><code>CONTINUE_TASK</code>: When the cutoff time is reached, any tasks that are running continue. The default value.</p></li>
    /// <li>
    /// <p><code>CANCEL_TASK</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For Automation, Lambda, Step Functions tasks: When the cutoff time is reached, any task invocations that are already running continue, but no new task invocations are started.</p></li>
    /// <li>
    /// <p>For Run Command tasks: When the cutoff time is reached, the system sends a <code>CancelCommand</code> operation that attempts to cancel the command associated with the task. However, there is no guarantee that the command will be terminated and the underlying process stopped.</p></li>
    /// </ul>
    /// <p>The status for tasks that are not completed is <code>TIMED_OUT</code>.</p></li>
    /// </ul>
    pub fn cutoff_behavior(mut self, input: crate::types::MaintenanceWindowTaskCutoffBehavior) -> Self {
        self.inner = self.inner.cutoff_behavior(input);
        self
    }
    /// <p>Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached.</p>
    /// <ul>
    /// <li>
    /// <p><code>CONTINUE_TASK</code>: When the cutoff time is reached, any tasks that are running continue. The default value.</p></li>
    /// <li>
    /// <p><code>CANCEL_TASK</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For Automation, Lambda, Step Functions tasks: When the cutoff time is reached, any task invocations that are already running continue, but no new task invocations are started.</p></li>
    /// <li>
    /// <p>For Run Command tasks: When the cutoff time is reached, the system sends a <code>CancelCommand</code> operation that attempts to cancel the command associated with the task. However, there is no guarantee that the command will be terminated and the underlying process stopped.</p></li>
    /// </ul>
    /// <p>The status for tasks that are not completed is <code>TIMED_OUT</code>.</p></li>
    /// </ul>
    pub fn set_cutoff_behavior(mut self, input: ::std::option::Option<crate::types::MaintenanceWindowTaskCutoffBehavior>) -> Self {
        self.inner = self.inner.set_cutoff_behavior(input);
        self
    }
    /// <p>Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached.</p>
    /// <ul>
    /// <li>
    /// <p><code>CONTINUE_TASK</code>: When the cutoff time is reached, any tasks that are running continue. The default value.</p></li>
    /// <li>
    /// <p><code>CANCEL_TASK</code>:</p>
    /// <ul>
    /// <li>
    /// <p>For Automation, Lambda, Step Functions tasks: When the cutoff time is reached, any task invocations that are already running continue, but no new task invocations are started.</p></li>
    /// <li>
    /// <p>For Run Command tasks: When the cutoff time is reached, the system sends a <code>CancelCommand</code> operation that attempts to cancel the command associated with the task. However, there is no guarantee that the command will be terminated and the underlying process stopped.</p></li>
    /// </ul>
    /// <p>The status for tasks that are not completed is <code>TIMED_OUT</code>.</p></li>
    /// </ul>
    pub fn get_cutoff_behavior(&self) -> &::std::option::Option<crate::types::MaintenanceWindowTaskCutoffBehavior> {
        self.inner.get_cutoff_behavior()
    }
    /// <p>The CloudWatch alarm you want to apply to your maintenance window task.</p>
    pub fn alarm_configuration(mut self, input: crate::types::AlarmConfiguration) -> Self {
        self.inner = self.inner.alarm_configuration(input);
        self
    }
    /// <p>The CloudWatch alarm you want to apply to your maintenance window task.</p>
    pub fn set_alarm_configuration(mut self, input: ::std::option::Option<crate::types::AlarmConfiguration>) -> Self {
        self.inner = self.inner.set_alarm_configuration(input);
        self
    }
    /// <p>The CloudWatch alarm you want to apply to your maintenance window task.</p>
    pub fn get_alarm_configuration(&self) -> &::std::option::Option<crate::types::AlarmConfiguration> {
        self.inner.get_alarm_configuration()
    }
}
