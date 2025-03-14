// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutGroupInput {
    /// <p>The identifier of the application in which the user and group mapping belongs.</p>
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the index in which you want to map users to their groups.</p>
    pub index_id: ::std::option::Option<::std::string::String>,
    /// <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    /// <p>If you have more than 1000 users and/or sub groups for a single group, you need to provide the path to the S3 file that lists your users and sub groups for a group. Your sub groups can contain more than 1000 users, but the list of sub groups that belong to a group (and/or users) must be no more than 1000.</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p>
    pub data_source_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of the group.</p>
    pub r#type: ::std::option::Option<crate::types::MembershipType>,
    /// <p>A list of users or sub groups that belong to a group. This is for generating Amazon Q chat results only from document a user has access to.</p>
    pub group_members: ::std::option::Option<crate::types::GroupMembers>,
}
impl PutGroupInput {
    /// <p>The identifier of the application in which the user and group mapping belongs.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The identifier of the index in which you want to map users to their groups.</p>
    pub fn index_id(&self) -> ::std::option::Option<&str> {
        self.index_id.as_deref()
    }
    /// <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    /// <p>If you have more than 1000 users and/or sub groups for a single group, you need to provide the path to the S3 file that lists your users and sub groups for a group. Your sub groups can contain more than 1000 users, but the list of sub groups that belong to a group (and/or users) must be no more than 1000.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p>
    pub fn data_source_id(&self) -> ::std::option::Option<&str> {
        self.data_source_id.as_deref()
    }
    /// <p>The type of the group.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::MembershipType> {
        self.r#type.as_ref()
    }
    /// <p>A list of users or sub groups that belong to a group. This is for generating Amazon Q chat results only from document a user has access to.</p>
    pub fn group_members(&self) -> ::std::option::Option<&crate::types::GroupMembers> {
        self.group_members.as_ref()
    }
}
impl PutGroupInput {
    /// Creates a new builder-style object to manufacture [`PutGroupInput`](crate::operation::put_group::PutGroupInput).
    pub fn builder() -> crate::operation::put_group::builders::PutGroupInputBuilder {
        crate::operation::put_group::builders::PutGroupInputBuilder::default()
    }
}

/// A builder for [`PutGroupInput`](crate::operation::put_group::PutGroupInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutGroupInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) index_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) data_source_id: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::MembershipType>,
    pub(crate) group_members: ::std::option::Option<crate::types::GroupMembers>,
}
impl PutGroupInputBuilder {
    /// <p>The identifier of the application in which the user and group mapping belongs.</p>
    /// This field is required.
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the application in which the user and group mapping belongs.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The identifier of the application in which the user and group mapping belongs.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.application_id
    }
    /// <p>The identifier of the index in which you want to map users to their groups.</p>
    /// This field is required.
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the index in which you want to map users to their groups.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_id = input;
        self
    }
    /// <p>The identifier of the index in which you want to map users to their groups.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_id
    }
    /// <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    /// <p>If you have more than 1000 users and/or sub groups for a single group, you need to provide the path to the S3 file that lists your users and sub groups for a group. Your sub groups can contain more than 1000 users, but the list of sub groups that belong to a group (and/or users) must be no more than 1000.</p>
    /// This field is required.
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    /// <p>If you have more than 1000 users and/or sub groups for a single group, you need to provide the path to the S3 file that lists your users and sub groups for a group. Your sub groups can contain more than 1000 users, but the list of sub groups that belong to a group (and/or users) must be no more than 1000.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    /// <p>If you have more than 1000 users and/or sub groups for a single group, you need to provide the path to the S3 file that lists your users and sub groups for a group. Your sub groups can contain more than 1000 users, but the list of sub groups that belong to a group (and/or users) must be no more than 1000.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p>
    pub fn data_source_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_source_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p>
    pub fn set_data_source_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_source_id = input;
        self
    }
    /// <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p>
    pub fn get_data_source_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_source_id
    }
    /// <p>The type of the group.</p>
    /// This field is required.
    pub fn r#type(mut self, input: crate::types::MembershipType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the group.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::MembershipType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of the group.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::MembershipType> {
        &self.r#type
    }
    /// <p>A list of users or sub groups that belong to a group. This is for generating Amazon Q chat results only from document a user has access to.</p>
    /// This field is required.
    pub fn group_members(mut self, input: crate::types::GroupMembers) -> Self {
        self.group_members = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of users or sub groups that belong to a group. This is for generating Amazon Q chat results only from document a user has access to.</p>
    pub fn set_group_members(mut self, input: ::std::option::Option<crate::types::GroupMembers>) -> Self {
        self.group_members = input;
        self
    }
    /// <p>A list of users or sub groups that belong to a group. This is for generating Amazon Q chat results only from document a user has access to.</p>
    pub fn get_group_members(&self) -> &::std::option::Option<crate::types::GroupMembers> {
        &self.group_members
    }
    /// Consumes the builder and constructs a [`PutGroupInput`](crate::operation::put_group::PutGroupInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_group::PutGroupInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_group::PutGroupInput {
            application_id: self.application_id,
            index_id: self.index_id,
            group_name: self.group_name,
            data_source_id: self.data_source_id,
            r#type: self.r#type,
            group_members: self.group_members,
        })
    }
}
