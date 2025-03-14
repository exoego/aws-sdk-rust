// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::recognize_celebrities::_recognize_celebrities_output::RecognizeCelebritiesOutputBuilder;

pub use crate::operation::recognize_celebrities::_recognize_celebrities_input::RecognizeCelebritiesInputBuilder;

impl RecognizeCelebritiesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::recognize_celebrities::RecognizeCelebritiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::recognize_celebrities::RecognizeCelebritiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.recognize_celebrities();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RecognizeCelebrities`.
///
/// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing celebrities in the Amazon Rekognition Developer Guide.</p>
/// <p><code>RecognizeCelebrities</code> returns the 64 largest faces in the image. It lists the recognized celebrities in the <code>CelebrityFaces</code> array and any unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces aren't among the largest 64 faces in the image.</p>
/// <p>For each celebrity recognized, <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p>
/// <p>Amazon Rekognition doesn't retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the <code>GetCelebrityInfo</code> operation.</p>
/// <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file.</p>
/// <p>For an example, see Recognizing celebrities in an image in the Amazon Rekognition Developer Guide.</p>
/// <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RecognizeCelebritiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::recognize_celebrities::builders::RecognizeCelebritiesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::recognize_celebrities::RecognizeCelebritiesOutput,
        crate::operation::recognize_celebrities::RecognizeCelebritiesError,
    > for RecognizeCelebritiesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::recognize_celebrities::RecognizeCelebritiesOutput,
            crate::operation::recognize_celebrities::RecognizeCelebritiesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RecognizeCelebritiesFluentBuilder {
    /// Creates a new `RecognizeCelebrities`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RecognizeCelebrities as a reference.
    pub fn as_input(&self) -> &crate::operation::recognize_celebrities::builders::RecognizeCelebritiesInputBuilder {
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
        crate::operation::recognize_celebrities::RecognizeCelebritiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::recognize_celebrities::RecognizeCelebritiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::recognize_celebrities::RecognizeCelebrities::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::recognize_celebrities::RecognizeCelebrities::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::recognize_celebrities::RecognizeCelebritiesOutput,
        crate::operation::recognize_celebrities::RecognizeCelebritiesError,
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
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn image(mut self, input: crate::types::Image) -> Self {
        self.inner = self.inner.image(input);
        self
    }
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn set_image(mut self, input: ::std::option::Option<crate::types::Image>) -> Self {
        self.inner = self.inner.set_image(input);
        self
    }
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn get_image(&self) -> &::std::option::Option<crate::types::Image> {
        self.inner.get_image()
    }
}
