// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_site_rack_physical_properties::_update_site_rack_physical_properties_output::UpdateSiteRackPhysicalPropertiesOutputBuilder;

pub use crate::operation::update_site_rack_physical_properties::_update_site_rack_physical_properties_input::UpdateSiteRackPhysicalPropertiesInputBuilder;

impl UpdateSiteRackPhysicalPropertiesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_site_rack_physical_properties();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSiteRackPhysicalProperties`.
///
/// <p>Update the physical and logistical details for a rack at a site. For more information about hardware requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#checklist">Network readiness checklist</a> in the Amazon Web Services Outposts User Guide.</p>
/// <p>To update a rack at a site with an order of <code>IN_PROGRESS</code>, you must wait for the order to complete or cancel the order.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSiteRackPhysicalPropertiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_site_rack_physical_properties::builders::UpdateSiteRackPhysicalPropertiesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput,
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesError,
    > for UpdateSiteRackPhysicalPropertiesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput,
            crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSiteRackPhysicalPropertiesFluentBuilder {
    /// Creates a new `UpdateSiteRackPhysicalProperties`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSiteRackPhysicalProperties as a reference.
    pub fn as_input(&self) -> &crate::operation::update_site_rack_physical_properties::builders::UpdateSiteRackPhysicalPropertiesInputBuilder {
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
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalProperties::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalProperties::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput,
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesError,
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
    /// <p>The ID or the Amazon Resource Name (ARN) of the site.</p>
    pub fn site_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.site_id(input.into());
        self
    }
    /// <p>The ID or the Amazon Resource Name (ARN) of the site.</p>
    pub fn set_site_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_site_id(input);
        self
    }
    /// <p>The ID or the Amazon Resource Name (ARN) of the site.</p>
    pub fn get_site_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_site_id()
    }
    /// <p>The power draw, in kVA, available at the hardware placement position for the rack.</p>
    pub fn power_draw_kva(mut self, input: crate::types::PowerDrawKva) -> Self {
        self.inner = self.inner.power_draw_kva(input);
        self
    }
    /// <p>The power draw, in kVA, available at the hardware placement position for the rack.</p>
    pub fn set_power_draw_kva(mut self, input: ::std::option::Option<crate::types::PowerDrawKva>) -> Self {
        self.inner = self.inner.set_power_draw_kva(input);
        self
    }
    /// <p>The power draw, in kVA, available at the hardware placement position for the rack.</p>
    pub fn get_power_draw_kva(&self) -> &::std::option::Option<crate::types::PowerDrawKva> {
        self.inner.get_power_draw_kva()
    }
    /// <p>The power option that you can provide for hardware.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed: 200 V to 277 V, 50 Hz or 60 Hz</p></li>
    /// <li>
    /// <p>Three-phase AC feed: 346 V to 480 V, 50 Hz or 60 Hz</p></li>
    /// </ul>
    pub fn power_phase(mut self, input: crate::types::PowerPhase) -> Self {
        self.inner = self.inner.power_phase(input);
        self
    }
    /// <p>The power option that you can provide for hardware.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed: 200 V to 277 V, 50 Hz or 60 Hz</p></li>
    /// <li>
    /// <p>Three-phase AC feed: 346 V to 480 V, 50 Hz or 60 Hz</p></li>
    /// </ul>
    pub fn set_power_phase(mut self, input: ::std::option::Option<crate::types::PowerPhase>) -> Self {
        self.inner = self.inner.set_power_phase(input);
        self
    }
    /// <p>The power option that you can provide for hardware.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed: 200 V to 277 V, 50 Hz or 60 Hz</p></li>
    /// <li>
    /// <p>Three-phase AC feed: 346 V to 480 V, 50 Hz or 60 Hz</p></li>
    /// </ul>
    pub fn get_power_phase(&self) -> &::std::option::Option<crate::types::PowerPhase> {
        self.inner.get_power_phase()
    }
    /// <p>The power connector that Amazon Web Services should plan to provide for connections to the hardware. Note the correlation between <code>PowerPhase</code> and <code>PowerConnector</code>.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>L6-30P</b> – (common in US); 30A; single phase</p></li>
    /// <li>
    /// <p><b>IEC309 (blue)</b> – P+N+E, 6hr; 32 A; single phase</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Three-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>AH530P7W (red)</b> – 3P+N+E, 7hr; 30A; three phase</p></li>
    /// <li>
    /// <p><b>AH532P6W (red)</b> – 3P+N+E, 6hr; 32A; three phase</p></li>
    /// </ul></li>
    /// </ul>
    pub fn power_connector(mut self, input: crate::types::PowerConnector) -> Self {
        self.inner = self.inner.power_connector(input);
        self
    }
    /// <p>The power connector that Amazon Web Services should plan to provide for connections to the hardware. Note the correlation between <code>PowerPhase</code> and <code>PowerConnector</code>.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>L6-30P</b> – (common in US); 30A; single phase</p></li>
    /// <li>
    /// <p><b>IEC309 (blue)</b> – P+N+E, 6hr; 32 A; single phase</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Three-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>AH530P7W (red)</b> – 3P+N+E, 7hr; 30A; three phase</p></li>
    /// <li>
    /// <p><b>AH532P6W (red)</b> – 3P+N+E, 6hr; 32A; three phase</p></li>
    /// </ul></li>
    /// </ul>
    pub fn set_power_connector(mut self, input: ::std::option::Option<crate::types::PowerConnector>) -> Self {
        self.inner = self.inner.set_power_connector(input);
        self
    }
    /// <p>The power connector that Amazon Web Services should plan to provide for connections to the hardware. Note the correlation between <code>PowerPhase</code> and <code>PowerConnector</code>.</p>
    /// <ul>
    /// <li>
    /// <p>Single-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>L6-30P</b> – (common in US); 30A; single phase</p></li>
    /// <li>
    /// <p><b>IEC309 (blue)</b> – P+N+E, 6hr; 32 A; single phase</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Three-phase AC feed</p>
    /// <ul>
    /// <li>
    /// <p><b>AH530P7W (red)</b> – 3P+N+E, 7hr; 30A; three phase</p></li>
    /// <li>
    /// <p><b>AH532P6W (red)</b> – 3P+N+E, 6hr; 32A; three phase</p></li>
    /// </ul></li>
    /// </ul>
    pub fn get_power_connector(&self) -> &::std::option::Option<crate::types::PowerConnector> {
        self.inner.get_power_connector()
    }
    /// <p>Indicates whether the power feed comes above or below the rack.</p>
    pub fn power_feed_drop(mut self, input: crate::types::PowerFeedDrop) -> Self {
        self.inner = self.inner.power_feed_drop(input);
        self
    }
    /// <p>Indicates whether the power feed comes above or below the rack.</p>
    pub fn set_power_feed_drop(mut self, input: ::std::option::Option<crate::types::PowerFeedDrop>) -> Self {
        self.inner = self.inner.set_power_feed_drop(input);
        self
    }
    /// <p>Indicates whether the power feed comes above or below the rack.</p>
    pub fn get_power_feed_drop(&self) -> &::std::option::Option<crate::types::PowerFeedDrop> {
        self.inner.get_power_feed_drop()
    }
    /// <p>The uplink speed the rack should support for the connection to the Region.</p>
    pub fn uplink_gbps(mut self, input: crate::types::UplinkGbps) -> Self {
        self.inner = self.inner.uplink_gbps(input);
        self
    }
    /// <p>The uplink speed the rack should support for the connection to the Region.</p>
    pub fn set_uplink_gbps(mut self, input: ::std::option::Option<crate::types::UplinkGbps>) -> Self {
        self.inner = self.inner.set_uplink_gbps(input);
        self
    }
    /// <p>The uplink speed the rack should support for the connection to the Region.</p>
    pub fn get_uplink_gbps(&self) -> &::std::option::Option<crate::types::UplinkGbps> {
        self.inner.get_uplink_gbps()
    }
    /// <p>Racks come with two Outpost network devices. Depending on the supported uplink speed at the site, the Outpost network devices provide a variable number of uplinks. Specify the number of uplinks for each Outpost network device that you intend to use to connect the rack to your network. Note the correlation between <code>UplinkGbps</code> and <code>UplinkCount</code>.</p>
    /// <ul>
    /// <li>
    /// <p>1Gbps - Uplinks available: 1, 2, 4, 6, 8</p></li>
    /// <li>
    /// <p>10Gbps - Uplinks available: 1, 2, 4, 8, 12, 16</p></li>
    /// <li>
    /// <p>40 and 100 Gbps- Uplinks available: 1, 2, 4</p></li>
    /// </ul>
    pub fn uplink_count(mut self, input: crate::types::UplinkCount) -> Self {
        self.inner = self.inner.uplink_count(input);
        self
    }
    /// <p>Racks come with two Outpost network devices. Depending on the supported uplink speed at the site, the Outpost network devices provide a variable number of uplinks. Specify the number of uplinks for each Outpost network device that you intend to use to connect the rack to your network. Note the correlation between <code>UplinkGbps</code> and <code>UplinkCount</code>.</p>
    /// <ul>
    /// <li>
    /// <p>1Gbps - Uplinks available: 1, 2, 4, 6, 8</p></li>
    /// <li>
    /// <p>10Gbps - Uplinks available: 1, 2, 4, 8, 12, 16</p></li>
    /// <li>
    /// <p>40 and 100 Gbps- Uplinks available: 1, 2, 4</p></li>
    /// </ul>
    pub fn set_uplink_count(mut self, input: ::std::option::Option<crate::types::UplinkCount>) -> Self {
        self.inner = self.inner.set_uplink_count(input);
        self
    }
    /// <p>Racks come with two Outpost network devices. Depending on the supported uplink speed at the site, the Outpost network devices provide a variable number of uplinks. Specify the number of uplinks for each Outpost network device that you intend to use to connect the rack to your network. Note the correlation between <code>UplinkGbps</code> and <code>UplinkCount</code>.</p>
    /// <ul>
    /// <li>
    /// <p>1Gbps - Uplinks available: 1, 2, 4, 6, 8</p></li>
    /// <li>
    /// <p>10Gbps - Uplinks available: 1, 2, 4, 8, 12, 16</p></li>
    /// <li>
    /// <p>40 and 100 Gbps- Uplinks available: 1, 2, 4</p></li>
    /// </ul>
    pub fn get_uplink_count(&self) -> &::std::option::Option<crate::types::UplinkCount> {
        self.inner.get_uplink_count()
    }
    /// <p>The type of fiber that you will use to attach the Outpost to your network.</p>
    pub fn fiber_optic_cable_type(mut self, input: crate::types::FiberOpticCableType) -> Self {
        self.inner = self.inner.fiber_optic_cable_type(input);
        self
    }
    /// <p>The type of fiber that you will use to attach the Outpost to your network.</p>
    pub fn set_fiber_optic_cable_type(mut self, input: ::std::option::Option<crate::types::FiberOpticCableType>) -> Self {
        self.inner = self.inner.set_fiber_optic_cable_type(input);
        self
    }
    /// <p>The type of fiber that you will use to attach the Outpost to your network.</p>
    pub fn get_fiber_optic_cable_type(&self) -> &::std::option::Option<crate::types::FiberOpticCableType> {
        self.inner.get_fiber_optic_cable_type()
    }
    /// <p>The type of optical standard that you will use to attach the Outpost to your network. This field is dependent on uplink speed, fiber type, and distance to the upstream device. For more information about networking requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#facility-networking">Network</a> in the Amazon Web Services Outposts User Guide.</p>
    /// <ul>
    /// <li>
    /// <p><code>OPTIC_10GBASE_SR</code>: 10GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_IR</code>: 10GBASE-IR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_LR</code>: 10GBASE-LR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_SR</code>: 40GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_ESR</code>: 40GBASE-ESR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_IR4_LR4L</code>: 40GBASE-IR (LR4L)</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_LR4</code>: 40GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_SR4</code>: 100GBASE-SR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_CWDM4</code>: 100GBASE-CWDM4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_LR4</code>: 100GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100G_PSM4_MSA</code>: 100G PSM4 MSA</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_LX</code>: 1000Base-LX</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_SX</code> : 1000Base-SX</p></li>
    /// </ul>
    pub fn optical_standard(mut self, input: crate::types::OpticalStandard) -> Self {
        self.inner = self.inner.optical_standard(input);
        self
    }
    /// <p>The type of optical standard that you will use to attach the Outpost to your network. This field is dependent on uplink speed, fiber type, and distance to the upstream device. For more information about networking requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#facility-networking">Network</a> in the Amazon Web Services Outposts User Guide.</p>
    /// <ul>
    /// <li>
    /// <p><code>OPTIC_10GBASE_SR</code>: 10GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_IR</code>: 10GBASE-IR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_LR</code>: 10GBASE-LR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_SR</code>: 40GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_ESR</code>: 40GBASE-ESR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_IR4_LR4L</code>: 40GBASE-IR (LR4L)</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_LR4</code>: 40GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_SR4</code>: 100GBASE-SR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_CWDM4</code>: 100GBASE-CWDM4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_LR4</code>: 100GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100G_PSM4_MSA</code>: 100G PSM4 MSA</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_LX</code>: 1000Base-LX</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_SX</code> : 1000Base-SX</p></li>
    /// </ul>
    pub fn set_optical_standard(mut self, input: ::std::option::Option<crate::types::OpticalStandard>) -> Self {
        self.inner = self.inner.set_optical_standard(input);
        self
    }
    /// <p>The type of optical standard that you will use to attach the Outpost to your network. This field is dependent on uplink speed, fiber type, and distance to the upstream device. For more information about networking requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#facility-networking">Network</a> in the Amazon Web Services Outposts User Guide.</p>
    /// <ul>
    /// <li>
    /// <p><code>OPTIC_10GBASE_SR</code>: 10GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_IR</code>: 10GBASE-IR</p></li>
    /// <li>
    /// <p><code>OPTIC_10GBASE_LR</code>: 10GBASE-LR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_SR</code>: 40GBASE-SR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_ESR</code>: 40GBASE-ESR</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_IR4_LR4L</code>: 40GBASE-IR (LR4L)</p></li>
    /// <li>
    /// <p><code>OPTIC_40GBASE_LR4</code>: 40GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_SR4</code>: 100GBASE-SR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_CWDM4</code>: 100GBASE-CWDM4</p></li>
    /// <li>
    /// <p><code>OPTIC_100GBASE_LR4</code>: 100GBASE-LR4</p></li>
    /// <li>
    /// <p><code>OPTIC_100G_PSM4_MSA</code>: 100G PSM4 MSA</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_LX</code>: 1000Base-LX</p></li>
    /// <li>
    /// <p><code>OPTIC_1000BASE_SX</code> : 1000Base-SX</p></li>
    /// </ul>
    pub fn get_optical_standard(&self) -> &::std::option::Option<crate::types::OpticalStandard> {
        self.inner.get_optical_standard()
    }
    /// <p>The maximum rack weight that this site can support. <code>NO_LIMIT</code> is over 2000lbs.</p>
    pub fn maximum_supported_weight_lbs(mut self, input: crate::types::MaximumSupportedWeightLbs) -> Self {
        self.inner = self.inner.maximum_supported_weight_lbs(input);
        self
    }
    /// <p>The maximum rack weight that this site can support. <code>NO_LIMIT</code> is over 2000lbs.</p>
    pub fn set_maximum_supported_weight_lbs(mut self, input: ::std::option::Option<crate::types::MaximumSupportedWeightLbs>) -> Self {
        self.inner = self.inner.set_maximum_supported_weight_lbs(input);
        self
    }
    /// <p>The maximum rack weight that this site can support. <code>NO_LIMIT</code> is over 2000lbs.</p>
    pub fn get_maximum_supported_weight_lbs(&self) -> &::std::option::Option<crate::types::MaximumSupportedWeightLbs> {
        self.inner.get_maximum_supported_weight_lbs()
    }
}
