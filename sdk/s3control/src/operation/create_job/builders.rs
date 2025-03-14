// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_job::_create_job_output::CreateJobOutputBuilder;

pub use crate::operation::create_job::_create_job_input::CreateJobInputBuilder;

impl CreateJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_job::CreateJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_job::CreateJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateJob`.
///
/// <p>This operation creates an S3 Batch Operations job.</p>
/// <p>You can use S3 Batch Operations to perform large-scale batch actions on Amazon S3 objects. Batch Operations can run a single action on lists of Amazon S3 objects that you specify. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/batch-ops.html">S3 Batch Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <p>For information about permissions required to use the Batch Operations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Granting permissions for S3 Batch Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </dd>
/// </dl>
/// <p></p>
/// <p>Related actions include:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DescribeJob.html">DescribeJob</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_ListJobs.html">ListJobs</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_UpdateJobPriority.html">UpdateJobPriority</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_UpdateJobStatus.html">UpdateJobStatus</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_JobOperation.html">JobOperation</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_job::builders::CreateJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_job::CreateJobOutput, crate::operation::create_job::CreateJobError>
    for CreateJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_job::CreateJobOutput, crate::operation::create_job::CreateJobError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateJobFluentBuilder {
    /// Creates a new `CreateJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_job::builders::CreateJobInputBuilder {
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
        crate::operation::create_job::CreateJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_job::CreateJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_job::CreateJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_job::CreateJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_job::CreateJobOutput,
        crate::operation::create_job::CreateJobError,
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
    /// <p>The Amazon Web Services account ID that creates the job.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that creates the job.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID that creates the job.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>Indicates whether confirmation is required before Amazon S3 runs the job. Confirmation is only required for jobs created through the Amazon S3 console.</p>
    pub fn confirmation_required(mut self, input: bool) -> Self {
        self.inner = self.inner.confirmation_required(input);
        self
    }
    /// <p>Indicates whether confirmation is required before Amazon S3 runs the job. Confirmation is only required for jobs created through the Amazon S3 console.</p>
    pub fn set_confirmation_required(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_confirmation_required(input);
        self
    }
    /// <p>Indicates whether confirmation is required before Amazon S3 runs the job. Confirmation is only required for jobs created through the Amazon S3 console.</p>
    pub fn get_confirmation_required(&self) -> &::std::option::Option<bool> {
        self.inner.get_confirmation_required()
    }
    /// <p>The action that you want this job to perform on every object listed in the manifest. For more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-actions.html">Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn operation(mut self, input: crate::types::JobOperation) -> Self {
        self.inner = self.inner.operation(input);
        self
    }
    /// <p>The action that you want this job to perform on every object listed in the manifest. For more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-actions.html">Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_operation(mut self, input: ::std::option::Option<crate::types::JobOperation>) -> Self {
        self.inner = self.inner.set_operation(input);
        self
    }
    /// <p>The action that you want this job to perform on every object listed in the manifest. For more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-actions.html">Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_operation(&self) -> &::std::option::Option<crate::types::JobOperation> {
        self.inner.get_operation()
    }
    /// <p>Configuration parameters for the optional job-completion report.</p>
    pub fn report(mut self, input: crate::types::JobReport) -> Self {
        self.inner = self.inner.report(input);
        self
    }
    /// <p>Configuration parameters for the optional job-completion report.</p>
    pub fn set_report(mut self, input: ::std::option::Option<crate::types::JobReport>) -> Self {
        self.inner = self.inner.set_report(input);
        self
    }
    /// <p>Configuration parameters for the optional job-completion report.</p>
    pub fn get_report(&self) -> &::std::option::Option<crate::types::JobReport> {
        self.inner.get_report()
    }
    /// <p>An idempotency token to ensure that you don't accidentally submit the same request twice. You can use any string up to the maximum length.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>An idempotency token to ensure that you don't accidentally submit the same request twice. You can use any string up to the maximum length.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>An idempotency token to ensure that you don't accidentally submit the same request twice. You can use any string up to the maximum length.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>Configuration parameters for the manifest.</p>
    pub fn manifest(mut self, input: crate::types::JobManifest) -> Self {
        self.inner = self.inner.manifest(input);
        self
    }
    /// <p>Configuration parameters for the manifest.</p>
    pub fn set_manifest(mut self, input: ::std::option::Option<crate::types::JobManifest>) -> Self {
        self.inner = self.inner.set_manifest(input);
        self
    }
    /// <p>Configuration parameters for the manifest.</p>
    pub fn get_manifest(&self) -> &::std::option::Option<crate::types::JobManifest> {
        self.inner.get_manifest()
    }
    /// <p>A description for this job. You can use any string within the permitted length. Descriptions don't need to be unique and can be used for multiple jobs.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for this job. You can use any string within the permitted length. Descriptions don't need to be unique and can be used for multiple jobs.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for this job. You can use any string within the permitted length. Descriptions don't need to be unique and can be used for multiple jobs.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The numerical priority for this job. Higher numbers indicate higher priority.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.inner = self.inner.priority(input);
        self
    }
    /// <p>The numerical priority for this job. Higher numbers indicate higher priority.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_priority(input);
        self
    }
    /// <p>The numerical priority for this job. Higher numbers indicate higher priority.</p>
    pub fn get_priority(&self) -> &::std::option::Option<i32> {
        self.inner.get_priority()
    }
    /// <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will use to run this job's action on every object in the manifest.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will use to run this job's action on every object in the manifest.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will use to run this job's action on every object in the manifest.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A set of tags to associate with the S3 Batch Operations job. This is an optional parameter.</p>
    pub fn tags(mut self, input: crate::types::S3Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A set of tags to associate with the S3 Batch Operations job. This is an optional parameter.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::S3Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A set of tags to associate with the S3 Batch Operations job. This is an optional parameter.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::S3Tag>> {
        self.inner.get_tags()
    }
    /// <p>The attribute container for the ManifestGenerator details. Jobs must be created with either a manifest file or a ManifestGenerator, but not both.</p>
    pub fn manifest_generator(mut self, input: crate::types::JobManifestGenerator) -> Self {
        self.inner = self.inner.manifest_generator(input);
        self
    }
    /// <p>The attribute container for the ManifestGenerator details. Jobs must be created with either a manifest file or a ManifestGenerator, but not both.</p>
    pub fn set_manifest_generator(mut self, input: ::std::option::Option<crate::types::JobManifestGenerator>) -> Self {
        self.inner = self.inner.set_manifest_generator(input);
        self
    }
    /// <p>The attribute container for the ManifestGenerator details. Jobs must be created with either a manifest file or a ManifestGenerator, but not both.</p>
    pub fn get_manifest_generator(&self) -> &::std::option::Option<crate::types::JobManifestGenerator> {
        self.inner.get_manifest_generator()
    }
}
