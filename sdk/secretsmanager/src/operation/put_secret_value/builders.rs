// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_secret_value::_put_secret_value_output::PutSecretValueOutputBuilder;

pub use crate::operation::put_secret_value::_put_secret_value_input::PutSecretValueInputBuilder;

impl PutSecretValueInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_secret_value::PutSecretValueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_secret_value::PutSecretValueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_secret_value();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutSecretValue`.
///
/// <p>Creates a new version with a new encrypted secret value and attaches it to the secret. The version can contain a new <code>SecretString</code> value or a new <code>SecretBinary</code> value.</p>
/// <p>We recommend you avoid calling <code>PutSecretValue</code> at a sustained rate of more than once every 10 minutes. When you update the secret value, Secrets Manager creates a new version of the secret. Secrets Manager removes outdated versions when there are more than 100, but it does not remove versions created less than 24 hours ago. If you call <code>PutSecretValue</code> more than once every 10 minutes, you create more versions than Secrets Manager removes, and you will reach the quota for secret versions.</p>
/// <p>You can specify the staging labels to attach to the new version in <code>VersionStages</code>. If you don't include <code>VersionStages</code>, then Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this version. If this operation creates the first version for the secret, then Secrets Manager automatically attaches the staging label <code>AWSCURRENT</code> to it. If this operation moves the staging label <code>AWSCURRENT</code> from another version to this version, then Secrets Manager also automatically moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>
/// <p>This operation is idempotent. If you call this operation with a <code>ClientRequestToken</code> that matches an existing version's VersionId, and you specify the same secret data, the operation succeeds but does nothing. However, if the secret data is different, then the operation fails because you can't modify an existing version; you can only create new ones.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters except <code>SecretBinary</code> or <code>SecretString</code> because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p><b>Required permissions: </b> <code>secretsmanager:PutSecretValue</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutSecretValueFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_secret_value::builders::PutSecretValueInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_secret_value::PutSecretValueOutput,
        crate::operation::put_secret_value::PutSecretValueError,
    > for PutSecretValueFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_secret_value::PutSecretValueOutput,
            crate::operation::put_secret_value::PutSecretValueError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutSecretValueFluentBuilder {
    /// Creates a new `PutSecretValue`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutSecretValue as a reference.
    pub fn as_input(&self) -> &crate::operation::put_secret_value::builders::PutSecretValueInputBuilder {
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
        crate::operation::put_secret_value::PutSecretValueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_secret_value::PutSecretValueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_secret_value::PutSecretValue::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_secret_value::PutSecretValue::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_secret_value::PutSecretValueOutput,
        crate::operation::put_secret_value::PutSecretValueError,
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
    /// <p>The ARN or name of the secret to add a new version to.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    /// <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret to add a new version to.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    /// <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// <p>The ARN or name of the secret to add a new version to.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    /// <p>If the secret doesn't already exist, use <code>CreateSecret</code> instead.</p>
    pub fn get_secret_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_id()
    }
    /// <p>A unique identifier for the new version of the secret.</p><note>
    /// <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request.</p>
    /// </note>
    /// <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>
    /// <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret.</p>
    /// <ul>
    /// <li>
    /// <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and that version's <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in the request then the request is ignored. The operation is idempotent.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and the version of the <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request, then the request fails because you can't modify a secret version. You can only create new versions to store new secret values.</p></li>
    /// </ul>
    /// <p>This value becomes the <code>VersionId</code> of the new version.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A unique identifier for the new version of the secret.</p><note>
    /// <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request.</p>
    /// </note>
    /// <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>
    /// <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret.</p>
    /// <ul>
    /// <li>
    /// <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and that version's <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in the request then the request is ignored. The operation is idempotent.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and the version of the <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request, then the request fails because you can't modify a secret version. You can only create new versions to store new secret values.</p></li>
    /// </ul>
    /// <p>This value becomes the <code>VersionId</code> of the new version.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>A unique identifier for the new version of the secret.</p><note>
    /// <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDKs to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes it as the value for this parameter in the request.</p>
    /// </note>
    /// <p>If you generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> and include it in the request.</p>
    /// <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret.</p>
    /// <ul>
    /// <li>
    /// <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and that version's <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in the request then the request is ignored. The operation is idempotent.</p></li>
    /// <li>
    /// <p>If a version with this value already exists and the version of the <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request, then the request fails because you can't modify a secret version. You can only create new versions to store new secret values.</p></li>
    /// </ul>
    /// <p>This value becomes the <code>VersionId</code> of the new version.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>The binary data to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then pass the contents of the file as a parameter.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>You can't access this value from the Secrets Manager console.</p>
    pub fn secret_binary(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.secret_binary(input);
        self
    }
    /// <p>The binary data to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then pass the contents of the file as a parameter.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>You can't access this value from the Secrets Manager console.</p>
    pub fn set_secret_binary(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_secret_binary(input);
        self
    }
    /// <p>The binary data to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then pass the contents of the file as a parameter.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>You can't access this value from the Secrets Manager console.</p>
    pub fn get_secret_binary(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_secret_binary()
    }
    /// <p>The text to encrypt and store in the new version of the secret.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>We recommend you create the secret string as JSON key/value pairs, as shown in the example.</p>
    pub fn secret_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_string(input.into());
        self
    }
    /// <p>The text to encrypt and store in the new version of the secret.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>We recommend you create the secret string as JSON key/value pairs, as shown in the example.</p>
    pub fn set_secret_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_string(input);
        self
    }
    /// <p>The text to encrypt and store in the new version of the secret.</p>
    /// <p>You must include <code>SecretBinary</code> or <code>SecretString</code>, but not both.</p>
    /// <p>We recommend you create the secret string as JSON key/value pairs, as shown in the example.</p>
    pub fn get_secret_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_string()
    }
    /// Appends an item to `VersionStages`.
    ///
    /// To override the contents of this collection use [`set_version_stages`](Self::set_version_stages).
    ///
    /// <p>A list of staging labels to attach to this version of the secret. Secrets Manager uses staging labels to track versions of a secret through the rotation process.</p>
    /// <p>If you specify a staging label that's already associated with a different version of the same secret, then Secrets Manager removes the label from the other version and attaches it to this version. If you specify <code>AWSCURRENT</code>, and it is already attached to another version, then Secrets Manager also moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>
    /// <p>If you don't include <code>VersionStages</code>, then Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this version.</p>
    pub fn version_stages(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_stages(input.into());
        self
    }
    /// <p>A list of staging labels to attach to this version of the secret. Secrets Manager uses staging labels to track versions of a secret through the rotation process.</p>
    /// <p>If you specify a staging label that's already associated with a different version of the same secret, then Secrets Manager removes the label from the other version and attaches it to this version. If you specify <code>AWSCURRENT</code>, and it is already attached to another version, then Secrets Manager also moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>
    /// <p>If you don't include <code>VersionStages</code>, then Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this version.</p>
    pub fn set_version_stages(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_version_stages(input);
        self
    }
    /// <p>A list of staging labels to attach to this version of the secret. Secrets Manager uses staging labels to track versions of a secret through the rotation process.</p>
    /// <p>If you specify a staging label that's already associated with a different version of the same secret, then Secrets Manager removes the label from the other version and attaches it to this version. If you specify <code>AWSCURRENT</code>, and it is already attached to another version, then Secrets Manager also moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p>
    /// <p>If you don't include <code>VersionStages</code>, then Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this version.</p>
    pub fn get_version_stages(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_version_stages()
    }
}
