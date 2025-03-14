// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::analyze_document::_analyze_document_output::AnalyzeDocumentOutputBuilder;

pub use crate::operation::analyze_document::_analyze_document_input::AnalyzeDocumentInputBuilder;

impl AnalyzeDocumentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::analyze_document::AnalyzeDocumentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::analyze_document::AnalyzeDocumentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.analyze_document();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AnalyzeDocument`.
///
/// <p>Analyzes an input document for relationships between detected items.</p>
/// <p>The types of information returned are as follows:</p>
/// <ul>
/// <li>
/// <p>Form data (key-value pairs). The related information is returned in two <code>Block</code> objects, each of type <code>KEY_VALUE_SET</code>: a KEY <code>Block</code> object and a VALUE <code>Block</code> object. For example, <i>Name: Ana Silva Carolina</i> contains a key and value. <i>Name:</i> is the key. <i>Ana Silva Carolina</i> is the value.</p></li>
/// <li>
/// <p>Table and table cell data. A TABLE <code>Block</code> object contains information about a detected table. A CELL <code>Block</code> object is returned for each cell in a table.</p></li>
/// <li>
/// <p>Lines and words of text. A LINE <code>Block</code> object contains one or more WORD <code>Block</code> objects. All lines and words that are detected in the document are returned (including text that doesn't have a relationship with the value of <code>FeatureTypes</code>).</p></li>
/// <li>
/// <p>Signatures. A SIGNATURE <code>Block</code> object contains the location information of a signature in a document. If used in conjunction with forms or tables, a signature can be given a Key-Value pairing or be detected in the cell of a table.</p></li>
/// <li>
/// <p>Query. A QUERY Block object contains the query text, alias and link to the associated Query results block object.</p></li>
/// <li>
/// <p>Query Result. A QUERY_RESULT Block object contains the answer to the query and an ID that connects it to the query asked. This Block also contains a confidence score.</p></li>
/// </ul>
/// <p>Selection elements such as check boxes and option buttons (radio buttons) can be detected in form data and in tables. A SELECTION_ELEMENT <code>Block</code> object contains information about a selection element, including the selection status.</p>
/// <p>You can choose which type of analysis to perform by specifying the <code>FeatureTypes</code> list.</p>
/// <p>The output is returned in a list of <code>Block</code> objects.</p>
/// <p><code>AnalyzeDocument</code> is a synchronous operation. To analyze documents asynchronously, use <code>StartDocumentAnalysis</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AnalyzeDocumentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::analyze_document::builders::AnalyzeDocumentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::analyze_document::AnalyzeDocumentOutput,
        crate::operation::analyze_document::AnalyzeDocumentError,
    > for AnalyzeDocumentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::analyze_document::AnalyzeDocumentOutput,
            crate::operation::analyze_document::AnalyzeDocumentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AnalyzeDocumentFluentBuilder {
    /// Creates a new `AnalyzeDocument`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AnalyzeDocument as a reference.
    pub fn as_input(&self) -> &crate::operation::analyze_document::builders::AnalyzeDocumentInputBuilder {
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
        crate::operation::analyze_document::AnalyzeDocumentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::analyze_document::AnalyzeDocumentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::analyze_document::AnalyzeDocument::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::analyze_document::AnalyzeDocument::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::analyze_document::AnalyzeDocumentOutput,
        crate::operation::analyze_document::AnalyzeDocumentError,
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
    /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Textract operations, you can't pass image bytes. The document must be an image in JPEG, PNG, PDF, or TIFF format.</p>
    /// <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes that are passed using the <code>Bytes</code> field.</p>
    pub fn document(mut self, input: crate::types::Document) -> Self {
        self.inner = self.inner.document(input);
        self
    }
    /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Textract operations, you can't pass image bytes. The document must be an image in JPEG, PNG, PDF, or TIFF format.</p>
    /// <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes that are passed using the <code>Bytes</code> field.</p>
    pub fn set_document(mut self, input: ::std::option::Option<crate::types::Document>) -> Self {
        self.inner = self.inner.set_document(input);
        self
    }
    /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Textract operations, you can't pass image bytes. The document must be an image in JPEG, PNG, PDF, or TIFF format.</p>
    /// <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes that are passed using the <code>Bytes</code> field.</p>
    pub fn get_document(&self) -> &::std::option::Option<crate::types::Document> {
        self.inner.get_document()
    }
    /// Appends an item to `FeatureTypes`.
    ///
    /// To override the contents of this collection use [`set_feature_types`](Self::set_feature_types).
    ///
    /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information about the tables that are detected in the input document. Add FORMS to return detected form data. Add SIGNATURES to return the locations of detected signatures. Add LAYOUT to the list to return information about the layout of the document. All lines and words detected in the document are included in the response (including text that isn't related to the value of <code>FeatureTypes</code>).</p>
    pub fn feature_types(mut self, input: crate::types::FeatureType) -> Self {
        self.inner = self.inner.feature_types(input);
        self
    }
    /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information about the tables that are detected in the input document. Add FORMS to return detected form data. Add SIGNATURES to return the locations of detected signatures. Add LAYOUT to the list to return information about the layout of the document. All lines and words detected in the document are included in the response (including text that isn't related to the value of <code>FeatureTypes</code>).</p>
    pub fn set_feature_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FeatureType>>) -> Self {
        self.inner = self.inner.set_feature_types(input);
        self
    }
    /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information about the tables that are detected in the input document. Add FORMS to return detected form data. Add SIGNATURES to return the locations of detected signatures. Add LAYOUT to the list to return information about the layout of the document. All lines and words detected in the document are included in the response (including text that isn't related to the value of <code>FeatureTypes</code>).</p>
    pub fn get_feature_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FeatureType>> {
        self.inner.get_feature_types()
    }
    /// <p>Sets the configuration for the human in the loop workflow for analyzing documents.</p>
    pub fn human_loop_config(mut self, input: crate::types::HumanLoopConfig) -> Self {
        self.inner = self.inner.human_loop_config(input);
        self
    }
    /// <p>Sets the configuration for the human in the loop workflow for analyzing documents.</p>
    pub fn set_human_loop_config(mut self, input: ::std::option::Option<crate::types::HumanLoopConfig>) -> Self {
        self.inner = self.inner.set_human_loop_config(input);
        self
    }
    /// <p>Sets the configuration for the human in the loop workflow for analyzing documents.</p>
    pub fn get_human_loop_config(&self) -> &::std::option::Option<crate::types::HumanLoopConfig> {
        self.inner.get_human_loop_config()
    }
    /// <p>Contains Queries and the alias for those Queries, as determined by the input.</p>
    pub fn queries_config(mut self, input: crate::types::QueriesConfig) -> Self {
        self.inner = self.inner.queries_config(input);
        self
    }
    /// <p>Contains Queries and the alias for those Queries, as determined by the input.</p>
    pub fn set_queries_config(mut self, input: ::std::option::Option<crate::types::QueriesConfig>) -> Self {
        self.inner = self.inner.set_queries_config(input);
        self
    }
    /// <p>Contains Queries and the alias for those Queries, as determined by the input.</p>
    pub fn get_queries_config(&self) -> &::std::option::Option<crate::types::QueriesConfig> {
        self.inner.get_queries_config()
    }
    /// <p>Specifies the adapter to be used when analyzing a document.</p>
    pub fn adapters_config(mut self, input: crate::types::AdaptersConfig) -> Self {
        self.inner = self.inner.adapters_config(input);
        self
    }
    /// <p>Specifies the adapter to be used when analyzing a document.</p>
    pub fn set_adapters_config(mut self, input: ::std::option::Option<crate::types::AdaptersConfig>) -> Self {
        self.inner = self.inner.set_adapters_config(input);
        self
    }
    /// <p>Specifies the adapter to be used when analyzing a document.</p>
    pub fn get_adapters_config(&self) -> &::std::option::Option<crate::types::AdaptersConfig> {
        self.inner.get_adapters_config()
    }
}
