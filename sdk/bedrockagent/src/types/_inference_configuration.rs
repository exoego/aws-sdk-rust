// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Configuration for inference in prompt configuration
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InferenceConfiguration {
    /// Controls randomness, higher values increase diversity
    pub temperature: ::std::option::Option<f32>,
    /// Cumulative probability cutoff for token selection
    pub top_p: ::std::option::Option<f32>,
    /// Sample from the k most likely next tokens
    pub top_k: ::std::option::Option<i32>,
    /// Maximum length of output
    pub maximum_length: ::std::option::Option<i32>,
    /// List of stop sequences
    pub stop_sequences: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl InferenceConfiguration {
    /// Controls randomness, higher values increase diversity
    pub fn temperature(&self) -> ::std::option::Option<f32> {
        self.temperature
    }
    /// Cumulative probability cutoff for token selection
    pub fn top_p(&self) -> ::std::option::Option<f32> {
        self.top_p
    }
    /// Sample from the k most likely next tokens
    pub fn top_k(&self) -> ::std::option::Option<i32> {
        self.top_k
    }
    /// Maximum length of output
    pub fn maximum_length(&self) -> ::std::option::Option<i32> {
        self.maximum_length
    }
    /// List of stop sequences
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.stop_sequences.is_none()`.
    pub fn stop_sequences(&self) -> &[::std::string::String] {
        self.stop_sequences.as_deref().unwrap_or_default()
    }
}
impl InferenceConfiguration {
    /// Creates a new builder-style object to manufacture [`InferenceConfiguration`](crate::types::InferenceConfiguration).
    pub fn builder() -> crate::types::builders::InferenceConfigurationBuilder {
        crate::types::builders::InferenceConfigurationBuilder::default()
    }
}

/// A builder for [`InferenceConfiguration`](crate::types::InferenceConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InferenceConfigurationBuilder {
    pub(crate) temperature: ::std::option::Option<f32>,
    pub(crate) top_p: ::std::option::Option<f32>,
    pub(crate) top_k: ::std::option::Option<i32>,
    pub(crate) maximum_length: ::std::option::Option<i32>,
    pub(crate) stop_sequences: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl InferenceConfigurationBuilder {
    /// Controls randomness, higher values increase diversity
    pub fn temperature(mut self, input: f32) -> Self {
        self.temperature = ::std::option::Option::Some(input);
        self
    }
    /// Controls randomness, higher values increase diversity
    pub fn set_temperature(mut self, input: ::std::option::Option<f32>) -> Self {
        self.temperature = input;
        self
    }
    /// Controls randomness, higher values increase diversity
    pub fn get_temperature(&self) -> &::std::option::Option<f32> {
        &self.temperature
    }
    /// Cumulative probability cutoff for token selection
    pub fn top_p(mut self, input: f32) -> Self {
        self.top_p = ::std::option::Option::Some(input);
        self
    }
    /// Cumulative probability cutoff for token selection
    pub fn set_top_p(mut self, input: ::std::option::Option<f32>) -> Self {
        self.top_p = input;
        self
    }
    /// Cumulative probability cutoff for token selection
    pub fn get_top_p(&self) -> &::std::option::Option<f32> {
        &self.top_p
    }
    /// Sample from the k most likely next tokens
    pub fn top_k(mut self, input: i32) -> Self {
        self.top_k = ::std::option::Option::Some(input);
        self
    }
    /// Sample from the k most likely next tokens
    pub fn set_top_k(mut self, input: ::std::option::Option<i32>) -> Self {
        self.top_k = input;
        self
    }
    /// Sample from the k most likely next tokens
    pub fn get_top_k(&self) -> &::std::option::Option<i32> {
        &self.top_k
    }
    /// Maximum length of output
    pub fn maximum_length(mut self, input: i32) -> Self {
        self.maximum_length = ::std::option::Option::Some(input);
        self
    }
    /// Maximum length of output
    pub fn set_maximum_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.maximum_length = input;
        self
    }
    /// Maximum length of output
    pub fn get_maximum_length(&self) -> &::std::option::Option<i32> {
        &self.maximum_length
    }
    /// Appends an item to `stop_sequences`.
    ///
    /// To override the contents of this collection use [`set_stop_sequences`](Self::set_stop_sequences).
    ///
    /// List of stop sequences
    pub fn stop_sequences(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.stop_sequences.unwrap_or_default();
        v.push(input.into());
        self.stop_sequences = ::std::option::Option::Some(v);
        self
    }
    /// List of stop sequences
    pub fn set_stop_sequences(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.stop_sequences = input;
        self
    }
    /// List of stop sequences
    pub fn get_stop_sequences(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.stop_sequences
    }
    /// Consumes the builder and constructs a [`InferenceConfiguration`](crate::types::InferenceConfiguration).
    pub fn build(self) -> crate::types::InferenceConfiguration {
        crate::types::InferenceConfiguration {
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            maximum_length: self.maximum_length,
            stop_sequences: self.stop_sequences,
        }
    }
}
