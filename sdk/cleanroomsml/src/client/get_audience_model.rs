// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAudienceModel`](crate::operation::get_audience_model::builders::GetAudienceModelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`audience_model_arn(impl Into<String>)`](crate::operation::get_audience_model::builders::GetAudienceModelFluentBuilder::audience_model_arn) / [`set_audience_model_arn(Option<String>)`](crate::operation::get_audience_model::builders::GetAudienceModelFluentBuilder::set_audience_model_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the audience model that you are interested in.</p><br>
    /// - On success, responds with [`GetAudienceModelOutput`](crate::operation::get_audience_model::GetAudienceModelOutput) with field(s):
    ///   - [`create_time(DateTime)`](crate::operation::get_audience_model::GetAudienceModelOutput::create_time): <p>The time at which the audience model was created.</p>
    ///   - [`update_time(DateTime)`](crate::operation::get_audience_model::GetAudienceModelOutput::update_time): <p>The most recent time at which the audience model was updated.</p>
    ///   - [`training_data_start_time(Option<DateTime>)`](crate::operation::get_audience_model::GetAudienceModelOutput::training_data_start_time): <p>The start date specified for the training window.</p>
    ///   - [`training_data_end_time(Option<DateTime>)`](crate::operation::get_audience_model::GetAudienceModelOutput::training_data_end_time): <p>The end date specified for the training window.</p>
    ///   - [`audience_model_arn(String)`](crate::operation::get_audience_model::GetAudienceModelOutput::audience_model_arn): <p>The Amazon Resource Name (ARN) of the audience model.</p>
    ///   - [`name(String)`](crate::operation::get_audience_model::GetAudienceModelOutput::name): <p>The name of the audience model.</p>
    ///   - [`training_dataset_arn(String)`](crate::operation::get_audience_model::GetAudienceModelOutput::training_dataset_arn): <p>The Amazon Resource Name (ARN) of the training dataset that was used for this audience model.</p>
    ///   - [`status(AudienceModelStatus)`](crate::operation::get_audience_model::GetAudienceModelOutput::status): <p>The status of the audience model.</p>
    ///   - [`status_details(Option<StatusDetails>)`](crate::operation::get_audience_model::GetAudienceModelOutput::status_details): <p>Details about the status of the audience model.</p>
    ///   - [`metrics(Option<Vec::<AudienceModelMetric>>)`](crate::operation::get_audience_model::GetAudienceModelOutput::metrics): <p>Accuracy metrics for the model.</p>
    ///   - [`kms_key_arn(Option<String>)`](crate::operation::get_audience_model::GetAudienceModelOutput::kms_key_arn): <p>The KMS key ARN used for the audience model.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_audience_model::GetAudienceModelOutput::tags): <p>The tags that are assigned to the audience model.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_audience_model::GetAudienceModelOutput::description): <p>The description of the audience model.</p>
    /// - On failure, responds with [`SdkError<GetAudienceModelError>`](crate::operation::get_audience_model::GetAudienceModelError)
    pub fn get_audience_model(&self) -> crate::operation::get_audience_model::builders::GetAudienceModelFluentBuilder {
        crate::operation::get_audience_model::builders::GetAudienceModelFluentBuilder::new(self.handle.clone())
    }
}
