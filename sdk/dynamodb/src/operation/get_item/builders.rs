// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_item::_get_item_output::GetItemOutputBuilder;

pub use crate::operation::get_item::_get_item_input::GetItemInputBuilder;

impl GetItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_item::GetItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetItem`.
///
/// <p>The <code>GetItem</code> operation returns a set of attributes for the item with the given primary key. If there is no matching item, <code>GetItem</code> does not return any data and there will be no <code>Item</code> element in the response.</p>
/// <p><code>GetItem</code> provides an eventually consistent read by default. If your application requires a strongly consistent read, set <code>ConsistentRead</code> to <code>true</code>. Although a strongly consistent read might take more time than an eventually consistent read, it always returns the last updated value.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetItemFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_item::builders::GetItemInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_item::GetItemOutput, crate::operation::get_item::GetItemError>
    for GetItemFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_item::GetItemOutput, crate::operation::get_item::GetItemError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetItemFluentBuilder {
    /// Creates a new `GetItem`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetItem as a reference.
    pub fn as_input(&self) -> &crate::operation::get_item::builders::GetItemInputBuilder {
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
        crate::operation::get_item::GetItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_item::GetItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_item::GetItem::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_item::GetItem::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_item::GetItemOutput, crate::operation::get_item::GetItemError, Self>
    {
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
    /// <p>The name of the table containing the requested item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table containing the requested item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table containing the requested item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// Adds a key-value pair to `Key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p>
    /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn key(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.key(k.into(), v);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p>
    /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p>
    /// <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        self.inner.get_key()
    }
    /// Appends an item to `AttributesToGet`.
    ///
    /// To override the contents of this collection use [`set_attributes_to_get`](Self::set_attributes_to_get).
    ///
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn attributes_to_get(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attributes_to_get(input.into());
        self
    }
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_attributes_to_get(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_attributes_to_get(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_attributes_to_get(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_attributes_to_get()
    }
    /// <p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p>
    pub fn consistent_read(mut self, input: bool) -> Self {
        self.inner = self.inner.consistent_read(input);
        self
    }
    /// <p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p>
    pub fn set_consistent_read(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_consistent_read(input);
        self
    }
    /// <p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p>
    pub fn get_consistent_read(&self) -> &::std::option::Option<bool> {
        self.inner.get_consistent_read()
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<crate::types::ReturnConsumedCapacity>) -> Self {
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<crate::types::ReturnConsumedCapacity> {
        self.inner.get_return_consumed_capacity()
    }
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>
    /// <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn projection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.projection_expression(input.into());
        self
    }
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>
    /// <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_projection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_projection_expression(input);
        self
    }
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p>
    /// <p>If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_projection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_projection_expression()
    }
    /// Adds a key-value pair to `ExpressionAttributeNames`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.expression_attribute_names(k.into(), v.into());
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_names(input);
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expression_attribute_names(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_expression_attribute_names()
    }
}
