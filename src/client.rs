#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`Action`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Action {
        ///Command executed in the Action.
        pub command: ::std::string::String,
        ///Error message for the Action if an error occurred, otherwise null.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub error: ::std::option::Option<ActionError>,
        ///Point in time when the Action was finished (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format). Only set if the Action is finished otherwise null.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub finished: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///ID of the [Action](#description/actions).
        pub id: ::std::num::NonZeroU64,
        ///Progress of the Action in percent.
        pub progress: i32,
        ///Resources the Action relates to.
        pub resources: ::std::vec::Vec<ActionResourcesItem>,
        ///Point in time when the Action was started (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub started: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Status of the Action.
        pub status: ActionStatus,
    }
    ///Error message for the Action if an error occurred, otherwise null.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionError {
        ///Fixed error code for machines.
        pub code: ::std::string::String,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ActionListResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionListResponse {
        pub actions: ::std::vec::Vec<Action>,
    }
    ///`ActionListResponseWithMeta`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionListResponseWithMeta {
        pub actions: ::std::vec::Vec<Action>,
        pub meta: ListMeta,
    }
    /**Action will only be returned if the [Primary IP](#tag/primary-ips) is getting assigned.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionNullable {
        ///Command executed in the Action.
        pub command: ::std::string::String,
        ///Error message for the Action if an error occurred, otherwise null.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub error: ::std::option::Option<ActionNullableError>,
        ///Point in time when the Action was finished (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format). Only set if the Action is finished otherwise null.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub finished: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///ID of the [Action](#description/actions).
        pub id: ::std::num::NonZeroU64,
        ///Progress of the Action in percent.
        pub progress: i32,
        ///Resources the Action relates to.
        pub resources: ::std::vec::Vec<ActionNullableResourcesItem>,
        ///Point in time when the Action was started (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub started: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Status of the Action.
        pub status: ActionNullableStatus,
    }
    ///Error message for the Action if an error occurred, otherwise null.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionNullableError {
        ///Fixed error code for machines.
        pub code: ::std::string::String,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ActionNullableResourcesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionNullableResourcesItem {
        ///ID of the Resource.
        pub id: ::std::num::NonZeroU64,
        ///Type of the Resource.
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ActionNullableStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ActionNullableStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ActionNullableStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ActionNullableStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ActionNullableStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ActionResourcesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionResourcesItem {
        ///ID of the Resource.
        pub id: ::std::num::NonZeroU64,
        ///Type of the Resource.
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    ///`ActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ActionResponse {
        pub action: Action,
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ActionStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ActionStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ActionStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ActionStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ActionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`AddNetworkRouteResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddNetworkRouteResponse {
        pub error: AddNetworkRouteResponseError,
    }
    ///`AddNetworkRouteResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddNetworkRouteResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AddNetworkSubnetResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddNetworkSubnetResponse {
        pub error: AddNetworkSubnetResponseError,
    }
    ///`AddNetworkSubnetResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddNetworkSubnetResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AddRouteRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddRouteRequest {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`AddServerToPlacementGroupResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddServerToPlacementGroupResponse {
        pub error: AddServerToPlacementGroupResponseError,
    }
    ///`AddServerToPlacementGroupResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddServerToPlacementGroupResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AddSubnetRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddSubnetRequest {
        /**IP range of the subnet.

        Uses CIDR notation.

        Must be a subnet of the parent [Networks](#tag/networks) `ip_range`.

        Must not overlap with any other subnets or with any destinations in routes.

        Minimum network size is /30. We highly recommend that you pick a larger subnet with a /24 netmask.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: AddSubnetRequestType,
        /**ID of the robot vSwitch.

        Must be supplied if the subnet is of type `vswitch`.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AddSubnetRequestType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for AddSubnetRequestType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for AddSubnetRequestType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AddSubnetRequestType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AddSubnetRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`AddToPlacementGroupRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddToPlacementGroupRequest {
        ///ID of Placement Group the Server should be added to.
        pub placement_group: i64,
    }
    ///`ApplyFirewallToResourcesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApplyFirewallToResourcesResponse {
        pub error: ApplyFirewallToResourcesResponseError,
    }
    ///`ApplyFirewallToResourcesResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApplyFirewallToResourcesResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ApplyToResourcesRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ApplyToResourcesRequest {
        /**Resources to apply the [Firewall](#tag/firewalls) to.

        Extends existing resources.
        */
        pub apply_to: ::std::vec::Vec<FirewallResource>,
    }
    ///`AssignPrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AssignPrimaryIpResponse {
        pub error: AssignPrimaryIpResponseError,
    }
    ///`AssignPrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AssignPrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AttachServerIsoBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachServerIsoBody {
        ///ID or name of ISO to attach to the Server as listed in GET `/isos`.
        pub iso: ::std::string::String,
    }
    ///`AttachServerIsoResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachServerIsoResponse {
        pub error: AttachServerIsoResponseError,
    }
    ///`AttachServerIsoResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachServerIsoResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AttachServerToNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachServerToNetworkResponse {
        pub error: AttachServerToNetworkResponseError,
    }
    ///`AttachServerToNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachServerToNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`AttachToNetworkRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AttachToNetworkRequest {
        ///Additional IPs to be assigned to this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///IP to request to be assigned to this Server; if you do not provide this then you will be auto assigned an IP address.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::string::String>,
        /**IP range in CIDR block notation of the subnet to attach to.

        This allows for auto assigning an IP address for a specific subnet.
        Providing `ip` that is not part of `ip_range` will result in an error.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        ///ID of an existing network to attach the Server to.
        pub network: i64,
    }
    ///`ChangeImageProtectionBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ChangeImageProtectionBody {
        ///If true, prevents the snapshot from being deleted.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub delete: ::std::option::Option<bool>,
    }
    ///`ChangeImageProtectionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeImageProtectionResponse {
        pub error: ChangeImageProtectionResponseError,
    }
    ///`ChangeImageProtectionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeImageProtectionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeIpRangeRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ChangeIpRangeRequest {
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.

        Must span all included subnets. Must be one of the private IPv4 ranges of RFC1918.

        Minimum network size is /24. We highly recommend that you pick a larger [Network](#tag/networks) with a /16 netmask.
        */
        pub ip_range: ::std::string::String,
    }
    ///`ChangeNetworkIpRangeResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeNetworkIpRangeResponse {
        pub error: ChangeNetworkIpRangeResponseError,
    }
    ///`ChangeNetworkIpRangeResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeNetworkIpRangeResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeNetworkProtectionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeNetworkProtectionResponse {
        pub error: ChangeNetworkProtectionResponseError,
    }
    ///`ChangeNetworkProtectionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeNetworkProtectionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangePrimaryIpDnsPtrResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangePrimaryIpDnsPtrResponse {
        pub error: ChangePrimaryIpDnsPtrResponseError,
    }
    ///`ChangePrimaryIpDnsPtrResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangePrimaryIpDnsPtrResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangePrimaryIpProtectionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangePrimaryIpProtectionResponse {
        pub error: ChangePrimaryIpProtectionResponseError,
    }
    ///`ChangePrimaryIpProtectionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangePrimaryIpProtectionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeProtectionRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct ChangeProtectionRequest {
        /**Delete protection setting.

        If true, prevents the [Network](#tag/networks) from being deleted.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub delete: ::std::option::Option<bool>,
    }
    ///`ChangeServerAliasIpsBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerAliasIpsBody {
        ///New alias IPs to set for this Server.
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///ID of an existing Network already attached to the Server.
        pub network: i64,
    }
    ///`ChangeServerAliasIpsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerAliasIpsResponse {
        pub error: ChangeServerAliasIpsResponseError,
    }
    ///`ChangeServerAliasIpsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerAliasIpsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeServerDnsPtrResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerDnsPtrResponse {
        pub error: ChangeServerDnsPtrResponseError,
    }
    ///`ChangeServerDnsPtrResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerDnsPtrResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeServerProtectionBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ChangeServerProtectionBody {
        ///If true, prevents the Server from being deleted (currently delete and rebuild attribute needs to have the same value).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub delete: ::std::option::Option<bool>,
        ///If true, prevents the Server from being rebuilt (currently delete and rebuild attribute needs to have the same value).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rebuild: ::std::option::Option<bool>,
    }
    ///`ChangeServerProtectionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerProtectionResponse {
        pub error: ChangeServerProtectionResponseError,
    }
    ///`ChangeServerProtectionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerProtectionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ChangeServerTypeBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerTypeBody {
        ///ID or name of Server type the Server should migrate to.
        pub server_type: ::std::string::String,
        ///If false, do not upgrade the disk (this allows downgrading the Server type later).
        pub upgrade_disk: bool,
    }
    ///`ChangeServerTypeResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerTypeResponse {
        pub error: ChangeServerTypeResponseError,
    }
    ///`ChangeServerTypeResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangeServerTypeResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateFirewallRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateFirewallRequest {
        /**Resources to apply the [Firewall](#tag/firewalls) to.

        Resources added directly are taking precedence over those added via a [Label Selector](#description/label-selector).
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub apply_to: ::std::vec::Vec<FirewallResource>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**Name of the [Firewall](#tag/firewalls).

        Must be unique per Project.
        */
        pub name: CreateFirewallRequestName,
        /**Array of rules.

        Rules are limited to 50 entries per [Firewall](#tag/firewalls) and [500 effective rules](https://docs.hetzner.com/cloud/firewalls/overview#limits).
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rules: ::std::vec::Vec<Rule>,
    }
    /**Name of the [Firewall](#tag/firewalls).

    Must be unique per Project.
    */
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateFirewallRequestName(::std::string::String);
    impl ::std::ops::Deref for CreateFirewallRequestName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreateFirewallRequestName> for ::std::string::String {
        fn from(value: CreateFirewallRequestName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreateFirewallRequestName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateFirewallRequestName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateFirewallRequestName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateFirewallRequestName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`CreateFirewallResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreateFirewallResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub actions: ::std::vec::Vec<Action>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub firewall: ::std::option::Option<FirewallResponse>,
    }
    ///`CreateFirewallResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateFirewallResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateImageRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateImageRequest {
        ///Description of the Image, will be auto-generated if not set.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Type of Image to create.
        #[serde(rename = "type", default = "defaults::create_image_request_type")]
        pub type_: CreateImageRequestType,
    }
    impl ::std::default::Default for CreateImageRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                labels: Default::default(),
                type_: defaults::create_image_request_type(),
            }
        }
    }
    ///Type of Image to create.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateImageRequestType {
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for CreateImageRequestType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for CreateImageRequestType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateImageRequestType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for CreateImageRequestType {
        fn default() -> Self {
            CreateImageRequestType::Snapshot
        }
    }
    ///`CreateNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreateNetworkResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<CreateNetworkResponseNetwork>,
    }
    ///`CreateNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateNetworkResponseNetwork`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateNetworkResponseNetwork {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch connection.
        pub expose_routes_to_vswitch: bool,
        ///ID of the [Network](#tag/networks).
        pub id: i64,
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.
        */
        pub ip_range: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Array of IDs of [Load Balancers](#tag/load-balancers) attached to this [Network](#tag/networks).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        ///Name of the [Network](#tag/networks).
        pub name: CreateNetworkResponseNetworkName,
        pub protection: CreateNetworkResponseNetworkProtection,
        ///Array of routes set in this [Network](#tag/networks).
        pub routes: ::std::vec::Vec<CreateNetworkResponseNetworkRoutesItem>,
        ///Array of IDs of [Servers](#tag/servers) attached to this [Network](#tag/networks).
        pub servers: ::std::vec::Vec<i64>,
        ///List of subnets allocated in this [Network](#tag/networks).
        pub subnets: ::std::vec::Vec<CreateNetworkResponseNetworkSubnetsItem>,
    }
    ///Name of the [Network](#tag/networks).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateNetworkResponseNetworkName(::std::string::String);
    impl ::std::ops::Deref for CreateNetworkResponseNetworkName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreateNetworkResponseNetworkName> for ::std::string::String {
        fn from(value: CreateNetworkResponseNetworkName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreateNetworkResponseNetworkName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateNetworkResponseNetworkName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateNetworkResponseNetworkProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///`CreateNetworkResponseNetworkRoutesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateNetworkResponseNetworkRoutesItem {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`CreateNetworkResponseNetworkSubnetsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateNetworkResponseNetworkSubnetsItem {
        /**Gateway for [Servers](#tag/servers) attached to this subnet.

        For subnets of type `server` this is always the first IP of the subnets IP range.
        */
        pub gateway: ::std::string::String,
        /**IP range of the subnet.

        Uses CIDR notation.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` property it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: CreateNetworkResponseNetworkSubnetsItemType,
        ///ID of the robot vSwitch if the subnet is of type `vswitch`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateNetworkResponseNetworkSubnetsItemType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for CreateNetworkResponseNetworkSubnetsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for CreateNetworkResponseNetworkSubnetsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateNetworkResponseNetworkSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateNetworkResponseNetworkSubnetsItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CreatePrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreatePrimaryIpResponse {
        /**Action will only be returned if the [Primary IP](#tag/primary-ips) is getting assigned.
         */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<ActionNullable>,
        pub primary_ip: PrimaryIp,
    }
    ///`CreatePrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreatePrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateServerImageResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct CreateServerImageResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<Action>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<CreateServerImageResponseImage>,
    }
    ///`CreateServerImageResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerImageResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateServerImageResponseImage`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerImageResponseImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: CreateServerImageResponseImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<CreateServerImageResponseImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: CreateServerImageResponseImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: CreateServerImageResponseImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: CreateServerImageResponseImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: CreateServerImageResponseImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerImageResponseImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for CreateServerImageResponseImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerImageResponseImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerImageResponseImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerImageResponseImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for CreateServerImageResponseImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerImageResponseImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerImageResponseImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerImageResponseImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for CreateServerImageResponseImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerImageResponseImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerImageResponseImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for CreateServerImageResponseImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerImageResponseImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CreateServerRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerRequest {
        ///Auto-mount Volumes after attach.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub automount: ::std::option::Option<bool>,
        ///Firewalls which should be applied on the Server's public network interface at creation time.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub firewalls: ::std::vec::Vec<CreateServerRequestFirewallsItem>,
        ///ID or name of the Image the Server is created from.
        pub image: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///ID or name of the Location to create the Server in.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<::std::string::String>,
        ///Name of the Server to create (must be unique per Project and a valid hostname as per RFC 1123).
        pub name: ::std::string::String,
        ///Network IDs which should be attached to the Server private network interface at the creation time.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub networks: ::std::vec::Vec<i64>,
        ///ID of the Placement Group the Server should be in.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub placement_group: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub public_net: ::std::option::Option<CreateServerRequestPublicNet>,
        ///ID or name of the Server type this Server should be created with.
        pub server_type: ::std::string::String,
        ///SSH key IDs (`integer`) or names (`string`) which should be injected into the Server at creation time.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ssh_keys: ::std::vec::Vec<::std::string::String>,
        ///This automatically triggers a [Power on a Server-Server Action](#tag/server-actions/poweron_server) after the creation is finished and is returned in the `next_actions` response object.
        #[serde(default = "defaults::default_bool::<true>")]
        pub start_after_create: bool,
        ///Cloud-Init user data to use during Server creation. This field is limited to 32KiB.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub user_data: ::std::option::Option<::std::string::String>,
        ///Volume IDs which should be attached to the Server at the creation time. Volumes must be in the same Location.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub volumes: ::std::vec::Vec<i64>,
    }
    ///`CreateServerRequestFirewallsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerRequestFirewallsItem {
        ///ID of the Firewall.
        pub firewall: i64,
    }
    ///Public Network options.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerRequestPublicNet {
        ///Attach an IPv4 on the public NIC. If false, no IPv4 address will be attached.
        #[serde(default = "defaults::default_bool::<true>")]
        pub enable_ipv4: bool,
        ///Attach an IPv6 on the public NIC. If false, no IPv6 address will be attached.
        #[serde(default = "defaults::default_bool::<true>")]
        pub enable_ipv6: bool,
        ///ID of the ipv4 Primary IP to use. If omitted and enable_ipv4 is true, a new ipv4 Primary IP will automatically be created.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ipv4: ::std::option::Option<i64>,
        ///ID of the ipv6 Primary IP to use. If omitted and enable_ipv6 is true, a new ipv6 Primary IP will automatically be created.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ipv6: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for CreateServerRequestPublicNet {
        fn default() -> Self {
            Self {
                enable_ipv4: defaults::default_bool::<true>(),
                enable_ipv6: defaults::default_bool::<true>(),
                ipv4: Default::default(),
                ipv6: Default::default(),
            }
        }
    }
    ///`CreateServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponse {
        pub action: Action,
        pub next_actions: ::std::vec::Vec<Action>,
        ///Root password when no SSH keys have been specified.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub root_password: ::std::option::Option<::std::string::String>,
        pub server: CreateServerResponseServer,
    }
    ///`CreateServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateServerResponseServer`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServer {
        ///Time window (UTC) in which the backup will run, or null if the backups are not enabled.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub backup_window: ::std::option::Option<::std::string::String>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
        ///Image the server is based on.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image: ::std::option::Option<CreateServerResponseServerImage>,
        ///Free Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub included_traffic: ::std::option::Option<i64>,
        ///Inbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ingoing_traffic: ::std::option::Option<i64>,
        ///ISO Image that is attached to this Server. Null if no ISO is attached.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub iso: ::std::option::Option<CreateServerResponseServerIso>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Load Balancer IDs assigned to the server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        pub location: CreateServerResponseServerLocation,
        ///True if Server has been locked and is not available to user.
        pub locked: bool,
        ///Name of the Server (must be unique per Project and a valid hostname as per RFC 1123).
        pub name: ::std::string::String,
        ///Outbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub outgoing_traffic: ::std::option::Option<i64>,
        ///The placement group the server is assigned to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub placement_group: ::std::option::Option<PlacementGroupNullable>,
        ///Size of the primary Disk.
        pub primary_disk_size: f64,
        ///Private networks information.
        pub private_net: ::std::vec::Vec<CreateServerResponseServerPrivateNetItem>,
        pub protection: CreateServerResponseServerProtection,
        pub public_net: CreateServerResponseServerPublicNet,
        ///True if rescue mode is enabled. Server will then boot into rescue system on next reboot.
        pub rescue_enabled: bool,
        pub server_type: CreateServerResponseServerServerType,
        ///Status of the Server.
        pub status: CreateServerResponseServerStatus,
        ///IDs of Volumes assigned to this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub volumes: ::std::vec::Vec<i64>,
    }
    ///Image the server is based on.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: CreateServerResponseServerImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<CreateServerResponseServerImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: CreateServerResponseServerImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: CreateServerResponseServerImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: CreateServerResponseServerImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: CreateServerResponseServerImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for CreateServerResponseServerImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateServerResponseServerImageArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for CreateServerResponseServerImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for CreateServerResponseServerImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for CreateServerResponseServerImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///ISO Image that is attached to this Server. Null if no ISO is attached.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerIso {
        /**CPU architecture compatible with the ISO.

        Null indicates no restriction on the architecture (wildcard).
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub architecture: ::std::option::Option<CreateServerResponseServerIsoArchitecture>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the ISO.
        pub description: ::std::string::String,
        ///ID of the [ISO](#tag/isos).
        pub id: ::std::num::NonZeroU64,
        ///Unique identifier of the ISO. Only set for public ISOs.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Type of the ISO.
        #[serde(
            rename = "type",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub type_: ::std::option::Option<CreateServerResponseServerIsoType>,
    }
    /**CPU architecture compatible with the ISO.

    Null indicates no restriction on the architecture (wildcard).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerIsoArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for CreateServerResponseServerIsoArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerIsoArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the ISO.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerIsoType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }
    impl ::std::fmt::Display for CreateServerResponseServerIsoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => f.write_str("public"),
                Self::Private => f.write_str("private"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerIsoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Location this Resource is located at.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: CreateServerResponseServerLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: CreateServerResponseServerLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateServerResponseServerLocationName(::std::string::String);
    impl ::std::ops::Deref for CreateServerResponseServerLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreateServerResponseServerLocationName> for ::std::string::String {
        fn from(value: CreateServerResponseServerLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateServerResponseServerLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateServerResponseServerLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for CreateServerResponseServerLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreateServerResponseServerLocationNetworkZone> for ::std::string::String {
        fn from(value: CreateServerResponseServerLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateServerResponseServerLocationNetworkZone
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateServerResponseServerLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`CreateServerResponseServerPrivateNetItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct CreateServerResponseServerPrivateNetItem {
        ///Additional IP addresses of the server on the network.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///The server IP address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::string::String>,
        ///The server MAC address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub mac_address: ::std::option::Option<::std::string::String>,
        ///The Network ID the server is attached to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<i64>,
    }
    ///Protection configuration for the Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerProtection {
        ///If true, prevents the Server from being deleted.
        pub delete: bool,
        ///If true, prevents the Server from being rebuilt.
        pub rebuild: bool,
    }
    ///Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerPublicNet {
        ///Firewalls applied to the public network interface of this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub firewalls: ::std::vec::Vec<ServerPublicNetFirewall>,
        ///IDs of Floating IPs assigned to this Server.
        pub floating_ips: ::std::vec::Vec<i64>,
        ///IP address (v4) and its reverse DNS entry of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv4: ::std::option::Option<CreateServerResponseServerPublicNetIpv4>,
        ///IPv6 network assigned to this Server and its reverse DNS entry.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv6: ::std::option::Option<CreateServerResponseServerPublicNetIpv6>,
    }
    ///IP address (v4) and its reverse DNS entry of this Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerPublicNetIpv4 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entry for the IPv4 addresses of this Server.
        pub dns_ptr: ::std::string::String,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v4) of this Server.
        pub ip: ::std::string::String,
    }
    ///IPv6 network assigned to this Server and its reverse DNS entry.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerPublicNetIpv6 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entries for the IPv6 addresses of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr: ::std::option::Option<
            ::std::vec::Vec<CreateServerResponseServerPublicNetIpv6DnsPtrItem>,
        >,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v6) of this Server.
        pub ip: ::std::string::String,
    }
    ///`CreateServerResponseServerPublicNetIpv6DnsPtrItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerPublicNetIpv6DnsPtrItem {
        ///DNS pointer for the specific IP address.
        pub dns_ptr: ::std::string::String,
        ///Single IPv6 address of this Server for which the reverse DNS entry has been set up.
        pub ip: ::std::string::String,
    }
    ///`CreateServerResponseServerServerType`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerType {
        /**CPU architecture of the Server Type.
         */
        pub architecture: CreateServerResponseServerServerTypeArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: CreateServerResponseServerServerTypeCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<CreateServerResponseServerServerTypeLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<CreateServerResponseServerServerTypePricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: CreateServerResponseServerServerTypeStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerServerTypeArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for CreateServerResponseServerServerTypeArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerServerTypeArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerServerTypeArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateServerResponseServerServerTypeArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerServerTypeCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for CreateServerResponseServerServerTypeCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerServerTypeCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateServerResponseServerServerTypeCpuType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerTypeLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`CreateServerResponseServerServerTypePricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerTypePricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: CreateServerResponseServerServerTypePricesItemPriceHourly,
        pub price_monthly: CreateServerResponseServerServerTypePricesItemPriceMonthly,
        pub price_per_tb_traffic: CreateServerResponseServerServerTypePricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerTypePricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerTypePricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServerResponseServerServerTypePricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerServerTypeStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for CreateServerResponseServerServerTypeStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerServerTypeStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CreateServerResponseServerServerTypeStorageType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateServerResponseServerStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "migrating")]
        Migrating,
        #[serde(rename = "rebuilding")]
        Rebuilding,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for CreateServerResponseServerStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Initializing => f.write_str("initializing"),
                Self::Starting => f.write_str("starting"),
                Self::Stopping => f.write_str("stopping"),
                Self::Off => f.write_str("off"),
                Self::Deleting => f.write_str("deleting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Rebuilding => f.write_str("rebuilding"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for CreateServerResponseServerStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "initializing" => Ok(Self::Initializing),
                "starting" => Ok(Self::Starting),
                "stopping" => Ok(Self::Stopping),
                "off" => Ok(Self::Off),
                "deleting" => Ok(Self::Deleting),
                "migrating" => Ok(Self::Migrating),
                "rebuilding" => Ok(Self::Rebuilding),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CreateSshKeyBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSshKeyBody {
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the SSH key.
        pub name: ::std::string::String,
        ///Public key.
        pub public_key: ::std::string::String,
    }
    ///`CreateSshKeyResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSshKeyResponse {
        pub ssh_key: CreateSshKeyResponseSshKey,
    }
    ///`CreateSshKeyResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSshKeyResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`CreateSshKeyResponseSshKey`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSshKeyResponseSshKey {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///MD5 fingerprint of the SSH public key.
        pub fingerprint: ::std::string::String,
        ///ID of the [SSH Key](#tag/ssh-keys).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        pub name: ::std::string::String,
        ///Public key.
        pub public_key: ::std::string::String,
    }
    ///`DeleteFirewallResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteFirewallResponse {
        pub error: DeleteFirewallResponseError,
    }
    ///`DeleteFirewallResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteFirewallResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteImageResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteImageResponse {
        pub error: DeleteImageResponseError,
    }
    ///`DeleteImageResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteImageResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkResponse {
        pub error: DeleteNetworkResponseError,
    }
    ///`DeleteNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteNetworkRouteResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkRouteResponse {
        pub error: DeleteNetworkRouteResponseError,
    }
    ///`DeleteNetworkRouteResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkRouteResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteNetworkSubnetResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkSubnetResponse {
        pub error: DeleteNetworkSubnetResponseError,
    }
    ///`DeleteNetworkSubnetResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteNetworkSubnetResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeletePrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeletePrimaryIpResponse {
        pub error: DeletePrimaryIpResponseError,
    }
    ///`DeletePrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeletePrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteRouteRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DeleteRouteRequest {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`DeleteServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct DeleteServerResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<Action>,
    }
    ///`DeleteServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteSshKeyResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteSshKeyResponse {
        pub error: DeleteSshKeyResponseError,
    }
    ///`DeleteSshKeyResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteSshKeyResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DeleteSubnetRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DeleteSubnetRequest {
        ///IP range in CIDR block notation of the subnet to delete.
        pub ip_range: ::std::string::String,
    }
    /**Describes if, when and how the resource is deprecated. If this field is
    set to `null` the resource is not deprecated. If a value is set, it is
    considered deprecated.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeprecationInfo {
        /**Date of the deprecation announcement.
         */
        pub announced: ::chrono::DateTime<::chrono::offset::Utc>,
        /**Date of the deprecated resource removal.

        Once this date is reached, the resource will not be returned
        by resource type "list" endpoint, and the resource can not be
        used to create new resources. For example, if this is an
        image, you can not create new servers with this image after
        the mentioned date.
        */
        pub unavailable_after: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    ///`DetachFromNetworkRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DetachFromNetworkRequest {
        ///ID of an existing network to detach the Server from.
        pub network: i64,
    }
    ///`DetachServerFromNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DetachServerFromNetworkResponse {
        pub error: DetachServerFromNetworkResponseError,
    }
    ///`DetachServerFromNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DetachServerFromNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DetachServerIsoResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DetachServerIsoResponse {
        pub error: DetachServerIsoResponseError,
    }
    ///`DetachServerIsoResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DetachServerIsoResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DisableServerBackupResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DisableServerBackupResponse {
        pub error: DisableServerBackupResponseError,
    }
    ///`DisableServerBackupResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DisableServerBackupResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`DisableServerRescueResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DisableServerRescueResponse {
        pub error: DisableServerRescueResponseError,
    }
    ///`DisableServerRescueResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DisableServerRescueResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`EnableServerBackupResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnableServerBackupResponse {
        pub error: EnableServerBackupResponseError,
    }
    ///`EnableServerBackupResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnableServerBackupResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`EnableServerRescueBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnableServerRescueBody {
        ///Array of SSH key IDs which should be injected into the rescue system.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ssh_keys: ::std::vec::Vec<i64>,
        ///Type of rescue system to boot.
        #[serde(rename = "type", default = "defaults::enable_server_rescue_body_type")]
        pub type_: EnableServerRescueBodyType,
    }
    impl ::std::default::Default for EnableServerRescueBody {
        fn default() -> Self {
            Self {
                ssh_keys: Default::default(),
                type_: defaults::enable_server_rescue_body_type(),
            }
        }
    }
    ///Type of rescue system to boot.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum EnableServerRescueBodyType {
        #[serde(rename = "linux64")]
        Linux64,
    }
    impl ::std::fmt::Display for EnableServerRescueBodyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Linux64 => f.write_str("linux64"),
            }
        }
    }
    impl ::std::str::FromStr for EnableServerRescueBodyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "linux64" => Ok(Self::Linux64),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for EnableServerRescueBodyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for EnableServerRescueBodyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for EnableServerRescueBodyType {
        fn default() -> Self {
            EnableServerRescueBodyType::Linux64
        }
    }
    /**The `root_password` key in the reply contains the root password that can be used to access the booted rescue system.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct EnableServerRescueResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<Action>,
        ///Password that will be set for this Server once the Action succeeds.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub root_password: ::std::option::Option<::std::string::String>,
    }
    ///`EnableServerRescueResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnableServerRescueResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`FirewallResource`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResource {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub label_selector: ::std::option::Option<FirewallResourceLabelSelector>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub server: ::std::option::Option<FirewallResourceServer>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: FirewallResourceType,
    }
    /**[Label Selector](#description/label-selector) the [Firewall](#tag/firewalls) is applied to.

    Only set for `type` `label_selector`, otherwise `null`.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResourceLabelSelector {
        ///The selector.
        pub selector: ::std::string::String,
    }
    /**[Server](#tag/servers) the [Firewall](#tag/firewalls) is applied to.

    Only set for `type` `server`, otherwise `null`.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResourceServer {
        ///ID of the [Server](#tag/servers).
        pub id: i64,
    }
    ///Type of the resource.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FirewallResourceType {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "label_selector")]
        LabelSelector,
    }
    impl ::std::fmt::Display for FirewallResourceType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::LabelSelector => f.write_str("label_selector"),
            }
        }
    }
    impl ::std::str::FromStr for FirewallResourceType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "label_selector" => Ok(Self::LabelSelector),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FirewallResourceType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FirewallResourceType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`FirewallResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponse {
        pub applied_to: ::std::vec::Vec<FirewallResponseAppliedToItem>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Firewall](#tag/firewalls).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**Name of the [Firewall](#tag/firewalls).

        Must be unique per Project.
        */
        pub name: FirewallResponseName,
        pub rules: ::std::vec::Vec<RuleResponse>,
    }
    ///`FirewallResponseAppliedToItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponseAppliedToItem {
        /**Resources applied to via this [Label Selector](#description/label-selector).
         */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub applied_to_resources:
            ::std::vec::Vec<FirewallResponseAppliedToItemAppliedToResourcesItem>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub label_selector: ::std::option::Option<FirewallResponseAppliedToItemLabelSelector>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub server: ::std::option::Option<FirewallResponseAppliedToItemServer>,
        ///The type of resource to apply.
        #[serde(rename = "type")]
        pub type_: FirewallResponseAppliedToItemType,
    }
    ///`FirewallResponseAppliedToItemAppliedToResourcesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponseAppliedToItemAppliedToResourcesItem {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub server:
            ::std::option::Option<FirewallResponseAppliedToItemAppliedToResourcesItemServer>,
        ///Type of resource.
        #[serde(
            rename = "type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<FirewallResponseAppliedToItemAppliedToResourcesItemType>,
    }
    /**[Server](#tag/servers) the [Firewall](#tag/firewalls) is applied to.

    Only set for `type` `server`, otherwise `null`.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponseAppliedToItemAppliedToResourcesItemServer {
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
    }
    ///Type of resource.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FirewallResponseAppliedToItemAppliedToResourcesItemType {
        #[serde(rename = "server")]
        Server,
    }
    impl ::std::fmt::Display for FirewallResponseAppliedToItemAppliedToResourcesItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
            }
        }
    }
    impl ::std::str::FromStr for FirewallResponseAppliedToItemAppliedToResourcesItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FirewallResponseAppliedToItemAppliedToResourcesItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for FirewallResponseAppliedToItemAppliedToResourcesItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**[Label Selector](#description/label-selector) the [Firewall](#tag/firewalls) is applied to.

    Only set for `type` `label_selector`, otherwise `null`.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponseAppliedToItemLabelSelector {
        ///The selector.
        pub selector: ::std::string::String,
    }
    /**[Server](#tag/servers) the [Firewall](#tag/firewalls) is applied to.

    Only set for `type` `server`, otherwise `null`.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallResponseAppliedToItemServer {
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
    }
    ///The type of resource to apply.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FirewallResponseAppliedToItemType {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "label_selector")]
        LabelSelector,
    }
    impl ::std::fmt::Display for FirewallResponseAppliedToItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::LabelSelector => f.write_str("label_selector"),
            }
        }
    }
    impl ::std::str::FromStr for FirewallResponseAppliedToItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "label_selector" => Ok(Self::LabelSelector),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FirewallResponseAppliedToItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FirewallResponseAppliedToItemType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Name of the [Firewall](#tag/firewalls).

    Must be unique per Project.
    */
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FirewallResponseName(::std::string::String);
    impl ::std::ops::Deref for FirewallResponseName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FirewallResponseName> for ::std::string::String {
        fn from(value: FirewallResponseName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for FirewallResponseName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FirewallResponseName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FirewallResponseName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FirewallResponseName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`FirewallsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallsResponse {
        pub firewalls: ::std::vec::Vec<FirewallResponse>,
        pub meta: ListMeta,
    }
    ///`GetActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetActionResponse {
        pub error: GetActionResponseError,
    }
    ///`GetActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetActionsResponse {
        pub error: GetActionsResponseError,
    }
    ///`GetActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetFirewallActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallActionResponse {
        pub error: GetFirewallActionResponseError,
    }
    ///`GetFirewallActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetFirewallResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallResponse {
        pub error: GetFirewallResponseError,
    }
    ///`GetFirewallResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetFirewallsActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallsActionResponse {
        pub error: GetFirewallsActionResponseError,
    }
    ///`GetFirewallsActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFirewallsActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetImageActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageActionResponse {
        pub error: GetImageActionResponseError,
    }
    ///`GetImageActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetImageResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct GetImageResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<GetImageResponseImage>,
    }
    ///`GetImageResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetImageResponseImage`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageResponseImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: GetImageResponseImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<GetImageResponseImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: GetImageResponseImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: GetImageResponseImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: GetImageResponseImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: GetImageResponseImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetImageResponseImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for GetImageResponseImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for GetImageResponseImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageResponseImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetImageResponseImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for GetImageResponseImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for GetImageResponseImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImageResponseImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetImageResponseImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for GetImageResponseImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for GetImageResponseImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetImageResponseImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for GetImageResponseImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for GetImageResponseImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetImagesActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImagesActionResponse {
        pub error: GetImagesActionResponseError,
    }
    ///`GetImagesActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetImagesActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetLocationResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetLocationResponse {
        pub location: GetLocationResponseLocation,
    }
    ///`GetLocationResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLocationResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetLocationResponseLocation`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLocationResponseLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: GetLocationResponseLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: GetLocationResponseLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationResponseLocationName(::std::string::String);
    impl ::std::ops::Deref for GetLocationResponseLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<GetLocationResponseLocationName> for ::std::string::String {
        fn from(value: GetLocationResponseLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for GetLocationResponseLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for GetLocationResponseLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetLocationResponseLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetLocationResponseLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationResponseLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for GetLocationResponseLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<GetLocationResponseLocationNetworkZone> for ::std::string::String {
        fn from(value: GetLocationResponseLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for GetLocationResponseLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for GetLocationResponseLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetLocationResponseLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetLocationResponseLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`GetNetworkActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworkActionResponse {
        pub error: GetNetworkActionResponseError,
    }
    ///`GetNetworkActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworkActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct GetNetworkResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<GetNetworkResponseNetwork>,
    }
    ///`GetNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetNetworkResponseNetwork`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetNetworkResponseNetwork {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch connection.
        pub expose_routes_to_vswitch: bool,
        ///ID of the [Network](#tag/networks).
        pub id: i64,
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.
        */
        pub ip_range: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Array of IDs of [Load Balancers](#tag/load-balancers) attached to this [Network](#tag/networks).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        ///Name of the [Network](#tag/networks).
        pub name: GetNetworkResponseNetworkName,
        pub protection: GetNetworkResponseNetworkProtection,
        ///Array of routes set in this [Network](#tag/networks).
        pub routes: ::std::vec::Vec<GetNetworkResponseNetworkRoutesItem>,
        ///Array of IDs of [Servers](#tag/servers) attached to this [Network](#tag/networks).
        pub servers: ::std::vec::Vec<i64>,
        ///List of subnets allocated in this [Network](#tag/networks).
        pub subnets: ::std::vec::Vec<GetNetworkResponseNetworkSubnetsItem>,
    }
    ///Name of the [Network](#tag/networks).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetNetworkResponseNetworkName(::std::string::String);
    impl ::std::ops::Deref for GetNetworkResponseNetworkName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<GetNetworkResponseNetworkName> for ::std::string::String {
        fn from(value: GetNetworkResponseNetworkName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for GetNetworkResponseNetworkName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for GetNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetNetworkResponseNetworkName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworkResponseNetworkProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///`GetNetworkResponseNetworkRoutesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetNetworkResponseNetworkRoutesItem {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`GetNetworkResponseNetworkSubnetsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetNetworkResponseNetworkSubnetsItem {
        /**Gateway for [Servers](#tag/servers) attached to this subnet.

        For subnets of type `server` this is always the first IP of the subnets IP range.
        */
        pub gateway: ::std::string::String,
        /**IP range of the subnet.

        Uses CIDR notation.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` property it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: GetNetworkResponseNetworkSubnetsItemType,
        ///ID of the robot vSwitch if the subnet is of type `vswitch`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetNetworkResponseNetworkSubnetsItemType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for GetNetworkResponseNetworkSubnetsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for GetNetworkResponseNetworkSubnetsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetNetworkResponseNetworkSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetNetworkResponseNetworkSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetNetworksActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworksActionResponse {
        pub error: GetNetworksActionResponseError,
    }
    ///`GetNetworksActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNetworksActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetPricingResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponse {
        pub pricing: GetPricingResponsePricing,
    }
    ///`GetPricingResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetPricingResponsePricing`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricing {
        ///Currency the returned prices are expressed in, coded according to [ISO 4217](https://wikipedia.org/wiki/ISO_4217).
        pub currency: ::std::string::String,
        pub floating_ip: GetPricingResponsePricingFloatingIp,
        ///Price of [Floating IPs](#tag/floating-ips) per type and per [Location](#tag/locations).
        pub floating_ips: ::std::vec::Vec<GetPricingResponsePricingFloatingIpsItem>,
        pub image: GetPricingResponsePricingImage,
        ///Price of Load Balancer per [type](#tag/load-balancer-types) and per [Location](#tag/locations).
        pub load_balancer_types: ::std::vec::Vec<GetPricingResponsePricingLoadBalancerTypesItem>,
        ///Price of [Primary IPs](#tag/primary-ips) per type and per [Location](#tag/locations).
        pub primary_ips: ::std::vec::Vec<GetPricingResponsePricingPrimaryIpsItem>,
        pub server_backup: GetPricingResponsePricingServerBackup,
        ///Price of Server per [type](#tag/server-types) and per [Location](#tag/locations).
        pub server_types: ::std::vec::Vec<GetPricingResponsePricingServerTypesItem>,
        ///VAT rate used for calculating prices with VAT.
        pub vat_rate: ::std::string::String,
        pub volume: GetPricingResponsePricingVolume,
    }
    /**Price of [Floating IPs](#tag/floating-ips).

    **Deprecated**: This field is deprecated, please refer to the `floating_ips` field instead.

    See the [Changelog](https://docs.hetzner.cloud/changelog#2024-08-29-field-floating_ip-in-pricing-response-is-now-deprecated) for more details.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingFloatingIp {
        pub price_monthly: GetPricingResponsePricingFloatingIpPriceMonthly,
    }
    ///Price of one [Floating IP](#tag/floating-ips) per month.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingFloatingIpPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///`GetPricingResponsePricingFloatingIpsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingFloatingIpsItem {
        ///Price of the [Floating IP](#tag/floating-ips) type per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetPricingResponsePricingFloatingIpsItemPricesItem>,
        ///Type of [Floating IP](#tag/floating-ips) the price is for.
        #[serde(rename = "type")]
        pub type_: GetPricingResponsePricingFloatingIpsItemType,
    }
    ///`GetPricingResponsePricingFloatingIpsItemPricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingFloatingIpsItemPricesItem {
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_monthly: GetPricingResponsePricingFloatingIpsItemPricesItemPriceMonthly,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingFloatingIpsItemPricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of [Floating IP](#tag/floating-ips) the price is for.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPricingResponsePricingFloatingIpsItemType {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
    }
    impl ::std::fmt::Display for GetPricingResponsePricingFloatingIpsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ipv4 => f.write_str("ipv4"),
                Self::Ipv6 => f.write_str("ipv6"),
            }
        }
    }
    impl ::std::str::FromStr for GetPricingResponsePricingFloatingIpsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ipv4" => Ok(Self::Ipv4),
                "ipv6" => Ok(Self::Ipv6),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPricingResponsePricingFloatingIpsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetPricingResponsePricingFloatingIpsItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Price of [Images](#tag/images).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingImage {
        pub price_per_gb_month: GetPricingResponsePricingImagePricePerGbMonth,
    }
    ///Price of [Images](#tag/images) per GB/month.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingImagePricePerGbMonth {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///`GetPricingResponsePricingLoadBalancerTypesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingLoadBalancerTypesItem {
        ///ID of the [Load Balancer Types](#tag/load-balancer-types) the price is for.
        pub id: i64,
        ///Name of the [Load Balancer Types](#tag/load-balancer-types) the price is for.
        pub name: ::std::string::String,
        ///Price of the [Load Balancer Types](#tag/load-balancer-types) per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetPricingResponsePricingLoadBalancerTypesItemPricesItem>,
    }
    ///`GetPricingResponsePricingLoadBalancerTypesItemPricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingLoadBalancerTypesItemPricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: GetPricingResponsePricingLoadBalancerTypesItemPricesItemPriceHourly,
        pub price_monthly: GetPricingResponsePricingLoadBalancerTypesItemPricesItemPriceMonthly,
        pub price_per_tb_traffic:
            GetPricingResponsePricingLoadBalancerTypesItemPricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingLoadBalancerTypesItemPricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingLoadBalancerTypesItemPricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingLoadBalancerTypesItemPricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///`GetPricingResponsePricingPrimaryIpsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingPrimaryIpsItem {
        ///Price of the [Primary IP](#tag/primary-ips) type per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetPricingResponsePricingPrimaryIpsItemPricesItem>,
        ///Type of [Primary IP](#tag/primary-ips) the price is for.
        #[serde(rename = "type")]
        pub type_: GetPricingResponsePricingPrimaryIpsItemType,
    }
    ///`GetPricingResponsePricingPrimaryIpsItemPricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingPrimaryIpsItemPricesItem {
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: GetPricingResponsePricingPrimaryIpsItemPricesItemPriceHourly,
        pub price_monthly: GetPricingResponsePricingPrimaryIpsItemPricesItemPriceMonthly,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingPrimaryIpsItemPricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingPrimaryIpsItemPricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of [Primary IP](#tag/primary-ips) the price is for.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPricingResponsePricingPrimaryIpsItemType {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
    }
    impl ::std::fmt::Display for GetPricingResponsePricingPrimaryIpsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ipv4 => f.write_str("ipv4"),
                Self::Ipv6 => f.write_str("ipv6"),
            }
        }
    }
    impl ::std::str::FromStr for GetPricingResponsePricingPrimaryIpsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ipv4" => Ok(Self::Ipv4),
                "ipv6" => Ok(Self::Ipv6),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPricingResponsePricingPrimaryIpsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetPricingResponsePricingPrimaryIpsItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Price of [Server](#tag/servers) backups.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerBackup {
        ///Price increase of the [Server](#tag/servers) base price in percentage.
        pub percentage: ::std::string::String,
    }
    ///`GetPricingResponsePricingServerTypesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerTypesItem {
        ///ID of the [Server Types](#tag/server-types) the price is for.
        pub id: i64,
        ///Name of the [Server Types](#tag/server-types) the price is for.
        pub name: ::std::string::String,
        ///Price of the [Server Types](#tag/server-types) per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetPricingResponsePricingServerTypesItemPricesItem>,
    }
    ///`GetPricingResponsePricingServerTypesItemPricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerTypesItemPricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: GetPricingResponsePricingServerTypesItemPricesItemPriceHourly,
        pub price_monthly: GetPricingResponsePricingServerTypesItemPricesItemPriceMonthly,
        pub price_per_tb_traffic:
            GetPricingResponsePricingServerTypesItemPricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerTypesItemPricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerTypesItemPricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingServerTypesItemPricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Price of [Volumes](#tag/volumes).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingVolume {
        pub price_per_gb_month: GetPricingResponsePricingVolumePricePerGbMonth,
    }
    ///Price of [Volumes](#tag/volumes) per GB/month.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPricingResponsePricingVolumePricePerGbMonth {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///`GetPrimaryIpActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpActionResponse {
        pub error: GetPrimaryIpActionResponseError,
    }
    ///`GetPrimaryIpActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetPrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpResponse {
        pub error: GetPrimaryIpResponseError,
    }
    ///`GetPrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetPrimaryIpsActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpsActionResponse {
        pub error: GetPrimaryIpsActionResponseError,
    }
    ///`GetPrimaryIpsActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPrimaryIpsActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetServerActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerActionResponse {
        pub error: GetServerActionResponseError,
    }
    ///`GetServerActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetServerMetricsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerMetricsResponse {
        pub metrics: GetServerMetricsResponseMetrics,
    }
    ///`GetServerMetricsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerMetricsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetServerMetricsResponseMetrics`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerMetricsResponseMetrics {
        ///End of period of metrics reported (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub end: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Start of period of metrics reported (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub start: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Resolution of results in seconds.
        pub step: f64,
        ///Hash with timeseries information, containing the name of timeseries as key.
        pub time_series: ::std::collections::HashMap<
            ::std::string::String,
            GetServerMetricsResponseMetricsTimeSeriesValue,
        >,
    }
    ///`GetServerMetricsResponseMetricsTimeSeriesValue`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerMetricsResponseMetricsTimeSeriesValue {
        ///Metrics timestamps with values.
        pub values: ::std::vec::Vec<[::serde_json::Value; 2usize]>,
    }
    ///`GetServerMetricsTypeItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerMetricsTypeItem {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "disk")]
        Disk,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for GetServerMetricsTypeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cpu => f.write_str("cpu"),
                Self::Disk => f.write_str("disk"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerMetricsTypeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "disk" => Ok(Self::Disk),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerMetricsTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerMetricsTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct GetServerResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub server: ::std::option::Option<GetServerResponseServer>,
    }
    ///`GetServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetServerResponseServer`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServer {
        ///Time window (UTC) in which the backup will run, or null if the backups are not enabled.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub backup_window: ::std::option::Option<::std::string::String>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
        ///Image the server is based on.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image: ::std::option::Option<GetServerResponseServerImage>,
        ///Free Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub included_traffic: ::std::option::Option<i64>,
        ///Inbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ingoing_traffic: ::std::option::Option<i64>,
        ///ISO Image that is attached to this Server. Null if no ISO is attached.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub iso: ::std::option::Option<GetServerResponseServerIso>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Load Balancer IDs assigned to the server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        pub location: GetServerResponseServerLocation,
        ///True if Server has been locked and is not available to user.
        pub locked: bool,
        ///Name of the Server (must be unique per Project and a valid hostname as per RFC 1123).
        pub name: ::std::string::String,
        ///Outbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub outgoing_traffic: ::std::option::Option<i64>,
        ///The placement group the server is assigned to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub placement_group: ::std::option::Option<PlacementGroupNullable>,
        ///Size of the primary Disk.
        pub primary_disk_size: f64,
        ///Private networks information.
        pub private_net: ::std::vec::Vec<GetServerResponseServerPrivateNetItem>,
        pub protection: GetServerResponseServerProtection,
        pub public_net: GetServerResponseServerPublicNet,
        ///True if rescue mode is enabled. Server will then boot into rescue system on next reboot.
        pub rescue_enabled: bool,
        pub server_type: GetServerResponseServerServerType,
        ///Status of the Server.
        pub status: GetServerResponseServerStatus,
        ///IDs of Volumes assigned to this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub volumes: ::std::vec::Vec<i64>,
    }
    ///Image the server is based on.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: GetServerResponseServerImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<GetServerResponseServerImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: GetServerResponseServerImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: GetServerResponseServerImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: GetServerResponseServerImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: GetServerResponseServerImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for GetServerResponseServerImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for GetServerResponseServerImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for GetServerResponseServerImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for GetServerResponseServerImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///ISO Image that is attached to this Server. Null if no ISO is attached.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerIso {
        /**CPU architecture compatible with the ISO.

        Null indicates no restriction on the architecture (wildcard).
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub architecture: ::std::option::Option<GetServerResponseServerIsoArchitecture>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the ISO.
        pub description: ::std::string::String,
        ///ID of the [ISO](#tag/isos).
        pub id: ::std::num::NonZeroU64,
        ///Unique identifier of the ISO. Only set for public ISOs.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Type of the ISO.
        #[serde(
            rename = "type",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub type_: ::std::option::Option<GetServerResponseServerIsoType>,
    }
    /**CPU architecture compatible with the ISO.

    Null indicates no restriction on the architecture (wildcard).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerIsoArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for GetServerResponseServerIsoArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerIsoArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the ISO.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerIsoType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }
    impl ::std::fmt::Display for GetServerResponseServerIsoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => f.write_str("public"),
                Self::Private => f.write_str("private"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerIsoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Location this Resource is located at.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: GetServerResponseServerLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: GetServerResponseServerLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetServerResponseServerLocationName(::std::string::String);
    impl ::std::ops::Deref for GetServerResponseServerLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<GetServerResponseServerLocationName> for ::std::string::String {
        fn from(value: GetServerResponseServerLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetServerResponseServerLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetServerResponseServerLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for GetServerResponseServerLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<GetServerResponseServerLocationNetworkZone> for ::std::string::String {
        fn from(value: GetServerResponseServerLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetServerResponseServerLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`GetServerResponseServerPrivateNetItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct GetServerResponseServerPrivateNetItem {
        ///Additional IP addresses of the server on the network.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///The server IP address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::string::String>,
        ///The server MAC address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub mac_address: ::std::option::Option<::std::string::String>,
        ///The Network ID the server is attached to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<i64>,
    }
    ///Protection configuration for the Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerProtection {
        ///If true, prevents the Server from being deleted.
        pub delete: bool,
        ///If true, prevents the Server from being rebuilt.
        pub rebuild: bool,
    }
    ///Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerPublicNet {
        ///Firewalls applied to the public network interface of this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub firewalls: ::std::vec::Vec<ServerPublicNetFirewall>,
        ///IDs of Floating IPs assigned to this Server.
        pub floating_ips: ::std::vec::Vec<i64>,
        ///IP address (v4) and its reverse DNS entry of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv4: ::std::option::Option<GetServerResponseServerPublicNetIpv4>,
        ///IPv6 network assigned to this Server and its reverse DNS entry.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv6: ::std::option::Option<GetServerResponseServerPublicNetIpv6>,
    }
    ///IP address (v4) and its reverse DNS entry of this Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerPublicNetIpv4 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entry for the IPv4 addresses of this Server.
        pub dns_ptr: ::std::string::String,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v4) of this Server.
        pub ip: ::std::string::String,
    }
    ///IPv6 network assigned to this Server and its reverse DNS entry.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerPublicNetIpv6 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entries for the IPv6 addresses of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr:
            ::std::option::Option<::std::vec::Vec<GetServerResponseServerPublicNetIpv6DnsPtrItem>>,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v6) of this Server.
        pub ip: ::std::string::String,
    }
    ///`GetServerResponseServerPublicNetIpv6DnsPtrItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerPublicNetIpv6DnsPtrItem {
        ///DNS pointer for the specific IP address.
        pub dns_ptr: ::std::string::String,
        ///Single IPv6 address of this Server for which the reverse DNS entry has been set up.
        pub ip: ::std::string::String,
    }
    ///`GetServerResponseServerServerType`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerType {
        /**CPU architecture of the Server Type.
         */
        pub architecture: GetServerResponseServerServerTypeArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: GetServerResponseServerServerTypeCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<GetServerResponseServerServerTypeLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetServerResponseServerServerTypePricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: GetServerResponseServerServerTypeStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerServerTypeArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for GetServerResponseServerServerTypeArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerServerTypeArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerServerTypeArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetServerResponseServerServerTypeArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerServerTypeCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for GetServerResponseServerServerTypeCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerServerTypeCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerTypeLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`GetServerResponseServerServerTypePricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerTypePricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: GetServerResponseServerServerTypePricesItemPriceHourly,
        pub price_monthly: GetServerResponseServerServerTypePricesItemPriceMonthly,
        pub price_per_tb_traffic: GetServerResponseServerServerTypePricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerTypePricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerTypePricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerResponseServerServerTypePricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerServerTypeStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for GetServerResponseServerServerTypeStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerServerTypeStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetServerResponseServerServerTypeStorageType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerResponseServerStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "migrating")]
        Migrating,
        #[serde(rename = "rebuilding")]
        Rebuilding,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for GetServerResponseServerStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Initializing => f.write_str("initializing"),
                Self::Starting => f.write_str("starting"),
                Self::Stopping => f.write_str("stopping"),
                Self::Off => f.write_str("off"),
                Self::Deleting => f.write_str("deleting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Rebuilding => f.write_str("rebuilding"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerResponseServerStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "initializing" => Ok(Self::Initializing),
                "starting" => Ok(Self::Starting),
                "stopping" => Ok(Self::Stopping),
                "off" => Ok(Self::Off),
                "deleting" => Ok(Self::Deleting),
                "migrating" => Ok(Self::Migrating),
                "rebuilding" => Ok(Self::Rebuilding),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetServerTypeResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponse {
        pub server_type: GetServerTypeResponseServerType,
    }
    ///`GetServerTypeResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetServerTypeResponseServerType`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerType {
        /**CPU architecture of the Server Type.
         */
        pub architecture: GetServerTypeResponseServerTypeArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: GetServerTypeResponseServerTypeCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<GetServerTypeResponseServerTypeLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<GetServerTypeResponseServerTypePricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: GetServerTypeResponseServerTypeStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerTypeResponseServerTypeArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for GetServerTypeResponseServerTypeArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerTypeResponseServerTypeArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerTypeResponseServerTypeArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetServerTypeResponseServerTypeArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerTypeResponseServerTypeCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for GetServerTypeResponseServerTypeCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerTypeResponseServerTypeCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerTypeResponseServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerTypeResponseServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerTypeLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`GetServerTypeResponseServerTypePricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerTypePricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: GetServerTypeResponseServerTypePricesItemPriceHourly,
        pub price_monthly: GetServerTypeResponseServerTypePricesItemPriceMonthly,
        pub price_per_tb_traffic: GetServerTypeResponseServerTypePricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerTypePricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerTypePricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServerTypeResponseServerTypePricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetServerTypeResponseServerTypeStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for GetServerTypeResponseServerTypeStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for GetServerTypeResponseServerTypeStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetServerTypeResponseServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetServerTypeResponseServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetServersActionResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServersActionResponse {
        pub error: GetServersActionResponseError,
    }
    ///`GetServersActionResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetServersActionResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetSshKeyResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSshKeyResponse {
        pub ssh_key: GetSshKeyResponseSshKey,
    }
    ///`GetSshKeyResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSshKeyResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`GetSshKeyResponseSshKey`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSshKeyResponseSshKey {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///MD5 fingerprint of the SSH public key.
        pub fingerprint: ::std::string::String,
        ///ID of the [SSH Key](#tag/ssh-keys).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        pub name: ::std::string::String,
        ///Public key.
        pub public_key: ::std::string::String,
    }
    ///`ListFirewallActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallActionsResponse {
        pub error: ListFirewallActionsResponseError,
    }
    ///`ListFirewallActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListFirewallActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListFirewallActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListFirewallActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListFirewallActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListFirewallActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListFirewallActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListFirewallActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListFirewallActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListFirewallActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListFirewallActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListFirewallActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListFirewallsActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallsActionsResponse {
        pub error: ListFirewallsActionsResponseError,
    }
    ///`ListFirewallsActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallsActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListFirewallsActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListFirewallsActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListFirewallsActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListFirewallsActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListFirewallsActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListFirewallsActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListFirewallsActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListFirewallsActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListFirewallsActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListFirewallsActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListFirewallsActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListFirewallsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallsResponse {
        pub error: ListFirewallsResponseError,
    }
    ///`ListFirewallsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListFirewallsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListFirewallsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListFirewallsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "created:asc")]
        CreatedAsc,
        #[serde(rename = "created:desc")]
        CreatedDesc,
    }
    impl ::std::fmt::Display for ListFirewallsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
                Self::Created => f.write_str("created"),
                Self::CreatedAsc => f.write_str("created:asc"),
                Self::CreatedDesc => f.write_str("created:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListFirewallsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                "created" => Ok(Self::Created),
                "created:asc" => Ok(Self::CreatedAsc),
                "created:desc" => Ok(Self::CreatedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListFirewallsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListFirewallsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListImageActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImageActionsResponse {
        pub error: ListImageActionsResponseError,
    }
    ///`ListImageActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImageActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListImageActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImageActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListImageActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListImageActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImageActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImageActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImageActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListImageActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListImageActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImageActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImageActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListImagesActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesActionsResponse {
        pub error: ListImagesActionsResponseError,
    }
    ///`ListImagesActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListImagesActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListImagesActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListImagesActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///CPU architecture of the Resource.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListImagesArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListImagesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesResponse {
        pub images: ::std::vec::Vec<ListImagesResponseImagesItem>,
        pub meta: ListMeta,
    }
    ///`ListImagesResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListImagesResponseImagesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesResponseImagesItem {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: ListImagesResponseImagesItemArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<ListImagesResponseImagesItemCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: ListImagesResponseImagesItemOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: ListImagesResponseImagesItemProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: ListImagesResponseImagesItemStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: ListImagesResponseImagesItemType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesResponseImagesItemArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListImagesResponseImagesItemArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesResponseImagesItemArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesResponseImagesItemArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesResponseImagesItemArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesResponseImagesItemCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesResponseImagesItemOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ListImagesResponseImagesItemOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesResponseImagesItemOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesResponseImagesItemOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesResponseImagesItemOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListImagesResponseImagesItemProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesResponseImagesItemStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for ListImagesResponseImagesItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesResponseImagesItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesResponseImagesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesResponseImagesItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesResponseImagesItemType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for ListImagesResponseImagesItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesResponseImagesItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesResponseImagesItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesResponseImagesItemType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListImagesSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "created:asc")]
        CreatedAsc,
        #[serde(rename = "created:desc")]
        CreatedDesc,
    }
    impl ::std::fmt::Display for ListImagesSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
                Self::Created => f.write_str("created"),
                Self::CreatedAsc => f.write_str("created:asc"),
                Self::CreatedDesc => f.write_str("created:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                "created" => Ok(Self::Created),
                "created:asc" => Ok(Self::CreatedAsc),
                "created:desc" => Ok(Self::CreatedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesStatusItem {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for ListImagesStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListImagesTypeItem {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for ListImagesTypeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for ListImagesTypeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListImagesTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListImagesTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListLocationsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationsResponse {
        ///List of [Locations](#tag/locations).
        pub locations: ::std::vec::Vec<ListLocationsResponseLocationsItem>,
        pub meta: ListMeta,
    }
    ///`ListLocationsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListLocationsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListLocationsResponseLocationsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListLocationsResponseLocationsItem {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: ListLocationsResponseLocationsItemName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: ListLocationsResponseLocationsItemNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationsResponseLocationsItemName(::std::string::String);
    impl ::std::ops::Deref for ListLocationsResponseLocationsItemName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ListLocationsResponseLocationsItemName> for ::std::string::String {
        fn from(value: ListLocationsResponseLocationsItemName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ListLocationsResponseLocationsItemName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ListLocationsResponseLocationsItemName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListLocationsResponseLocationsItemName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListLocationsResponseLocationsItemName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationsResponseLocationsItemNetworkZone(::std::string::String);
    impl ::std::ops::Deref for ListLocationsResponseLocationsItemNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ListLocationsResponseLocationsItemNetworkZone> for ::std::string::String {
        fn from(value: ListLocationsResponseLocationsItemNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ListLocationsResponseLocationsItemNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ListLocationsResponseLocationsItemNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListLocationsResponseLocationsItemNetworkZone
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListLocationsResponseLocationsItemNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`ListLocationsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListLocationsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
    }
    impl ::std::fmt::Display for ListLocationsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListLocationsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListLocationsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListLocationsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListMeta`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListMeta {
        pub pagination: ListMetaPagination,
    }
    ///See "[Pagination](#description/pagination)" for more information.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListMetaPagination {
        ///Page number of the last page available. Can be null if the current page is the last one.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub last_page: ::std::option::Option<i64>,
        ///Page number of the next page. Can be null if the current page is the last one.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub next_page: ::std::option::Option<i64>,
        ///Current page number.
        pub page: ::std::num::NonZeroU64,
        ///Maximum number of entries returned per page.
        pub per_page: ::std::num::NonZeroU64,
        ///Page number of the previous page. Can be null if the current page is the first one.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub previous_page: ::std::option::Option<i64>,
        ///Total number of entries that exist for this query. Can be null if unknown.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub total_entries: ::std::option::Option<i64>,
    }
    ///`ListNetworkActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworkActionsResponse {
        pub error: ListNetworkActionsResponseError,
    }
    ///`ListNetworkActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworkActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListNetworkActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListNetworkActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListNetworkActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListNetworkMembersResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworkMembersResponse {
        pub members: ::std::vec::Vec<ListNetworkMembersResponseMembersItem>,
        pub meta: ListMeta,
    }
    ///`ListNetworkMembersResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworkMembersResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListNetworkMembersResponseMembersItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworkMembersResponseMembersItem {
        ///Additional IP addresses of the resource within the [Network](#tag/networks).
        pub alias_ips: ::std::vec::Vec<::std::net::Ipv4Addr>,
        ///ID of the attached resource.
        pub id: ::std::num::NonZeroU64,
        ///IP address of the resource within the [Network](#tag/networks).
        pub ip: ::std::net::Ipv4Addr,
        /**Status of the resource within the [Network](#tag/networks).
         */
        pub status: ListNetworkMembersResponseMembersItemStatus,
        /**IP range of the subnet the resource is attached to.

        Uses CIDR notation.
        */
        pub subnet: ::std::string::String,
        /**Type of the resource attached to the [Network](#tag/networks).
         */
        #[serde(rename = "type")]
        pub type_: ListNetworkMembersResponseMembersItemType,
    }
    /**Status of the resource within the [Network](#tag/networks).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkMembersResponseMembersItemStatus {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "attaching")]
        Attaching,
        #[serde(rename = "detaching")]
        Detaching,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListNetworkMembersResponseMembersItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
                Self::Attaching => f.write_str("attaching"),
                Self::Detaching => f.write_str("detaching"),
                Self::Updating => f.write_str("updating"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkMembersResponseMembersItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "attaching" => Ok(Self::Attaching),
                "detaching" => Ok(Self::Detaching),
                "updating" => Ok(Self::Updating),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkMembersResponseMembersItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListNetworkMembersResponseMembersItemStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Type of the resource attached to the [Network](#tag/networks).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkMembersResponseMembersItemType {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "load_balancer")]
        LoadBalancer,
    }
    impl ::std::fmt::Display for ListNetworkMembersResponseMembersItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::LoadBalancer => f.write_str("load_balancer"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkMembersResponseMembersItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "load_balancer" => Ok(Self::LoadBalancer),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkMembersResponseMembersItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkMembersResponseMembersItemType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListNetworkMembersSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkMembersSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "type")]
        Type,
        #[serde(rename = "type:asc")]
        TypeAsc,
        #[serde(rename = "type:desc")]
        TypeDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "ip")]
        Ip,
        #[serde(rename = "ip:asc")]
        IpAsc,
        #[serde(rename = "ip:desc")]
        IpDesc,
    }
    impl ::std::fmt::Display for ListNetworkMembersSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Type => f.write_str("type"),
                Self::TypeAsc => f.write_str("type:asc"),
                Self::TypeDesc => f.write_str("type:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Ip => f.write_str("ip"),
                Self::IpAsc => f.write_str("ip:asc"),
                Self::IpDesc => f.write_str("ip:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkMembersSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "type" => Ok(Self::Type),
                "type:asc" => Ok(Self::TypeAsc),
                "type:desc" => Ok(Self::TypeDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "ip" => Ok(Self::Ip),
                "ip:asc" => Ok(Self::IpAsc),
                "ip:desc" => Ok(Self::IpDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkMembersSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkMembersSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Status of the resource within the [Network](#tag/networks).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkMembersStatusItem {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "attaching")]
        Attaching,
        #[serde(rename = "detaching")]
        Detaching,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListNetworkMembersStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
                Self::Attaching => f.write_str("attaching"),
                Self::Detaching => f.write_str("detaching"),
                Self::Updating => f.write_str("updating"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkMembersStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "attaching" => Ok(Self::Attaching),
                "detaching" => Ok(Self::Detaching),
                "updating" => Ok(Self::Updating),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkMembersStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkMembersStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Type of the resource attached to the [Network](#tag/networks).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworkMembersTypeItem {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "load_balancer")]
        LoadBalancer,
    }
    impl ::std::fmt::Display for ListNetworkMembersTypeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::LoadBalancer => f.write_str("load_balancer"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworkMembersTypeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "load_balancer" => Ok(Self::LoadBalancer),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworkMembersTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworkMembersTypeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListNetworksActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworksActionsResponse {
        pub error: ListNetworksActionsResponseError,
    }
    ///`ListNetworksActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworksActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListNetworksActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworksActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListNetworksActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworksActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworksActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworksActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworksActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListNetworksActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworksActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworksActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworksActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListNetworksResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworksResponse {
        pub meta: ListMeta,
        pub networks: ::std::vec::Vec<ListNetworksResponseNetworksItem>,
    }
    ///`ListNetworksResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworksResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListNetworksResponseNetworksItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworksResponseNetworksItem {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch connection.
        pub expose_routes_to_vswitch: bool,
        ///ID of the [Network](#tag/networks).
        pub id: i64,
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.
        */
        pub ip_range: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Array of IDs of [Load Balancers](#tag/load-balancers) attached to this [Network](#tag/networks).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        ///Name of the [Network](#tag/networks).
        pub name: ListNetworksResponseNetworksItemName,
        pub protection: ListNetworksResponseNetworksItemProtection,
        ///Array of routes set in this [Network](#tag/networks).
        pub routes: ::std::vec::Vec<ListNetworksResponseNetworksItemRoutesItem>,
        ///Array of IDs of [Servers](#tag/servers) attached to this [Network](#tag/networks).
        pub servers: ::std::vec::Vec<i64>,
        ///List of subnets allocated in this [Network](#tag/networks).
        pub subnets: ::std::vec::Vec<ListNetworksResponseNetworksItemSubnetsItem>,
    }
    ///Name of the [Network](#tag/networks).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListNetworksResponseNetworksItemName(::std::string::String);
    impl ::std::ops::Deref for ListNetworksResponseNetworksItemName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ListNetworksResponseNetworksItemName> for ::std::string::String {
        fn from(value: ListNetworksResponseNetworksItemName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ListNetworksResponseNetworksItemName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworksResponseNetworksItemName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworksResponseNetworksItemName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListNetworksResponseNetworksItemName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListNetworksResponseNetworksItemProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///`ListNetworksResponseNetworksItemRoutesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworksResponseNetworksItemRoutesItem {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`ListNetworksResponseNetworksItemSubnetsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListNetworksResponseNetworksItemSubnetsItem {
        /**Gateway for [Servers](#tag/servers) attached to this subnet.

        For subnets of type `server` this is always the first IP of the subnets IP range.
        */
        pub gateway: ::std::string::String,
        /**IP range of the subnet.

        Uses CIDR notation.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` property it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: ListNetworksResponseNetworksItemSubnetsItemType,
        ///ID of the robot vSwitch if the subnet is of type `vswitch`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworksResponseNetworksItemSubnetsItemType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for ListNetworksResponseNetworksItemSubnetsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworksResponseNetworksItemSubnetsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworksResponseNetworksItemSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListNetworksResponseNetworksItemSubnetsItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListNetworksSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListNetworksSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "created:asc")]
        CreatedAsc,
        #[serde(rename = "created:desc")]
        CreatedDesc,
    }
    impl ::std::fmt::Display for ListNetworksSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
                Self::Created => f.write_str("created"),
                Self::CreatedAsc => f.write_str("created:asc"),
                Self::CreatedDesc => f.write_str("created:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListNetworksSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                "created" => Ok(Self::Created),
                "created:asc" => Ok(Self::CreatedAsc),
                "created:desc" => Ok(Self::CreatedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListNetworksSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListNetworksSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListPrimaryIpActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpActionsResponse {
        pub error: ListPrimaryIpActionsResponseError,
    }
    ///`ListPrimaryIpActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListPrimaryIpActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListPrimaryIpActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListPrimaryIpActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListPrimaryIpActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListPrimaryIpActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListPrimaryIpActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListPrimaryIpActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListPrimaryIpActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListPrimaryIpActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListPrimaryIpActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListPrimaryIpActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListPrimaryIpsActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpsActionsResponse {
        pub error: ListPrimaryIpsActionsResponseError,
    }
    ///`ListPrimaryIpsActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpsActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListPrimaryIpsActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListPrimaryIpsActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListPrimaryIpsActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListPrimaryIpsActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListPrimaryIpsActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListPrimaryIpsActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListPrimaryIpsActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListPrimaryIpsActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListPrimaryIpsActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListPrimaryIpsActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListPrimaryIpsActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListPrimaryIpsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpsResponse {
        pub error: ListPrimaryIpsResponseError,
    }
    ///`ListPrimaryIpsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListPrimaryIpsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListPrimaryIpsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListPrimaryIpsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "created:asc")]
        CreatedAsc,
        #[serde(rename = "created:desc")]
        CreatedDesc,
    }
    impl ::std::fmt::Display for ListPrimaryIpsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Created => f.write_str("created"),
                Self::CreatedAsc => f.write_str("created:asc"),
                Self::CreatedDesc => f.write_str("created:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListPrimaryIpsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "created" => Ok(Self::Created),
                "created:asc" => Ok(Self::CreatedAsc),
                "created:desc" => Ok(Self::CreatedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListPrimaryIpsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListPrimaryIpsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListServerActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerActionsResponse {
        pub error: ListServerActionsResponseError,
    }
    ///`ListServerActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListServerActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServerActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListServerActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListServerActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServerActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServerActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServerActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListServerActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListServerActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServerActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServerActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListServerTypesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponse {
        pub meta: ListMeta,
        pub server_types: ::std::vec::Vec<ListServerTypesResponseServerTypesItem>,
    }
    ///`ListServerTypesResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListServerTypesResponseServerTypesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItem {
        /**CPU architecture of the Server Type.
         */
        pub architecture: ListServerTypesResponseServerTypesItemArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: ListServerTypesResponseServerTypesItemCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<ListServerTypesResponseServerTypesItemLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<ListServerTypesResponseServerTypesItemPricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: ListServerTypesResponseServerTypesItemStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServerTypesResponseServerTypesItemArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListServerTypesResponseServerTypesItemArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListServerTypesResponseServerTypesItemArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServerTypesResponseServerTypesItemArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServerTypesResponseServerTypesItemArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServerTypesResponseServerTypesItemCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for ListServerTypesResponseServerTypesItemCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for ListServerTypesResponseServerTypesItemCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServerTypesResponseServerTypesItemCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServerTypesResponseServerTypesItemCpuType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItemLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`ListServerTypesResponseServerTypesItemPricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItemPricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: ListServerTypesResponseServerTypesItemPricesItemPriceHourly,
        pub price_monthly: ListServerTypesResponseServerTypesItemPricesItemPriceMonthly,
        pub price_per_tb_traffic: ListServerTypesResponseServerTypesItemPricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItemPricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItemPricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServerTypesResponseServerTypesItemPricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServerTypesResponseServerTypesItemStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for ListServerTypesResponseServerTypesItemStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for ListServerTypesResponseServerTypesItemStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServerTypesResponseServerTypesItemStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServerTypesResponseServerTypesItemStorageType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListServersActionsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersActionsResponse {
        pub error: ListServersActionsResponseError,
    }
    ///`ListServersActionsResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersActionsResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListServersActionsSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersActionsSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "command")]
        Command,
        #[serde(rename = "command:asc")]
        CommandAsc,
        #[serde(rename = "command:desc")]
        CommandDesc,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "status:asc")]
        StatusAsc,
        #[serde(rename = "status:desc")]
        StatusDesc,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "started:asc")]
        StartedAsc,
        #[serde(rename = "started:desc")]
        StartedDesc,
        #[serde(rename = "finished")]
        Finished,
        #[serde(rename = "finished:asc")]
        FinishedAsc,
        #[serde(rename = "finished:desc")]
        FinishedDesc,
    }
    impl ::std::fmt::Display for ListServersActionsSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Command => f.write_str("command"),
                Self::CommandAsc => f.write_str("command:asc"),
                Self::CommandDesc => f.write_str("command:desc"),
                Self::Status => f.write_str("status"),
                Self::StatusAsc => f.write_str("status:asc"),
                Self::StatusDesc => f.write_str("status:desc"),
                Self::Started => f.write_str("started"),
                Self::StartedAsc => f.write_str("started:asc"),
                Self::StartedDesc => f.write_str("started:desc"),
                Self::Finished => f.write_str("finished"),
                Self::FinishedAsc => f.write_str("finished:asc"),
                Self::FinishedDesc => f.write_str("finished:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersActionsSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "command" => Ok(Self::Command),
                "command:asc" => Ok(Self::CommandAsc),
                "command:desc" => Ok(Self::CommandDesc),
                "status" => Ok(Self::Status),
                "status:asc" => Ok(Self::StatusAsc),
                "status:desc" => Ok(Self::StatusDesc),
                "started" => Ok(Self::Started),
                "started:asc" => Ok(Self::StartedAsc),
                "started:desc" => Ok(Self::StartedDesc),
                "finished" => Ok(Self::Finished),
                "finished:asc" => Ok(Self::FinishedAsc),
                "finished:desc" => Ok(Self::FinishedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersActionsSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Action.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersActionsStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ListServersActionsStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Success => f.write_str("success"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersActionsStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "success" => Ok(Self::Success),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersActionsStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListServersResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponse {
        pub meta: ListMeta,
        pub servers: ::std::vec::Vec<ListServersResponseServersItem>,
    }
    ///`ListServersResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListServersResponseServersItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItem {
        ///Time window (UTC) in which the backup will run, or null if the backups are not enabled.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub backup_window: ::std::option::Option<::std::string::String>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
        ///Image the server is based on.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image: ::std::option::Option<ListServersResponseServersItemImage>,
        ///Free Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub included_traffic: ::std::option::Option<i64>,
        ///Inbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ingoing_traffic: ::std::option::Option<i64>,
        ///ISO Image that is attached to this Server. Null if no ISO is attached.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub iso: ::std::option::Option<ListServersResponseServersItemIso>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Load Balancer IDs assigned to the server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        pub location: ListServersResponseServersItemLocation,
        ///True if Server has been locked and is not available to user.
        pub locked: bool,
        ///Name of the Server (must be unique per Project and a valid hostname as per RFC 1123).
        pub name: ::std::string::String,
        ///Outbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub outgoing_traffic: ::std::option::Option<i64>,
        ///The placement group the server is assigned to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub placement_group: ::std::option::Option<PlacementGroupNullable>,
        ///Size of the primary Disk.
        pub primary_disk_size: f64,
        ///Private networks information.
        pub private_net: ::std::vec::Vec<ListServersResponseServersItemPrivateNetItem>,
        pub protection: ListServersResponseServersItemProtection,
        pub public_net: ListServersResponseServersItemPublicNet,
        ///True if rescue mode is enabled. Server will then boot into rescue system on next reboot.
        pub rescue_enabled: bool,
        pub server_type: ListServersResponseServersItemServerType,
        ///Status of the Server.
        pub status: ListServersResponseServersItemStatus,
        ///IDs of Volumes assigned to this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub volumes: ::std::vec::Vec<i64>,
    }
    ///Image the server is based on.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: ListServersResponseServersItemImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<ListServersResponseServersItemImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: ListServersResponseServersItemImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: ListServersResponseServersItemImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: ListServersResponseServersItemImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: ListServersResponseServersItemImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemImageArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemImageOsFlavor
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersResponseServersItemImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersResponseServersItemImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///ISO Image that is attached to this Server. Null if no ISO is attached.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemIso {
        /**CPU architecture compatible with the ISO.

        Null indicates no restriction on the architecture (wildcard).
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub architecture: ::std::option::Option<ListServersResponseServersItemIsoArchitecture>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the ISO.
        pub description: ::std::string::String,
        ///ID of the [ISO](#tag/isos).
        pub id: ::std::num::NonZeroU64,
        ///Unique identifier of the ISO. Only set for public ISOs.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Type of the ISO.
        #[serde(
            rename = "type",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub type_: ::std::option::Option<ListServersResponseServersItemIsoType>,
    }
    /**CPU architecture compatible with the ISO.

    Null indicates no restriction on the architecture (wildcard).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemIsoArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemIsoArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemIsoArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemIsoArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the ISO.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemIsoType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemIsoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => f.write_str("public"),
                Self::Private => f.write_str("private"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemIsoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemIsoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersResponseServersItemIsoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Location this Resource is located at.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: ListServersResponseServersItemLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: ListServersResponseServersItemLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListServersResponseServersItemLocationName(::std::string::String);
    impl ::std::ops::Deref for ListServersResponseServersItemLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ListServersResponseServersItemLocationName> for ::std::string::String {
        fn from(value: ListServersResponseServersItemLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersResponseServersItemLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListServersResponseServersItemLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListServersResponseServersItemLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for ListServersResponseServersItemLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ListServersResponseServersItemLocationNetworkZone>
        for ::std::string::String
    {
        fn from(value: ListServersResponseServersItemLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemLocationNetworkZone
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListServersResponseServersItemLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`ListServersResponseServersItemPrivateNetItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ListServersResponseServersItemPrivateNetItem {
        ///Additional IP addresses of the server on the network.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///The server IP address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::string::String>,
        ///The server MAC address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub mac_address: ::std::option::Option<::std::string::String>,
        ///The Network ID the server is attached to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<i64>,
    }
    ///Protection configuration for the Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemProtection {
        ///If true, prevents the Server from being deleted.
        pub delete: bool,
        ///If true, prevents the Server from being rebuilt.
        pub rebuild: bool,
    }
    ///Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemPublicNet {
        ///Firewalls applied to the public network interface of this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub firewalls: ::std::vec::Vec<ServerPublicNetFirewall>,
        ///IDs of Floating IPs assigned to this Server.
        pub floating_ips: ::std::vec::Vec<i64>,
        ///IP address (v4) and its reverse DNS entry of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv4: ::std::option::Option<ListServersResponseServersItemPublicNetIpv4>,
        ///IPv6 network assigned to this Server and its reverse DNS entry.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv6: ::std::option::Option<ListServersResponseServersItemPublicNetIpv6>,
    }
    ///IP address (v4) and its reverse DNS entry of this Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemPublicNetIpv4 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entry for the IPv4 addresses of this Server.
        pub dns_ptr: ::std::string::String,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v4) of this Server.
        pub ip: ::std::string::String,
    }
    ///IPv6 network assigned to this Server and its reverse DNS entry.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemPublicNetIpv6 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entries for the IPv6 addresses of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr: ::std::option::Option<
            ::std::vec::Vec<ListServersResponseServersItemPublicNetIpv6DnsPtrItem>,
        >,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v6) of this Server.
        pub ip: ::std::string::String,
    }
    ///`ListServersResponseServersItemPublicNetIpv6DnsPtrItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemPublicNetIpv6DnsPtrItem {
        ///DNS pointer for the specific IP address.
        pub dns_ptr: ::std::string::String,
        ///Single IPv6 address of this Server for which the reverse DNS entry has been set up.
        pub ip: ::std::string::String,
    }
    ///`ListServersResponseServersItemServerType`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerType {
        /**CPU architecture of the Server Type.
         */
        pub architecture: ListServersResponseServersItemServerTypeArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: ListServersResponseServersItemServerTypeCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<ListServersResponseServersItemServerTypeLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<ListServersResponseServersItemServerTypePricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: ListServersResponseServersItemServerTypeStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemServerTypeArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemServerTypeArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemServerTypeArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemServerTypeArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemServerTypeArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemServerTypeCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemServerTypeCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemServerTypeCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemServerTypeCpuType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerTypeLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`ListServersResponseServersItemServerTypePricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerTypePricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: ListServersResponseServersItemServerTypePricesItemPriceHourly,
        pub price_monthly: ListServersResponseServersItemServerTypePricesItemPriceMonthly,
        pub price_per_tb_traffic:
            ListServersResponseServersItemServerTypePricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerTypePricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerTypePricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListServersResponseServersItemServerTypePricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemServerTypeStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemServerTypeStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemServerTypeStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ListServersResponseServersItemServerTypeStorageType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersResponseServersItemStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "migrating")]
        Migrating,
        #[serde(rename = "rebuilding")]
        Rebuilding,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ListServersResponseServersItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Initializing => f.write_str("initializing"),
                Self::Starting => f.write_str("starting"),
                Self::Stopping => f.write_str("stopping"),
                Self::Off => f.write_str("off"),
                Self::Deleting => f.write_str("deleting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Rebuilding => f.write_str("rebuilding"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersResponseServersItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "initializing" => Ok(Self::Initializing),
                "starting" => Ok(Self::Starting),
                "stopping" => Ok(Self::Stopping),
                "off" => Ok(Self::Off),
                "deleting" => Ok(Self::Deleting),
                "migrating" => Ok(Self::Migrating),
                "rebuilding" => Ok(Self::Rebuilding),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersResponseServersItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersResponseServersItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListServersSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "created:asc")]
        CreatedAsc,
        #[serde(rename = "created:desc")]
        CreatedDesc,
    }
    impl ::std::fmt::Display for ListServersSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
                Self::Created => f.write_str("created"),
                Self::CreatedAsc => f.write_str("created:asc"),
                Self::CreatedDesc => f.write_str("created:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                "created" => Ok(Self::Created),
                "created:asc" => Ok(Self::CreatedAsc),
                "created:desc" => Ok(Self::CreatedDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListServersStatusItem {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "migrating")]
        Migrating,
        #[serde(rename = "rebuilding")]
        Rebuilding,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ListServersStatusItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Initializing => f.write_str("initializing"),
                Self::Starting => f.write_str("starting"),
                Self::Stopping => f.write_str("stopping"),
                Self::Off => f.write_str("off"),
                Self::Deleting => f.write_str("deleting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Rebuilding => f.write_str("rebuilding"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ListServersStatusItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "initializing" => Ok(Self::Initializing),
                "starting" => Ok(Self::Starting),
                "stopping" => Ok(Self::Stopping),
                "off" => Ok(Self::Off),
                "deleting" => Ok(Self::Deleting),
                "migrating" => Ok(Self::Migrating),
                "rebuilding" => Ok(Self::Rebuilding),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListServersStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListServersStatusItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListSshKeysResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListSshKeysResponse {
        pub meta: ListMeta,
        pub ssh_keys: ::std::vec::Vec<ListSshKeysResponseSshKeysItem>,
    }
    ///`ListSshKeysResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListSshKeysResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ListSshKeysResponseSshKeysItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListSshKeysResponseSshKeysItem {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///MD5 fingerprint of the SSH public key.
        pub fingerprint: ::std::string::String,
        ///ID of the [SSH Key](#tag/ssh-keys).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        pub name: ::std::string::String,
        ///Public key.
        pub public_key: ::std::string::String,
    }
    ///`ListSshKeysSortItem`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ListSshKeysSortItem {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id:asc")]
        IdAsc,
        #[serde(rename = "id:desc")]
        IdDesc,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "name:asc")]
        NameAsc,
        #[serde(rename = "name:desc")]
        NameDesc,
    }
    impl ::std::fmt::Display for ListSshKeysSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Id => f.write_str("id"),
                Self::IdAsc => f.write_str("id:asc"),
                Self::IdDesc => f.write_str("id:desc"),
                Self::Name => f.write_str("name"),
                Self::NameAsc => f.write_str("name:asc"),
                Self::NameDesc => f.write_str("name:desc"),
            }
        }
    }
    impl ::std::str::FromStr for ListSshKeysSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "id:asc" => Ok(Self::IdAsc),
                "id:desc" => Ok(Self::IdDesc),
                "name" => Ok(Self::Name),
                "name:asc" => Ok(Self::NameAsc),
                "name:desc" => Ok(Self::NameDesc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ListSshKeysSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ListSshKeysSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Service health check.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct LoadBalancerServiceHealthCheck {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub http: ::std::option::Option<LoadBalancerServiceHealthCheckHttp>,
        ///Time interval in seconds health checks are performed.
        pub interval: i64,
        ///Port the health check will be performed on.
        pub port: i64,
        ///Type of the health check.
        pub protocol: LoadBalancerServiceHealthCheckProtocol,
        ///Unsuccessful retries needed until a target is considered unhealthy; an unhealthy target needs the same number of successful retries to become healthy again.
        pub retries: ::std::num::NonZeroU64,
        ///Time in seconds after an attempt is considered a timeout.
        pub timeout: ::std::num::NonZeroU64,
    }
    ///Additional configuration for protocol http.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct LoadBalancerServiceHealthCheckHttp {
        ///Host header to send in the HTTP request. May not contain spaces, percent or backslash symbols. Can be null, in that case no host header is sent.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub domain: ::std::option::Option<LoadBalancerServiceHealthCheckHttpDomain>,
        ///HTTP path to use for health checks. May not contain literal spaces, use percent-encoding instead.
        pub path: LoadBalancerServiceHealthCheckHttpPath,
        ///String that must be contained in HTTP response in order to pass the health check.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub response: ::std::option::Option<LoadBalancerServiceHealthCheckHttpResponse>,
        ///List of returned HTTP status codes in order to pass the health check. Supports the wildcards `?` for exactly one character and `*` for multiple ones.
        #[serde(default = "defaults::load_balancer_service_health_check_http_status_codes")]
        pub status_codes: ::std::vec::Vec<::std::string::String>,
        ///Use HTTPS for health check.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub tls: ::std::option::Option<bool>,
    }
    ///Host header to send in the HTTP request. May not contain spaces, percent or backslash symbols. Can be null, in that case no host header is sent.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerServiceHealthCheckHttpDomain(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerServiceHealthCheckHttpDomain {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerServiceHealthCheckHttpDomain> for ::std::string::String {
        fn from(value: LoadBalancerServiceHealthCheckHttpDomain) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHealthCheckHttpDomain {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHealthCheckHttpDomain {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHealthCheckHttpDomain {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerServiceHealthCheckHttpDomain {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///HTTP path to use for health checks. May not contain literal spaces, use percent-encoding instead.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerServiceHealthCheckHttpPath(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerServiceHealthCheckHttpPath {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerServiceHealthCheckHttpPath> for ::std::string::String {
        fn from(value: LoadBalancerServiceHealthCheckHttpPath) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHealthCheckHttpPath {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 256usize {
                return Err("longer than 256 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHealthCheckHttpPath {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHealthCheckHttpPath {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerServiceHealthCheckHttpPath {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///String that must be contained in HTTP response in order to pass the health check.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerServiceHealthCheckHttpResponse(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerServiceHealthCheckHttpResponse {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerServiceHealthCheckHttpResponse> for ::std::string::String {
        fn from(value: LoadBalancerServiceHealthCheckHttpResponse) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHealthCheckHttpResponse {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 256usize {
                return Err("longer than 256 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHealthCheckHttpResponse {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHealthCheckHttpResponse {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerServiceHealthCheckHttpResponse {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Type of the health check.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum LoadBalancerServiceHealthCheckProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "http")]
        Http,
    }
    impl ::std::fmt::Display for LoadBalancerServiceHealthCheckProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
                Self::Http => f.write_str("http"),
            }
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHealthCheckProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                "http" => Ok(Self::Http),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHealthCheckProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHealthCheckProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Configuration option for protocol http.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerServiceHttpConfig {
        ///Lifetime of the cookie used for sticky sessions (in seconds).
        pub cookie_lifetime: i64,
        ///Name of the cookie used for sticky sessions.
        pub cookie_name: LoadBalancerServiceHttpConfigCookieName,
        ///Use sticky sessions.
        pub sticky_sessions: bool,
        ///Idle timeout in seconds for the client and server side.
        pub timeout_idle: i64,
    }
    ///Name of the cookie used for sticky sessions.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerServiceHttpConfigCookieName(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerServiceHttpConfigCookieName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerServiceHttpConfigCookieName> for ::std::string::String {
        fn from(value: LoadBalancerServiceHttpConfigCookieName) -> Self {
            value.0
        }
    }
    impl ::std::default::Default for LoadBalancerServiceHttpConfigCookieName {
        fn default() -> Self {
            LoadBalancerServiceHttpConfigCookieName("HCLBSTICKY".to_string())
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHttpConfigCookieName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 100usize {
                return Err("longer than 100 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHttpConfigCookieName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHttpConfigCookieName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerServiceHttpConfigCookieName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Configuration option for protocol https.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerServiceHttpsConfig {
        ///IDs of the Certificates to use for TLS/SSL termination by the Load Balancer; empty for TLS/SSL passthrough.
        pub certificates: ::std::vec::Vec<i64>,
        ///Lifetime of the cookie used for sticky sessions (in seconds).
        pub cookie_lifetime: i64,
        ///Name of the cookie used for sticky sessions.
        pub cookie_name: LoadBalancerServiceHttpsConfigCookieName,
        ///Redirect HTTP requests to HTTPS.
        pub redirect_http: bool,
        ///Use sticky sessions.
        pub sticky_sessions: bool,
        ///Idle timeout in seconds for the client and server side.
        pub timeout_idle: i64,
    }
    ///Name of the cookie used for sticky sessions.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerServiceHttpsConfigCookieName(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerServiceHttpsConfigCookieName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerServiceHttpsConfigCookieName> for ::std::string::String {
        fn from(value: LoadBalancerServiceHttpsConfigCookieName) -> Self {
            value.0
        }
    }
    impl ::std::default::Default for LoadBalancerServiceHttpsConfigCookieName {
        fn default() -> Self {
            LoadBalancerServiceHttpsConfigCookieName("HCLBSTICKY".to_string())
        }
    }
    impl ::std::str::FromStr for LoadBalancerServiceHttpsConfigCookieName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 100usize {
                return Err("longer than 100 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerServiceHttpsConfigCookieName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerServiceHttpsConfigCookieName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerServiceHttpsConfigCookieName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`LoadBalancerTargetHealthStatusItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerTargetHealthStatusItem {
        ///Additional details about why the health check failed. Only present when `status` is `unhealthy`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<LoadBalancerTargetHealthStatusItemDetail>,
        ///HTTP status code returned by the target during the last health check. Only present when `status` is `unhealthy` and `detail` is `unexpected_http_status`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub http_status_code: ::std::option::Option<i64>,
        pub listen_port: i64,
        pub status: LoadBalancerTargetHealthStatusItemStatus,
    }
    ///Additional details about why the health check failed. Only present when `status` is `unhealthy`.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum LoadBalancerTargetHealthStatusItemDetail {
        #[serde(rename = "unspecified")]
        Unspecified,
        #[serde(rename = "layer4_no_connection")]
        Layer4NoConnection,
        #[serde(rename = "layer4_timeout")]
        Layer4Timeout,
        #[serde(rename = "layer7_timeout")]
        Layer7Timeout,
        #[serde(rename = "unexpected_http_status")]
        UnexpectedHttpStatus,
        #[serde(rename = "unexpected_http_content")]
        UnexpectedHttpContent,
    }
    impl ::std::fmt::Display for LoadBalancerTargetHealthStatusItemDetail {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unspecified => f.write_str("unspecified"),
                Self::Layer4NoConnection => f.write_str("layer4_no_connection"),
                Self::Layer4Timeout => f.write_str("layer4_timeout"),
                Self::Layer7Timeout => f.write_str("layer7_timeout"),
                Self::UnexpectedHttpStatus => f.write_str("unexpected_http_status"),
                Self::UnexpectedHttpContent => f.write_str("unexpected_http_content"),
            }
        }
    }
    impl ::std::str::FromStr for LoadBalancerTargetHealthStatusItemDetail {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "unspecified" => Ok(Self::Unspecified),
                "layer4_no_connection" => Ok(Self::Layer4NoConnection),
                "layer4_timeout" => Ok(Self::Layer4Timeout),
                "layer7_timeout" => Ok(Self::Layer7Timeout),
                "unexpected_http_status" => Ok(Self::UnexpectedHttpStatus),
                "unexpected_http_content" => Ok(Self::UnexpectedHttpContent),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerTargetHealthStatusItemDetail {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerTargetHealthStatusItemDetail {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`LoadBalancerTargetHealthStatusItemStatus`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum LoadBalancerTargetHealthStatusItemStatus {
        #[serde(rename = "healthy")]
        Healthy,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for LoadBalancerTargetHealthStatusItemStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Healthy => f.write_str("healthy"),
                Self::Unhealthy => f.write_str("unhealthy"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for LoadBalancerTargetHealthStatusItemStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "healthy" => Ok(Self::Healthy),
                "unhealthy" => Ok(Self::Unhealthy),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerTargetHealthStatusItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerTargetHealthStatusItemStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///IP target where the traffic should be routed to. It is only possible to use the (Public or vSwitch) IPs of Hetzner Online Root Servers belonging to the project owner. IPs belonging to other users are blocked. Additionally IPs belonging to services provided by Hetzner Cloud (Servers, Load Balancers, ...) are blocked as well. Only present for target type `ip`.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerTargetIp {
        ///IP of a server that belongs to the same customer (public IPv4/IPv6) or private IP in a subnet type vswitch.
        pub ip: ::std::string::String,
    }
    ///Label selector used to determine targets. Only present for target type "label_selector".
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerTargetLabelSelector {
        ///Label selector.
        pub selector: LoadBalancerTargetLabelSelectorSelector,
    }
    ///Label selector.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerTargetLabelSelectorSelector(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerTargetLabelSelectorSelector {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LoadBalancerTargetLabelSelectorSelector> for ::std::string::String {
        fn from(value: LoadBalancerTargetLabelSelectorSelector) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for LoadBalancerTargetLabelSelectorSelector {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 1000usize {
                return Err("longer than 1000 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for LoadBalancerTargetLabelSelectorSelector {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerTargetLabelSelectorSelector {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LoadBalancerTargetLabelSelectorSelector {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Server where the traffic should be routed to. Only present for target type "server".
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerTargetServer {
        ///ID of the Server.
        pub id: i64,
        ///IP of the Server that the traffic is routed to. Either the Server's primary IPv4 or, if an IPv6 was specified when adding the target, an IPv6 within the Server's /64 subnet.
        pub ip: ::std::string::String,
    }
    ///`LoadBalancerTargetTarget`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoadBalancerTargetTarget {
        ///List of health statuses of the services on this target. Only present for target types "server" and "ip".
        pub health_status: ::std::vec::Vec<LoadBalancerTargetHealthStatusItem>,
        pub server: LoadBalancerTargetServer,
        ///Type of the resource. Here always "server".
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///Use the private network IP instead of the public IP. Only present for target types "server" and "label_selector".
        pub use_private_ip: bool,
    }
    ///`NetworkCreateRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct NetworkCreateRequest {
        /**Toggle to expose routes to the [Networks](#tag/networks) vSwitch.

        Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch in this [Network](#tag/networks). Only takes effect if a [vSwitch is setup](https://docs.hetzner.com/cloud/networks/connect-dedi-vswitch) in this [Network](#tag/networks).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expose_routes_to_vswitch: ::std::option::Option<bool>,
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.

        Must span all included subnets. Must be one of the private IPv4 ranges of RFC1918.

        Minimum network size is /24. We highly recommend that you pick a larger [Network](#tag/networks) with a /16 netmask.
        */
        pub ip_range: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the [Network](#tag/networks).
        pub name: NetworkCreateRequestName,
        ///Array of routes set in this [Network](#tag/networks).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub routes: ::std::vec::Vec<NetworkCreateRequestRoutesItem>,
        ///Array of subnets to allocate.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub subnets: ::std::vec::Vec<NetworkCreateRequestSubnetsItem>,
    }
    ///Name of the [Network](#tag/networks).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct NetworkCreateRequestName(::std::string::String);
    impl ::std::ops::Deref for NetworkCreateRequestName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<NetworkCreateRequestName> for ::std::string::String {
        fn from(value: NetworkCreateRequestName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for NetworkCreateRequestName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for NetworkCreateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for NetworkCreateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkCreateRequestName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`NetworkCreateRequestRoutesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct NetworkCreateRequestRoutesItem {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`NetworkCreateRequestSubnetsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct NetworkCreateRequestSubnetsItem {
        /**IP range of the subnet.

        Uses CIDR notation.

        Must be a subnet of the parent [Networks](#tag/networks) `ip_range`.

        Must not overlap with any other subnets or with any destinations in routes.

        Minimum network size is /30. We highly recommend that you pick a larger subnet with a /24 netmask.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` property it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: NetworkCreateRequestSubnetsItemType,
        /**ID of the robot vSwitch.

        Must only be supplied for subnets of type `vswitch`.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum NetworkCreateRequestSubnetsItemType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for NetworkCreateRequestSubnetsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for NetworkCreateRequestSubnetsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for NetworkCreateRequestSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for NetworkCreateRequestSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`NetworkUpdateRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct NetworkUpdateRequest {
        /**Toggle to expose routes to the [Networks](#tag/networks) vSwitch.

        Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch in this [Network](#tag/networks). Only takes effect if a [vSwitch is setup](https://docs.hetzner.com/cloud/networks/connect-dedi-vswitch) in this [Network](#tag/networks).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expose_routes_to_vswitch: ::std::option::Option<bool>,
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///New [Network](#tag/networks) name.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    ///The placement group the server is assigned to.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlacementGroupNullable {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Placement Group](#tag/placement-groups).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        pub name: ::std::string::String,
        ///Array of IDs of Servers that are part of this Placement Group.
        pub servers: ::std::vec::Vec<i64>,
        /**Type of Placement Group.
         */
        #[serde(rename = "type")]
        pub type_: PlacementGroupNullableType,
    }
    /**Type of Placement Group.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PlacementGroupNullableType {
        #[serde(rename = "spread")]
        Spread,
    }
    impl ::std::fmt::Display for PlacementGroupNullableType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Spread => f.write_str("spread"),
            }
        }
    }
    impl ::std::str::FromStr for PlacementGroupNullableType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "spread" => Ok(Self::Spread),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PlacementGroupNullableType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PlacementGroupNullableType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PoweroffServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PoweroffServerResponse {
        pub error: PoweroffServerResponseError,
    }
    ///`PoweroffServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PoweroffServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`PoweronServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PoweronServerResponse {
        pub error: PoweronServerResponseError,
    }
    ///`PoweronServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PoweronServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`PrimaryIPsResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIPsResponse {
        pub meta: ListMeta,
        pub primary_ips: ::std::vec::Vec<PrimaryIp>,
    }
    ///`PrimaryIp`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIp {
        /**ID of resource the [Primary IP](#tag/primary-ips) is assigned to.

        `null` if the [Primary IP](#tag/primary-ips) is not assigned.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub assignee_id: ::std::option::Option<i64>,
        /**Type of resource the [Primary IP](#tag/primary-ips) is assigned to.

        As of 23 September 2026 `unassigned` is returned if the [Primary IP](#tag/primary-ips) is not assigned.
        */
        pub assignee_type: PrimaryIpAssigneeType,
        /**Auto deletion state.

        If enabled the [Primary IP](#tag/primary-ips) will be deleted once the assigned resource gets deleted.
        */
        pub auto_delete: bool,
        ///Blocked state of the [Primary IP](#tag/primary-ips).
        pub blocked: bool,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///List of reverse DNS records.
        pub dns_ptr: ::std::vec::Vec<PrimaryIpDnsPtrItem>,
        ///ID of the [Primary IP](#tag/primary-ips).
        pub id: ::std::num::NonZeroU64,
        ///IP address.
        pub ip: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        pub location: PrimaryIpLocation,
        ///Name of the Resource. Must be unique per Project.
        pub name: PrimaryIpName,
        pub protection: PrimaryIpProtection,
        ///[Primary IP](#tag/primary-ips) type.
        #[serde(rename = "type")]
        pub type_: PrimaryIpType,
    }
    ///`PrimaryIpActionsAssignRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIpActionsAssignRequest {
        ///ID of a resource of type `assignee_type`.
        pub assignee_id: i64,
        ///Type of resource assigning the Primary IP to.
        pub assignee_type: PrimaryIpActionsAssignRequestAssigneeType,
    }
    ///Type of resource assigning the Primary IP to.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PrimaryIpActionsAssignRequestAssigneeType {
        #[serde(rename = "server")]
        Server,
    }
    impl ::std::fmt::Display for PrimaryIpActionsAssignRequestAssigneeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
            }
        }
    }
    impl ::std::str::FromStr for PrimaryIpActionsAssignRequestAssigneeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpActionsAssignRequestAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpActionsAssignRequestAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PrimaryIpActionsChangeDnsPtrRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PrimaryIpActionsChangeDnsPtrRequest {
        /**Domain Name to point to.

        PTR record content used for reverse DNS. Set to null to reset (IPv4) to the default value or remove (IPv6) the record.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr: ::std::option::Option<::std::string::String>,
        /**Single IPv4 or IPv6 address to create pointer for.
         */
        pub ip: ::std::string::String,
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PrimaryIpActionsChangeProtectionRequest {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    /**Type of resource the [Primary IP](#tag/primary-ips) is assigned to.

    As of 23 September 2026 `unassigned` is returned if the [Primary IP](#tag/primary-ips) is not assigned.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PrimaryIpAssigneeType {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "unassigned")]
        Unassigned,
    }
    impl ::std::fmt::Display for PrimaryIpAssigneeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::Unassigned => f.write_str("unassigned"),
            }
        }
    }
    impl ::std::str::FromStr for PrimaryIpAssigneeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "unassigned" => Ok(Self::Unassigned),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PrimaryIpCreateRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIpCreateRequest {
        /**ID of resource to assign the [Primary IP](#tag/primary-ips) to.

        Omitted if the [Primary IP](#tag/primary-ips) should not get assigned.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub assignee_id: ::std::option::Option<i64>,
        /**Type of resource to assign the [Primary IP](#tag/primary-ips) to.

        Omitted if the [Primary IP](#tag/primary-ips) should not get assigned.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub assignee_type: ::std::option::Option<PrimaryIpCreateRequestAssigneeType>,
        /**Auto deletion state.

        If enabled the [Primary IP](#tag/primary-ips) will be deleted once the assigned resource gets deleted.
        */
        #[serde(default)]
        pub auto_delete: bool,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**[Location](#tag/locations) ID or name the [Primary IP](#tag/primary-ips) will be bound to.

        Omit if `assignee_id`/`assignee_type` or `datacenter` are provided.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<PrimaryIpCreateRequestLocation>,
        ///Name of the Resource. Must be unique per Project.
        pub name: PrimaryIpCreateRequestName,
        ///[Primary IP](#tag/primary-ips) type.
        #[serde(rename = "type")]
        pub type_: PrimaryIpCreateRequestType,
    }
    /**Type of resource to assign the [Primary IP](#tag/primary-ips) to.

    Omitted if the [Primary IP](#tag/primary-ips) should not get assigned.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PrimaryIpCreateRequestAssigneeType {
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "unassigned")]
        Unassigned,
    }
    impl ::std::fmt::Display for PrimaryIpCreateRequestAssigneeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
                Self::Unassigned => f.write_str("unassigned"),
            }
        }
    }
    impl ::std::str::FromStr for PrimaryIpCreateRequestAssigneeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                "unassigned" => Ok(Self::Unassigned),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpCreateRequestAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpCreateRequestAssigneeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**[Location](#tag/locations) ID or name the [Primary IP](#tag/primary-ips) will be bound to.

    Omit if `assignee_id`/`assignee_type` or `datacenter` are provided.
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PrimaryIpCreateRequestLocation {
        String(::std::string::String),
        Int64(::std::num::NonZeroU64),
    }
    impl ::std::fmt::Display for PrimaryIpCreateRequestLocation {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::String(x) => x.fmt(f),
                Self::Int64(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<::std::num::NonZeroU64> for PrimaryIpCreateRequestLocation {
        fn from(value: ::std::num::NonZeroU64) -> Self {
            Self::Int64(value)
        }
    }
    ///Name of the Resource. Must be unique per Project.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrimaryIpCreateRequestName(::std::string::String);
    impl ::std::ops::Deref for PrimaryIpCreateRequestName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PrimaryIpCreateRequestName> for ::std::string::String {
        fn from(value: PrimaryIpCreateRequestName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for PrimaryIpCreateRequestName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpCreateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpCreateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryIpCreateRequestName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///[Primary IP](#tag/primary-ips) type.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PrimaryIpCreateRequestType {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
    }
    impl ::std::fmt::Display for PrimaryIpCreateRequestType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ipv4 => f.write_str("ipv4"),
                Self::Ipv6 => f.write_str("ipv6"),
            }
        }
    }
    impl ::std::str::FromStr for PrimaryIpCreateRequestType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ipv4" => Ok(Self::Ipv4),
                "ipv6" => Ok(Self::Ipv6),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpCreateRequestType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpCreateRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PrimaryIpDnsPtrItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PrimaryIpDnsPtrItem {
        /**Domain Name to point to.

        PTR record content used for reverse DNS.
        */
        pub dns_ptr: ::std::string::String,
        /**Single IPv4 or IPv6 address of the record.
         */
        pub ip: ::std::string::String,
    }
    /**[Location](#tag/locations) of the [Primary IP](#tag/primary-ips).
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PrimaryIpLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: PrimaryIpLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: PrimaryIpLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrimaryIpLocationName(::std::string::String);
    impl ::std::ops::Deref for PrimaryIpLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PrimaryIpLocationName> for ::std::string::String {
        fn from(value: PrimaryIpLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for PrimaryIpLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryIpLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrimaryIpLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for PrimaryIpLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PrimaryIpLocationNetworkZone> for ::std::string::String {
        fn from(value: PrimaryIpLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for PrimaryIpLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryIpLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Resource. Must be unique per Project.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrimaryIpName(::std::string::String);
    impl ::std::ops::Deref for PrimaryIpName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PrimaryIpName> for ::std::string::String {
        fn from(value: PrimaryIpName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for PrimaryIpName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryIpName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PrimaryIpProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///`PrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIpResponse {
        pub primary_ip: PrimaryIp,
    }
    ///[Primary IP](#tag/primary-ips) type.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PrimaryIpType {
        #[serde(rename = "ipv4")]
        Ipv4,
        #[serde(rename = "ipv6")]
        Ipv6,
    }
    impl ::std::fmt::Display for PrimaryIpType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ipv4 => f.write_str("ipv4"),
                Self::Ipv6 => f.write_str("ipv6"),
            }
        }
    }
    impl ::std::str::FromStr for PrimaryIpType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ipv4" => Ok(Self::Ipv4),
                "ipv6" => Ok(Self::Ipv6),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PrimaryIpUpdateRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct PrimaryIpUpdateRequest {
        /**Auto deletion state.

        If enabled the [Primary IP](#tag/primary-ips) will be deleted once the assigned resource gets deleted.
        */
        #[serde(default)]
        pub auto_delete: bool,
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<PrimaryIpUpdateRequestName>,
    }
    ///Name of the Resource. Must be unique per Project.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrimaryIpUpdateRequestName(::std::string::String);
    impl ::std::ops::Deref for PrimaryIpUpdateRequestName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PrimaryIpUpdateRequestName> for ::std::string::String {
        fn from(value: PrimaryIpUpdateRequestName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for PrimaryIpUpdateRequestName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PrimaryIpUpdateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PrimaryIpUpdateRequestName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryIpUpdateRequestName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`RebootServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RebootServerResponse {
        pub error: RebootServerResponseError,
    }
    ///`RebootServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RebootServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`RebuildServerRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RebuildServerRequest {
        ///ID or name of Image to rebuilt from.
        pub image: ::std::string::String,
        ///Cloud-Init user data to use during Server rebuild. This field is limited to 32KiB. If not specified, the Server's previous user_data value will be re-used (if any was set).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub user_data: ::std::option::Option<::std::string::String>,
    }
    ///`RebuildServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct RebuildServerResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<Action>,
        ///New root password when not using SSH keys.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub root_password: ::std::option::Option<::std::string::String>,
    }
    ///`RebuildServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RebuildServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`RemoveFirewallFromResourcesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RemoveFirewallFromResourcesResponse {
        pub error: RemoveFirewallFromResourcesResponseError,
    }
    ///`RemoveFirewallFromResourcesResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RemoveFirewallFromResourcesResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`RemoveFromResourcesRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RemoveFromResourcesRequest {
        ///Resources to remove the [Firewall](#tag/firewalls) from.
        pub remove_from: ::std::vec::Vec<FirewallResource>,
    }
    ///`RemoveServerFromPlacementGroupResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RemoveServerFromPlacementGroupResponse {
        pub error: RemoveServerFromPlacementGroupResponseError,
    }
    ///`RemoveServerFromPlacementGroupResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RemoveServerFromPlacementGroupResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`RequestServerConsoleResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RequestServerConsoleResponse {
        pub action: Action,
        ///VNC password to use for this connection (this password only works in combination with a wss_url with valid token).
        pub password: ::std::string::String,
        ///URL of websocket proxy to use; this includes a token which is valid for a limited time only.
        pub wss_url: ::std::string::String,
    }
    ///`RequestServerConsoleResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RequestServerConsoleResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    /**The `root_password` key in the reply contains the new root password that will be active if the Action succeeds.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ResetServerPasswordResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<Action>,
        ///Password that will be set for this Server once the Action succeeds.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub root_password: ::std::option::Option<::std::string::String>,
    }
    ///`ResetServerPasswordResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResetServerPasswordResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`ResetServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResetServerResponse {
        pub error: ResetServerResponseError,
    }
    ///`ResetServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResetServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`Rule`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Rule {
        ///Description of the rule.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<RuleDescription>,
        /**List of permitted IPv4/IPv6 addresses for outgoing traffic.

        The `direction` must be set to `out`.

        IPs must be in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR
        blocks at most.

        The CIDR blocks may refer to networks (with empty host bits) or single hosts.
        For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`,
        and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`.

        Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.

        IPv6 CIDRs will be transformed to their canonical form according to [RFC5952](https://datatracker.ietf.org/doc/html/rfc5952#section-4).
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub destination_ips: ::std::vec::Vec<::std::string::String>,
        /**Traffic direction in which the rule should be applied to.

        Use `source_ips` for direction `in` and `destination_ips` for direction `out` to specify IPs.
        */
        pub direction: RuleDirection,
        /**Port or port range to apply the rule for.

        Only applicable for protocols `tcp` and `udp`.

        A port range can be specified by separating lower and upper bounds with a dash. `1024-5000` will include
        all ports starting from 1024 up to port 5000.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub port: ::std::option::Option<::std::string::String>,
        ///Network protocol to apply the rule for.
        pub protocol: RuleProtocol,
        /**List of permitted IPv4/IPv6 addresses for incoming traffic.

        The `direction` must be set to `in`.

        IPs must be provided in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR
        blocks at most.

        The CIDR blocks may refer to networks (with empty host bits) or single hosts.
        For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`,
        and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`.

        Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.

        IPv6 CIDRs will be transformed to their canonical form according to [RFC5952](https://datatracker.ietf.org/doc/html/rfc5952#section-4).
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub source_ips: ::std::vec::Vec<::std::string::String>,
    }
    ///Description of the rule.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RuleDescription(::std::string::String);
    impl ::std::ops::Deref for RuleDescription {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<RuleDescription> for ::std::string::String {
        fn from(value: RuleDescription) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for RuleDescription {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleDescription {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleDescription {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleDescription {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**Traffic direction in which the rule should be applied to.

    Use `source_ips` for direction `in` and `destination_ips` for direction `out` to specify IPs.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleDirection {
        #[serde(rename = "in")]
        In,
        #[serde(rename = "out")]
        Out,
    }
    impl ::std::fmt::Display for RuleDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::In => f.write_str("in"),
                Self::Out => f.write_str("out"),
            }
        }
    }
    impl ::std::str::FromStr for RuleDirection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "in" => Ok(Self::In),
                "out" => Ok(Self::Out),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Network protocol to apply the rule for.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
        #[serde(rename = "icmp")]
        Icmp,
        #[serde(rename = "esp")]
        Esp,
        #[serde(rename = "gre")]
        Gre,
    }
    impl ::std::fmt::Display for RuleProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
                Self::Udp => f.write_str("udp"),
                Self::Icmp => f.write_str("icmp"),
                Self::Esp => f.write_str("esp"),
                Self::Gre => f.write_str("gre"),
            }
        }
    }
    impl ::std::str::FromStr for RuleProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                "udp" => Ok(Self::Udp),
                "icmp" => Ok(Self::Icmp),
                "esp" => Ok(Self::Esp),
                "gre" => Ok(Self::Gre),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`RuleResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RuleResponse {
        ///Description of the rule.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<RuleResponseDescription>,
        /**List of permitted IPv4/IPv6 addresses for outgoing traffic.

        The `direction` must be set to `out`.

        IPs must be in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR
        blocks at most.

        The CIDR blocks may refer to networks (with empty host bits) or single hosts.
        For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`,
        and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`.

        Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.

        IPv6 CIDRs will be transformed to their canonical form according to [RFC5952](https://datatracker.ietf.org/doc/html/rfc5952#section-4).
        */
        pub destination_ips: ::std::vec::Vec<::std::string::String>,
        /**Traffic direction in which the rule should be applied to.

        Use `source_ips` for direction `in` and `destination_ips` for direction `out` to specify IPs.
        */
        pub direction: RuleResponseDirection,
        /**Port or port range to apply the rule for.

        Only applicable for protocols `tcp` and `udp`.

        A port range can be specified by separating lower and upper bounds with a dash. `1024-5000` will include
        all ports starting from 1024 up to port 5000.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub port: ::std::option::Option<::std::string::String>,
        ///Network protocol to apply the rule for.
        pub protocol: RuleResponseProtocol,
        /**List of permitted IPv4/IPv6 addresses for incoming traffic.

        The `direction` must be set to `in`.

        IPs must be provided in [CIDR block notation](https://wikipedia.org/wiki/CIDR). You can specify 100 CIDR
        blocks at most.

        The CIDR blocks may refer to networks (with empty host bits) or single hosts.
        For example, a network could be defined with `10.0.1.0/24` or `2001:db8:ff00:42::/64`,
        and a single host with `10.0.1.1/32` or `2001:db8:ff00:42::8329/128`.

        Use `0.0.0.0/0` to allow any IPv4 addresses and `::/0` to allow any IPv6 addresses.

        IPv6 CIDRs will be transformed to their canonical form according to [RFC5952](https://datatracker.ietf.org/doc/html/rfc5952#section-4).
        */
        pub source_ips: ::std::vec::Vec<::std::string::String>,
    }
    ///Description of the rule.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RuleResponseDescription(::std::string::String);
    impl ::std::ops::Deref for RuleResponseDescription {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<RuleResponseDescription> for ::std::string::String {
        fn from(value: RuleResponseDescription) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for RuleResponseDescription {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleResponseDescription {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleResponseDescription {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleResponseDescription {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**Traffic direction in which the rule should be applied to.

    Use `source_ips` for direction `in` and `destination_ips` for direction `out` to specify IPs.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleResponseDirection {
        #[serde(rename = "in")]
        In,
        #[serde(rename = "out")]
        Out,
    }
    impl ::std::fmt::Display for RuleResponseDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::In => f.write_str("in"),
                Self::Out => f.write_str("out"),
            }
        }
    }
    impl ::std::str::FromStr for RuleResponseDirection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "in" => Ok(Self::In),
                "out" => Ok(Self::Out),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleResponseDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleResponseDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Network protocol to apply the rule for.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleResponseProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
        #[serde(rename = "icmp")]
        Icmp,
        #[serde(rename = "esp")]
        Esp,
        #[serde(rename = "gre")]
        Gre,
    }
    impl ::std::fmt::Display for RuleResponseProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
                Self::Udp => f.write_str("udp"),
                Self::Icmp => f.write_str("icmp"),
                Self::Esp => f.write_str("esp"),
                Self::Gre => f.write_str("gre"),
            }
        }
    }
    impl ::std::str::FromStr for RuleResponseProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                "udp" => Ok(Self::Udp),
                "icmp" => Ok(Self::Icmp),
                "esp" => Ok(Self::Esp),
                "gre" => Ok(Self::Gre),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RuleResponseProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RuleResponseProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ServerChangeDnsPtrRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServerChangeDnsPtrRequest {
        /**Domain Name to point to.

        PTR record content used for reverse DNS. Set to null to reset (IPv4) to the default value or remove (IPv6) the record.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr: ::std::option::Option<::std::string::String>,
        /**Single IPv4 or IPv6 address to create pointer for.
         */
        pub ip: ::std::string::String,
    }
    ///`ServerPublicNetFirewall`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ServerPublicNetFirewall {
        ///ID of the [Firewall](#tag/firewalls).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///Status of the Firewall on the Server.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<ServerPublicNetFirewallStatus>,
    }
    ///Status of the Firewall on the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ServerPublicNetFirewallStatus {
        #[serde(rename = "applied")]
        Applied,
        #[serde(rename = "pending")]
        Pending,
    }
    impl ::std::fmt::Display for ServerPublicNetFirewallStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Applied => f.write_str("applied"),
                Self::Pending => f.write_str("pending"),
            }
        }
    }
    impl ::std::str::FromStr for ServerPublicNetFirewallStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "applied" => Ok(Self::Applied),
                "pending" => Ok(Self::Pending),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ServerPublicNetFirewallStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ServerPublicNetFirewallStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ServiceHttpProtocol`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServiceHttpProtocol {
        ///Port the Load Balancer will balance to.
        pub destination_port: i64,
        pub health_check: LoadBalancerServiceHealthCheck,
        pub http: LoadBalancerServiceHttpConfig,
        ///Port the Load Balancer listens on.
        pub listen_port: i64,
        pub protocol: ServiceHttpProtocolProtocol,
        ///Is Proxyprotocol enabled or not.
        pub proxyprotocol: bool,
    }
    ///`ServiceHttpProtocolProtocol`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ServiceHttpProtocolProtocol {
        #[serde(rename = "http")]
        Http,
    }
    impl ::std::fmt::Display for ServiceHttpProtocolProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Http => f.write_str("http"),
            }
        }
    }
    impl ::std::str::FromStr for ServiceHttpProtocolProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "http" => Ok(Self::Http),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ServiceHttpProtocolProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ServiceHttpProtocolProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ServiceHttpsProtocol`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServiceHttpsProtocol {
        ///Port the Load Balancer will balance to.
        pub destination_port: i64,
        pub health_check: LoadBalancerServiceHealthCheck,
        pub http: LoadBalancerServiceHttpsConfig,
        ///Port the Load Balancer listens on.
        pub listen_port: i64,
        pub protocol: ServiceHttpsProtocolProtocol,
        ///Is Proxyprotocol enabled or not.
        pub proxyprotocol: bool,
    }
    ///`ServiceHttpsProtocolProtocol`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ServiceHttpsProtocolProtocol {
        #[serde(rename = "https")]
        Https,
    }
    impl ::std::fmt::Display for ServiceHttpsProtocolProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Https => f.write_str("https"),
            }
        }
    }
    impl ::std::str::FromStr for ServiceHttpsProtocolProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "https" => Ok(Self::Https),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ServiceHttpsProtocolProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ServiceHttpsProtocolProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ServiceTcp`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServiceTcp {
        ///Port the Load Balancer will balance to.
        pub destination_port: i64,
        pub health_check: LoadBalancerServiceHealthCheck,
        ///Port the Load Balancer listens on.
        pub listen_port: i64,
        pub protocol: ServiceTcpProtocol,
        ///Is Proxyprotocol enabled or not.
        pub proxyprotocol: bool,
    }
    ///`ServiceTcpProtocol`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ServiceTcpProtocol {
        #[serde(rename = "tcp")]
        Tcp,
    }
    impl ::std::fmt::Display for ServiceTcpProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
            }
        }
    }
    impl ::std::str::FromStr for ServiceTcpProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ServiceTcpProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ServiceTcpProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`SetFirewallRulesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SetFirewallRulesResponse {
        pub error: SetFirewallRulesResponseError,
    }
    ///`SetFirewallRulesResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SetFirewallRulesResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`SetRulesRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SetRulesRequest {
        /**Array of rules.

        Rules are limited to 50 entries per [Firewall](#tag/firewalls) and [500 effective rules](https://docs.hetzner.com/cloud/firewalls/overview#limits).

        Existing rules will be replaced.
        */
        pub rules: ::std::vec::Vec<Rule>,
    }
    ///`ShutdownServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ShutdownServerResponse {
        pub error: ShutdownServerResponseError,
    }
    ///`ShutdownServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ShutdownServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`TargetTypeIp`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetTypeIp {
        ///List of health statuses of the services on this target. Only present for target types "server" and "ip".
        pub health_status: ::std::vec::Vec<LoadBalancerTargetHealthStatusItem>,
        pub ip: LoadBalancerTargetIp,
        #[serde(rename = "type")]
        pub type_: TargetTypeIpType,
    }
    ///`TargetTypeIpType`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TargetTypeIpType {
        #[serde(rename = "ip")]
        Ip,
    }
    impl ::std::fmt::Display for TargetTypeIpType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ip => f.write_str("ip"),
            }
        }
    }
    impl ::std::str::FromStr for TargetTypeIpType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ip" => Ok(Self::Ip),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TargetTypeIpType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for TargetTypeIpType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`TargetTypeLabelSelector`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetTypeLabelSelector {
        pub label_selector: LoadBalancerTargetLabelSelector,
        ///List of resolved label selector target Servers.
        pub targets: ::std::vec::Vec<LoadBalancerTargetTarget>,
        #[serde(rename = "type")]
        pub type_: TargetTypeLabelSelectorType,
        ///Use the private network IP instead of the public IP. Only present for target types "server" and "label_selector".
        pub use_private_ip: bool,
    }
    ///`TargetTypeLabelSelectorType`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TargetTypeLabelSelectorType {
        #[serde(rename = "label_selector")]
        LabelSelector,
    }
    impl ::std::fmt::Display for TargetTypeLabelSelectorType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::LabelSelector => f.write_str("label_selector"),
            }
        }
    }
    impl ::std::str::FromStr for TargetTypeLabelSelectorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "label_selector" => Ok(Self::LabelSelector),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TargetTypeLabelSelectorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for TargetTypeLabelSelectorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`TargetTypeServer`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetTypeServer {
        ///List of health statuses of the services on this target. Only present for target types "server" and "ip".
        pub health_status: ::std::vec::Vec<LoadBalancerTargetHealthStatusItem>,
        pub server: LoadBalancerTargetServer,
        #[serde(rename = "type")]
        pub type_: TargetTypeServerType,
        ///Use the private network IP instead of the public IP. Only present for target types "server" and "label_selector".
        pub use_private_ip: bool,
    }
    ///`TargetTypeServerType`
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TargetTypeServerType {
        #[serde(rename = "server")]
        Server,
    }
    impl ::std::fmt::Display for TargetTypeServerType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Server => f.write_str("server"),
            }
        }
    }
    impl ::std::str::FromStr for TargetTypeServerType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "server" => Ok(Self::Server),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TargetTypeServerType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for TargetTypeServerType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`UnassignPrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UnassignPrimaryIpResponse {
        pub error: UnassignPrimaryIpResponseError,
    }
    ///`UnassignPrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UnassignPrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateFirewallRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateFirewallRequest {
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**Name of the [Firewall](#tag/firewalls).

        Must be unique per Project.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<UpdateFirewallRequestName>,
    }
    /**Name of the [Firewall](#tag/firewalls).

    Must be unique per Project.
    */
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateFirewallRequestName(::std::string::String);
    impl ::std::ops::Deref for UpdateFirewallRequestName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UpdateFirewallRequestName> for ::std::string::String {
        fn from(value: UpdateFirewallRequestName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for UpdateFirewallRequestName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateFirewallRequestName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateFirewallRequestName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateFirewallRequestName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`UpdateFirewallResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateFirewallResponse {
        pub error: UpdateFirewallResponseError,
    }
    ///`UpdateFirewallResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateFirewallResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateImageRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateImageRequest {
        ///New description of Image.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Destination Image type to convert to.
        #[serde(
            rename = "type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<UpdateImageRequestType>,
    }
    ///Destination Image type to convert to.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateImageRequestType {
        #[serde(rename = "snapshot")]
        Snapshot,
    }
    impl ::std::fmt::Display for UpdateImageRequestType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Snapshot => f.write_str("snapshot"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateImageRequestType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "snapshot" => Ok(Self::Snapshot),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateImageRequestType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateImageRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`UpdateImageResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateImageResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<UpdateImageResponseImage>,
    }
    ///`UpdateImageResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateImageResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateImageResponseImage`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateImageResponseImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: UpdateImageResponseImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<UpdateImageResponseImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: UpdateImageResponseImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: UpdateImageResponseImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: UpdateImageResponseImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: UpdateImageResponseImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateImageResponseImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for UpdateImageResponseImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateImageResponseImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateImageResponseImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateImageResponseImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateImageResponseImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for UpdateImageResponseImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateImageResponseImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateImageResponseImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateImageResponseImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateImageResponseImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for UpdateImageResponseImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateImageResponseImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateImageResponseImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateImageResponseImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for UpdateImageResponseImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateImageResponseImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateImageResponseImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`UpdateNetworkResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateNetworkResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<UpdateNetworkResponseNetwork>,
    }
    ///`UpdateNetworkResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateNetworkResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateNetworkResponseNetwork`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateNetworkResponseNetwork {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Indicates if the routes from this [Network](#tag/networks) should be exposed to the vSwitch connection.
        pub expose_routes_to_vswitch: bool,
        ///ID of the [Network](#tag/networks).
        pub id: i64,
        /**IP range of the [Network](#tag/networks).

        Uses CIDR notation.
        */
        pub ip_range: ::std::string::String,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Array of IDs of [Load Balancers](#tag/load-balancers) attached to this [Network](#tag/networks).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        ///Name of the [Network](#tag/networks).
        pub name: UpdateNetworkResponseNetworkName,
        pub protection: UpdateNetworkResponseNetworkProtection,
        ///Array of routes set in this [Network](#tag/networks).
        pub routes: ::std::vec::Vec<UpdateNetworkResponseNetworkRoutesItem>,
        ///Array of IDs of [Servers](#tag/servers) attached to this [Network](#tag/networks).
        pub servers: ::std::vec::Vec<i64>,
        ///List of subnets allocated in this [Network](#tag/networks).
        pub subnets: ::std::vec::Vec<UpdateNetworkResponseNetworkSubnetsItem>,
    }
    ///Name of the [Network](#tag/networks).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateNetworkResponseNetworkName(::std::string::String);
    impl ::std::ops::Deref for UpdateNetworkResponseNetworkName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UpdateNetworkResponseNetworkName> for ::std::string::String {
        fn from(value: UpdateNetworkResponseNetworkName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for UpdateNetworkResponseNetworkName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateNetworkResponseNetworkName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateNetworkResponseNetworkName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateNetworkResponseNetworkProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///`UpdateNetworkResponseNetworkRoutesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateNetworkResponseNetworkRoutesItem {
        /**Destination network or host of the route.

        Packages addressed for IPs matching the destination IP prefix will be send to the specified gateway.

        Must be one of
        * private IPv4 ranges of RFC1918
        * or `0.0.0.0/0`.

        Must not overlap with
        * an existing ip_range in any subnets
        * or with any destinations in other routes
        * or with `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub destination: ::std::string::String,
        /**Gateway of the route.

        Packages addressed for the specified destination will be send to this IP address.

        Cannot be
        * the first IP of the networks ip_range,
        * an IP behind a vSwitch or
        * `172.31.1.1`.

        `172.31.1.1` is being used as a gateway for the public network interface of [Servers](#tag/servers).
        */
        pub gateway: ::std::string::String,
    }
    ///`UpdateNetworkResponseNetworkSubnetsItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateNetworkResponseNetworkSubnetsItem {
        /**Gateway for [Servers](#tag/servers) attached to this subnet.

        For subnets of type `server` this is always the first IP of the subnets IP range.
        */
        pub gateway: ::std::string::String,
        /**IP range of the subnet.

        Uses CIDR notation.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip_range: ::std::option::Option<::std::string::String>,
        /**Name of the [Network Zone](#tag/network-zones).

        The [Location](#tag/locations) contains the `network_zone` property it belongs to.
        */
        pub network_zone: ::std::string::String,
        /**Type of subnet.
         */
        #[serde(rename = "type")]
        pub type_: UpdateNetworkResponseNetworkSubnetsItemType,
        ///ID of the robot vSwitch if the subnet is of type `vswitch`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vswitch_id: ::std::option::Option<i64>,
    }
    /**Type of subnet.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateNetworkResponseNetworkSubnetsItemType {
        #[serde(rename = "cloud")]
        Cloud,
        #[serde(rename = "server")]
        Server,
        #[serde(rename = "vswitch")]
        Vswitch,
    }
    impl ::std::fmt::Display for UpdateNetworkResponseNetworkSubnetsItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cloud => f.write_str("cloud"),
                Self::Server => f.write_str("server"),
                Self::Vswitch => f.write_str("vswitch"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateNetworkResponseNetworkSubnetsItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "cloud" => Ok(Self::Cloud),
                "server" => Ok(Self::Server),
                "vswitch" => Ok(Self::Vswitch),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateNetworkResponseNetworkSubnetsItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateNetworkResponseNetworkSubnetsItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`UpdatePrimaryIpResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdatePrimaryIpResponse {
        pub error: UpdatePrimaryIpResponseError,
    }
    ///`UpdatePrimaryIpResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdatePrimaryIpResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateServerRequest`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateServerRequest {
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///New name to set.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    ///`UpdateServerResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateServerResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub server: ::std::option::Option<UpdateServerResponseServer>,
    }
    ///`UpdateServerResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateServerResponseServer`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServer {
        ///Time window (UTC) in which the backup will run, or null if the backups are not enabled.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub backup_window: ::std::option::Option<::std::string::String>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Server](#tag/servers).
        pub id: ::std::num::NonZeroU64,
        ///Image the server is based on.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image: ::std::option::Option<UpdateServerResponseServerImage>,
        ///Free Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub included_traffic: ::std::option::Option<i64>,
        ///Inbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ingoing_traffic: ::std::option::Option<i64>,
        ///ISO Image that is attached to this Server. Null if no ISO is attached.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub iso: ::std::option::Option<UpdateServerResponseServerIso>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Load Balancer IDs assigned to the server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub load_balancers: ::std::vec::Vec<i64>,
        pub location: UpdateServerResponseServerLocation,
        ///True if Server has been locked and is not available to user.
        pub locked: bool,
        ///Name of the Server (must be unique per Project and a valid hostname as per RFC 1123).
        pub name: ::std::string::String,
        ///Outbound Traffic for the current billing period in bytes.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub outgoing_traffic: ::std::option::Option<i64>,
        ///The placement group the server is assigned to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub placement_group: ::std::option::Option<PlacementGroupNullable>,
        ///Size of the primary Disk.
        pub primary_disk_size: f64,
        ///Private networks information.
        pub private_net: ::std::vec::Vec<UpdateServerResponseServerPrivateNetItem>,
        pub protection: UpdateServerResponseServerProtection,
        pub public_net: UpdateServerResponseServerPublicNet,
        ///True if rescue mode is enabled. Server will then boot into rescue system on next reboot.
        pub rescue_enabled: bool,
        pub server_type: UpdateServerResponseServerServerType,
        ///Status of the Server.
        pub status: UpdateServerResponseServerStatus,
        ///IDs of Volumes assigned to this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub volumes: ::std::vec::Vec<i64>,
    }
    ///Image the server is based on.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerImage {
        /**CPU architecture compatible with the Image.
         */
        pub architecture: UpdateServerResponseServerImageArchitecture,
        ///ID of Server the Image is bound to. Only set for Images of type `backup`.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub bound_to: ::std::option::Option<i64>,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Information about the Server the Image was created from.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub created_from: ::std::option::Option<UpdateServerResponseServerImageCreatedFrom>,
        ///Point in time where the Image was deleted (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deleted: ::std::option::Option<::std::string::String>,
        #[doc = "**Deprecated:** This field is deprecated, use the deprecation object instead.\n\nPoint in time when the Image is considered to be deprecated (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).\n"]
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Image.
        pub description: ::std::string::String,
        ///Size of the disk contained in the Image in GB.
        pub disk_size: f64,
        ///ID of the [Image](#tag/images).
        pub id: ::std::num::NonZeroU64,
        ///Size of the Image file in our storage in GB. For snapshot Images this is the value relevant for calculating costs for the Image.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub image_size: ::std::option::Option<f64>,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Unique identifier of the Image. This value is only set for system Images.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Flavor of operating system contained in the Image.
        pub os_flavor: UpdateServerResponseServerImageOsFlavor,
        ///Operating system version.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub os_version: ::std::option::Option<::std::string::String>,
        pub protection: UpdateServerResponseServerImageProtection,
        ///Indicates that rapid deploy of the Image is available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rapid_deploy: ::std::option::Option<bool>,
        ///Status of the Image.
        pub status: UpdateServerResponseServerImageStatus,
        ///Type of the Image.
        #[serde(rename = "type")]
        pub type_: UpdateServerResponseServerImageType,
    }
    /**CPU architecture compatible with the Image.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerImageArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerImageArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerImageArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerImageArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateServerResponseServerImageArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about the Server the Image was created from.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerImageCreatedFrom {
        ///ID of the Server the Image was created from.
        pub id: i64,
        ///Server name at the time the Image was created.
        pub name: ::std::string::String,
    }
    ///Flavor of operating system contained in the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerImageOsFlavor {
        #[serde(rename = "ubuntu")]
        Ubuntu,
        #[serde(rename = "centos")]
        Centos,
        #[serde(rename = "debian")]
        Debian,
        #[serde(rename = "fedora")]
        Fedora,
        #[serde(rename = "rocky")]
        Rocky,
        #[serde(rename = "alma")]
        Alma,
        #[serde(rename = "opensuse")]
        Opensuse,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerImageOsFlavor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ubuntu => f.write_str("ubuntu"),
                Self::Centos => f.write_str("centos"),
                Self::Debian => f.write_str("debian"),
                Self::Fedora => f.write_str("fedora"),
                Self::Rocky => f.write_str("rocky"),
                Self::Alma => f.write_str("alma"),
                Self::Opensuse => f.write_str("opensuse"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerImageOsFlavor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ubuntu" => Ok(Self::Ubuntu),
                "centos" => Ok(Self::Centos),
                "debian" => Ok(Self::Debian),
                "fedora" => Ok(Self::Fedora),
                "rocky" => Ok(Self::Rocky),
                "alma" => Ok(Self::Alma),
                "opensuse" => Ok(Self::Opensuse),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerImageOsFlavor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerImageProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Status of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerImageStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerImageStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Available => f.write_str("available"),
                Self::Creating => f.write_str("creating"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerImageStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "available" => Ok(Self::Available),
                "creating" => Ok(Self::Creating),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerImageStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the Image.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerImageType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "snapshot")]
        Snapshot,
        #[serde(rename = "backup")]
        Backup,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerImageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::App => f.write_str("app"),
                Self::Snapshot => f.write_str("snapshot"),
                Self::Backup => f.write_str("backup"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerImageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "app" => Ok(Self::App),
                "snapshot" => Ok(Self::Snapshot),
                "backup" => Ok(Self::Backup),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerImageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///ISO Image that is attached to this Server. Null if no ISO is attached.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerIso {
        /**CPU architecture compatible with the ISO.

        Null indicates no restriction on the architecture (wildcard).
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub architecture: ::std::option::Option<UpdateServerResponseServerIsoArchitecture>,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the ISO.
        pub description: ::std::string::String,
        ///ID of the [ISO](#tag/isos).
        pub id: ::std::num::NonZeroU64,
        ///Unique identifier of the ISO. Only set for public ISOs.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Type of the ISO.
        #[serde(
            rename = "type",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub type_: ::std::option::Option<UpdateServerResponseServerIsoType>,
    }
    /**CPU architecture compatible with the ISO.

    Null indicates no restriction on the architecture (wildcard).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerIsoArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerIsoArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerIsoArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerIsoArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of the ISO.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerIsoType {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerIsoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => f.write_str("public"),
                Self::Private => f.write_str("private"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerIsoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerIsoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Location this Resource is located at.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerLocation {
        /**Name of the closest city to the [Location](#tag/locations).

        City name or city name and state in short form. E.g. `Falkenstein` or `Ashburn, VA`.
        */
        pub city: ::std::string::String,
        /**Country the [Location](#tag/locations) resides in.

        ISO 3166-1 alpha-2 code of the country.
        */
        pub country: ::std::string::String,
        ///Human readable description of the [Location](#tag/locations).
        pub description: ::std::string::String,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Latitude of the city closest to the [Location](#tag/locations).
        pub latitude: f64,
        ///Longitude of the city closest to the [Location](#tag/locations).
        pub longitude: f64,
        ///Unique identifier of the [Location](#tag/locations).
        pub name: UpdateServerResponseServerLocationName,
        ///Name of the Network Zone this [Location](#tag/locations) resides in.
        pub network_zone: UpdateServerResponseServerLocationNetworkZone,
    }
    ///Unique identifier of the [Location](#tag/locations).
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateServerResponseServerLocationName(::std::string::String);
    impl ::std::ops::Deref for UpdateServerResponseServerLocationName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UpdateServerResponseServerLocationName> for ::std::string::String {
        fn from(value: UpdateServerResponseServerLocationName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerLocationName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateServerResponseServerLocationName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Name of the Network Zone this [Location](#tag/locations) resides in.
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateServerResponseServerLocationNetworkZone(::std::string::String);
    impl ::std::ops::Deref for UpdateServerResponseServerLocationNetworkZone {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UpdateServerResponseServerLocationNetworkZone> for ::std::string::String {
        fn from(value: UpdateServerResponseServerLocationNetworkZone) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerLocationNetworkZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9]+(-?[a-z0-9]*)*$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z0-9]+(-?[a-z0-9]*)*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerLocationNetworkZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateServerResponseServerLocationNetworkZone
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateServerResponseServerLocationNetworkZone {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`UpdateServerResponseServerPrivateNetItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateServerResponseServerPrivateNetItem {
        ///Additional IP addresses of the server on the network.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alias_ips: ::std::vec::Vec<::std::string::String>,
        ///The server IP address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::string::String>,
        ///The server MAC address on the network.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub mac_address: ::std::option::Option<::std::string::String>,
        ///The Network ID the server is attached to.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub network: ::std::option::Option<i64>,
    }
    ///Protection configuration for the Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerProtection {
        ///If true, prevents the Server from being deleted.
        pub delete: bool,
        ///If true, prevents the Server from being rebuilt.
        pub rebuild: bool,
    }
    ///Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerPublicNet {
        ///Firewalls applied to the public network interface of this Server.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub firewalls: ::std::vec::Vec<ServerPublicNetFirewall>,
        ///IDs of Floating IPs assigned to this Server.
        pub floating_ips: ::std::vec::Vec<i64>,
        ///IP address (v4) and its reverse DNS entry of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv4: ::std::option::Option<UpdateServerResponseServerPublicNetIpv4>,
        ///IPv6 network assigned to this Server and its reverse DNS entry.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ipv6: ::std::option::Option<UpdateServerResponseServerPublicNetIpv6>,
    }
    ///IP address (v4) and its reverse DNS entry of this Server.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerPublicNetIpv4 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entry for the IPv4 addresses of this Server.
        pub dns_ptr: ::std::string::String,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v4) of this Server.
        pub ip: ::std::string::String,
    }
    ///IPv6 network assigned to this Server and its reverse DNS entry.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerPublicNetIpv6 {
        ///Whether the IP is blocked by our abuse department.
        pub blocked: bool,
        ///Reverse DNS PTR entries for the IPv6 addresses of this Server.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub dns_ptr: ::std::option::Option<
            ::std::vec::Vec<UpdateServerResponseServerPublicNetIpv6DnsPtrItem>,
        >,
        ///ID of the [Primary IP](#tag/primary-ips).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::num::NonZeroU64>,
        ///IP address (v6) of this Server.
        pub ip: ::std::string::String,
    }
    ///`UpdateServerResponseServerPublicNetIpv6DnsPtrItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerPublicNetIpv6DnsPtrItem {
        ///DNS pointer for the specific IP address.
        pub dns_ptr: ::std::string::String,
        ///Single IPv6 address of this Server for which the reverse DNS entry has been set up.
        pub ip: ::std::string::String,
    }
    ///`UpdateServerResponseServerServerType`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerType {
        /**CPU architecture of the Server Type.
         */
        pub architecture: UpdateServerResponseServerServerTypeArchitecture,
        ///Category of Server Type.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        ///Number of cpu cores a Server of this type will have.
        pub cores: f64,
        ///Type of cpu.
        pub cpu_type: UpdateServerResponseServerServerTypeCpuType,
        ///This field is deprecated. Use the deprecation object instead.
        pub deprecated: bool,
        /**This field is deprecated.

        Use the `deprecation` object in the `locations` field instead (`.locations[].deprecation`).
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///Description of the Server type.
        pub description: ::std::string::String,
        ///Disk size a Server of this type will have in GB.
        pub disk: f64,
        ///ID of the Server type.
        pub id: i64,
        /**Supported [Location](#tag/locations) and per [Location](#tag/locations) details for the [Server Type](#tag/server-types).

        A [Server Type](#tag/server-types) is:
         - only supported in the [Location](#tag/locations) that are listed.
         - deprecated in the [Location](#tag/locations) when the `deprecation` property is set.
         - permanently unavailable in the [Location](#tag/locations) when the `deprecation.unavailable_after` date is in the past.
         - temporarily unavailable in the [Location](#tag/locations) when the `deprecation.available` is false.
        */
        pub locations: ::std::vec::Vec<UpdateServerResponseServerServerTypeLocationsItem>,
        ///Memory a Server of this type will have in GB.
        pub memory: f64,
        ///Unique identifier of the Server type.
        pub name: ::std::string::String,
        ///Price per [Location](#tag/locations).
        pub prices: ::std::vec::Vec<UpdateServerResponseServerServerTypePricesItem>,
        ///Type of Server boot drive. Local has higher speed. Network has better availability.
        pub storage_type: UpdateServerResponseServerServerTypeStorageType,
    }
    /**CPU architecture of the Server Type.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerServerTypeArchitecture {
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerServerTypeArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X86 => f.write_str("x86"),
                Self::Arm => f.write_str("arm"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerServerTypeArchitecture {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x86" => Ok(Self::X86),
                "arm" => Ok(Self::Arm),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerServerTypeArchitecture {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateServerResponseServerServerTypeArchitecture
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Type of cpu.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerServerTypeCpuType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "dedicated")]
        Dedicated,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerServerTypeCpuType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Shared => f.write_str("shared"),
                Self::Dedicated => f.write_str("dedicated"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerServerTypeCpuType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "shared" => Ok(Self::Shared),
                "dedicated" => Ok(Self::Dedicated),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerServerTypeCpuType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateServerResponseServerServerTypeCpuType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A [Location](#tag/locations) (containing only id and name) and Server Type specific metadata.
     */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerTypeLocationsItem {
        /**Whether the [Server Type](#tag/server-types) is temporarily unavailable in this [Location](#tag/locations).
         */
        pub available: bool,
        /**Describes if, when and how the resource is deprecated. If this field is
        set to `null` the resource is not deprecated. If a value is set, it is
        considered deprecated.
        */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub deprecation: ::std::option::Option<DeprecationInfo>,
        ///ID of the [Location](#tag/locations).
        pub id: ::std::num::NonZeroU64,
        ///Name of the [Location](#tag/locations).
        pub name: ::std::string::String,
        /**Whether the [Server Type](#tag/server-types) is recommended in this [Location](#tag/locations).
         */
        pub recommended: bool,
    }
    ///`UpdateServerResponseServerServerTypePricesItem`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerTypePricesItem {
        ///Free traffic per month in bytes in this [Location](#tag/locations).
        pub included_traffic: i64,
        ///Name of the [Location](#tag/locations) the price is for.
        pub location: ::std::string::String,
        pub price_hourly: UpdateServerResponseServerServerTypePricesItemPriceHourly,
        pub price_monthly: UpdateServerResponseServerServerTypePricesItemPriceMonthly,
        pub price_per_tb_traffic: UpdateServerResponseServerServerTypePricesItemPricePerTbTraffic,
    }
    ///Hourly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerTypePricesItemPriceHourly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Monthly price in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerTypePricesItemPriceMonthly {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Additional traffic price per TB in this [Location](#tag/locations).
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateServerResponseServerServerTypePricesItemPricePerTbTraffic {
        ///Price with VAT added.
        pub gross: ::std::string::String,
        ///Price without VAT.
        pub net: ::std::string::String,
    }
    ///Type of Server boot drive. Local has higher speed. Network has better availability.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerServerTypeStorageType {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "network")]
        Network,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerServerTypeStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Local => f.write_str("local"),
                Self::Network => f.write_str("network"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerServerTypeStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "local" => Ok(Self::Local),
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerServerTypeStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for UpdateServerResponseServerServerTypeStorageType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Status of the Server.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpdateServerResponseServerStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "migrating")]
        Migrating,
        #[serde(rename = "rebuilding")]
        Rebuilding,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for UpdateServerResponseServerStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Initializing => f.write_str("initializing"),
                Self::Starting => f.write_str("starting"),
                Self::Stopping => f.write_str("stopping"),
                Self::Off => f.write_str("off"),
                Self::Deleting => f.write_str("deleting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Rebuilding => f.write_str("rebuilding"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for UpdateServerResponseServerStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "initializing" => Ok(Self::Initializing),
                "starting" => Ok(Self::Starting),
                "stopping" => Ok(Self::Stopping),
                "off" => Ok(Self::Off),
                "deleting" => Ok(Self::Deleting),
                "migrating" => Ok(Self::Migrating),
                "rebuilding" => Ok(Self::Rebuilding),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UpdateServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UpdateServerResponseServerStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`UpdateSshKeyBody`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct UpdateSshKeyBody {
        /**User-defined labels (`key/value` pairs) for the Resource.

        Note that the set of [Labels](#description/labels) provided in the request will overwrite the
        existing one.

        For more information, see "[Labels](#description/labels)".
        */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///New name Name to set.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    ///`UpdateSshKeyResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateSshKeyResponse {
        pub ssh_key: UpdateSshKeyResponseSshKey,
    }
    ///`UpdateSshKeyResponseError`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateSshKeyResponseError {
        ///Error code for machines.
        pub code: ::std::string::String,
        ///Details about the error.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Error message for humans.
        pub message: ::std::string::String,
    }
    ///`UpdateSshKeyResponseSshKey`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateSshKeyResponseSshKey {
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///MD5 fingerprint of the SSH public key.
        pub fingerprint: ::std::string::String,
        ///ID of the [SSH Key](#tag/ssh-keys).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name of the Resource. Must be unique per Project.
        pub name: ::std::string::String,
        ///Public key.
        pub public_key: ::std::string::String,
    }
    ///`ZonePrimary`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ZonePrimary {
        pub authoritative_nameservers: ZonePrimaryAuthoritativeNameservers,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Zone](#tag/zones).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**Mode of the [Zone](#tag/zones).

        For more information, see [Zone Modes](#tag/zones/zone-modes).
        */
        pub mode: ZonePrimaryMode,
        /**Name of the [Zone](#tag/zones).

        All names with [well-known public suffixes](https://publicsuffix.org/) (e.g. `.de`,
        `.com`, `.co.uk`) are supported. Subdomains are not supported.

        The name must be in lower case and must not end with a dot.
        [Internationalized domain
        names](https://en.wikipedia.org/wiki/Internationalized_domain_name) must be
        transcribed to [Punycode](https://wikipedia.org/wiki/Punycode) representation with
        ACE prefix, e.g. `xn--mnchen-3ya.de` (`münchen.de`).
        */
        pub name: ZonePrimaryName,
        /**Primary nameservers of the [Zone](#tag/zones).

        Only set if [Zone](#tag/zones) is in [secondary mode](#tag/zones/zone-modes), otherwise empty.
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub primary_nameservers: ::std::vec::Vec<ZonePrimaryPrimaryNameserversItem>,
        pub protection: ZonePrimaryProtection,
        ///Number of resource records (RR) within the [Zone](#tag/zones).
        pub record_count: i64,
        ///Registrar of the domain.
        pub registrar: ZonePrimaryRegistrar,
        /**Status of the [Zone](#tag/zones).
         */
        pub status: ZonePrimaryStatus,
        /**Default Time To Live (TTL) of the [Zone](#tag/zones).

        Must be in between 60s and 2147483647s.

        This TTL is used for [RRSets](#tag/zone-rrsets) that do not explicitly define a TTL.
        */
        pub ttl: i64,
    }
    ///`ZonePrimaryAuthoritativeNameservers`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ZonePrimaryAuthoritativeNameservers {
        /**Authoritative Hetzner nameservers assigned to this [Zone](#tag/zones).
         */
        pub assigned: ::std::vec::Vec<::std::string::String>,
        /**Authoritative nameservers currently delegated to by the parent DNS zone.

        If these don't match the assigned authoritative nameservers, the DNS zone is currently not being served by Hetzner.
        */
        pub delegated: ::std::vec::Vec<::std::string::String>,
        /**Point in time (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format) when the DNS zone delegation was last checked.
         */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub delegation_last_check: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Status of the delegation.
         */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub delegation_status:
            ::std::option::Option<ZonePrimaryAuthoritativeNameserversDelegationStatus>,
    }
    /**Status of the delegation.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZonePrimaryAuthoritativeNameserversDelegationStatus {
        #[serde(rename = "valid")]
        Valid,
        #[serde(rename = "partially-valid")]
        PartiallyValid,
        #[serde(rename = "invalid")]
        Invalid,
        #[serde(rename = "lame")]
        Lame,
        #[serde(rename = "unregistered")]
        Unregistered,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ZonePrimaryAuthoritativeNameserversDelegationStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Valid => f.write_str("valid"),
                Self::PartiallyValid => f.write_str("partially-valid"),
                Self::Invalid => f.write_str("invalid"),
                Self::Lame => f.write_str("lame"),
                Self::Unregistered => f.write_str("unregistered"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ZonePrimaryAuthoritativeNameserversDelegationStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "valid" => Ok(Self::Valid),
                "partially-valid" => Ok(Self::PartiallyValid),
                "invalid" => Ok(Self::Invalid),
                "lame" => Ok(Self::Lame),
                "unregistered" => Ok(Self::Unregistered),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryAuthoritativeNameserversDelegationStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ZonePrimaryAuthoritativeNameserversDelegationStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Mode of the [Zone](#tag/zones).

    For more information, see [Zone Modes](#tag/zones/zone-modes).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZonePrimaryMode {
        #[serde(rename = "primary")]
        Primary,
    }
    impl ::std::fmt::Display for ZonePrimaryMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Primary => f.write_str("primary"),
            }
        }
    }
    impl ::std::str::FromStr for ZonePrimaryMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "primary" => Ok(Self::Primary),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZonePrimaryMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Name of the [Zone](#tag/zones).

    All names with [well-known public suffixes](https://publicsuffix.org/) (e.g. `.de`,
    `.com`, `.co.uk`) are supported. Subdomains are not supported.

    The name must be in lower case and must not end with a dot.
    [Internationalized domain
    names](https://en.wikipedia.org/wiki/Internationalized_domain_name) must be
    transcribed to [Punycode](https://wikipedia.org/wiki/Punycode) representation with
    ACE prefix, e.g. `xn--mnchen-3ya.de` (`münchen.de`).
    */
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ZonePrimaryName(::std::string::String);
    impl ::std::ops::Deref for ZonePrimaryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ZonePrimaryName> for ::std::string::String {
        fn from(value: ZonePrimaryName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ZonePrimaryName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZonePrimaryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ZonePrimaryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**Primary nameserver that returns [Zones](#tag/zones) via `AXFR`.

    Must allow queries from and may send `NOTIFY` queries to [Hetzner's secondary
    nameservers](https://docs.hetzner.com/dns-console/dns/general/authoritative-name-servers#secondary-dns-servers-old-name-servers-for-robot-customers).
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ZonePrimaryPrimaryNameserversItem {
        /**Public IPv4 or IPv6 address of the primary nameserver.

        Must be unique across all primary nameservers of this zone.
        */
        pub address: ::std::string::String,
        ///Port of the primary nameserver.
        #[serde(default = "defaults::default_u64::<i64, 53>")]
        pub port: i64,
        /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) algorithm
        used to generate the TSIG key.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub tsig_algorithm: ::std::option::Option<ZonePrimaryPrimaryNameserversItemTsigAlgorithm>,
        /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) key to use
        for the zone transfer. Must be base64 encoded.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub tsig_key: ::std::option::Option<::std::string::String>,
    }
    /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) algorithm
    used to generate the TSIG key.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZonePrimaryPrimaryNameserversItemTsigAlgorithm {
        #[serde(rename = "hmac-md5")]
        HmacMd5,
        #[serde(rename = "hmac-sha1")]
        HmacSha1,
        #[serde(rename = "hmac-sha256")]
        HmacSha256,
    }
    impl ::std::fmt::Display for ZonePrimaryPrimaryNameserversItemTsigAlgorithm {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::HmacMd5 => f.write_str("hmac-md5"),
                Self::HmacSha1 => f.write_str("hmac-sha1"),
                Self::HmacSha256 => f.write_str("hmac-sha256"),
            }
        }
    }
    impl ::std::str::FromStr for ZonePrimaryPrimaryNameserversItemTsigAlgorithm {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "hmac-md5" => Ok(Self::HmacMd5),
                "hmac-sha1" => Ok(Self::HmacSha1),
                "hmac-sha256" => Ok(Self::HmacSha256),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryPrimaryNameserversItemTsigAlgorithm {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ZonePrimaryPrimaryNameserversItemTsigAlgorithm
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ZonePrimaryProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Registrar of the domain.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZonePrimaryRegistrar {
        #[serde(rename = "hetzner")]
        Hetzner,
        #[serde(rename = "other")]
        Other,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ZonePrimaryRegistrar {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Hetzner => f.write_str("hetzner"),
                Self::Other => f.write_str("other"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ZonePrimaryRegistrar {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "hetzner" => Ok(Self::Hetzner),
                "other" => Ok(Self::Other),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryRegistrar {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZonePrimaryRegistrar {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Status of the [Zone](#tag/zones).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZonePrimaryStatus {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ZonePrimaryStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
                Self::Updating => f.write_str("updating"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ZonePrimaryStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "updating" => Ok(Self::Updating),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZonePrimaryStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZonePrimaryStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ZoneSecondary`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ZoneSecondary {
        pub authoritative_nameservers: ZoneSecondaryAuthoritativeNameservers,
        ///Point in time when the Resource was created (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the [Zone](#tag/zones).
        pub id: ::std::num::NonZeroU64,
        /**User-defined labels (`key/value` pairs) for the Resource.
        For more information, see "[Labels](#description/labels)".
        */
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        /**Mode of the [Zone](#tag/zones).

        For more information, see [Zone Modes](#tag/zones/zone-modes).
        */
        pub mode: ZoneSecondaryMode,
        /**Name of the [Zone](#tag/zones).

        All names with [well-known public suffixes](https://publicsuffix.org/) (e.g. `.de`,
        `.com`, `.co.uk`) are supported. Subdomains are not supported.

        The name must be in lower case and must not end with a dot.
        [Internationalized domain
        names](https://en.wikipedia.org/wiki/Internationalized_domain_name) must be
        transcribed to [Punycode](https://wikipedia.org/wiki/Punycode) representation with
        ACE prefix, e.g. `xn--mnchen-3ya.de` (`münchen.de`).
        */
        pub name: ZoneSecondaryName,
        /**Primary nameservers of the [Zone](#tag/zones).

        Only set if [Zone](#tag/zones) is in [secondary mode](#tag/zones/zone-modes), otherwise empty.
        */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub primary_nameservers: ::std::vec::Vec<ZoneSecondaryPrimaryNameserversItem>,
        pub protection: ZoneSecondaryProtection,
        ///Number of resource records (RR) within the [Zone](#tag/zones).
        pub record_count: i64,
        ///Registrar of the domain.
        pub registrar: ZoneSecondaryRegistrar,
        /**Status of the [Zone](#tag/zones).
         */
        pub status: ZoneSecondaryStatus,
        /**Default Time To Live (TTL) of the [Zone](#tag/zones).

        Must be in between 60s and 2147483647s.

        This TTL is used for [RRSets](#tag/zone-rrsets) that do not explicitly define a TTL.
        */
        pub ttl: i64,
    }
    ///`ZoneSecondaryAuthoritativeNameservers`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ZoneSecondaryAuthoritativeNameservers {
        /**Authoritative Hetzner nameservers assigned to this [Zone](#tag/zones).
         */
        pub assigned: ::std::vec::Vec<::std::string::String>,
        /**Authoritative nameservers currently delegated to by the parent DNS zone.

        If these don't match the assigned authoritative nameservers, the DNS zone is currently not being served by Hetzner.
        */
        pub delegated: ::std::vec::Vec<::std::string::String>,
        /**Point in time (in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format) when the DNS zone delegation was last checked.
         */
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub delegation_last_check: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        /**Status of the delegation.
         */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub delegation_status:
            ::std::option::Option<ZoneSecondaryAuthoritativeNameserversDelegationStatus>,
    }
    /**Status of the delegation.
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZoneSecondaryAuthoritativeNameserversDelegationStatus {
        #[serde(rename = "valid")]
        Valid,
        #[serde(rename = "partially-valid")]
        PartiallyValid,
        #[serde(rename = "invalid")]
        Invalid,
        #[serde(rename = "lame")]
        Lame,
        #[serde(rename = "unregistered")]
        Unregistered,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ZoneSecondaryAuthoritativeNameserversDelegationStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Valid => f.write_str("valid"),
                Self::PartiallyValid => f.write_str("partially-valid"),
                Self::Invalid => f.write_str("invalid"),
                Self::Lame => f.write_str("lame"),
                Self::Unregistered => f.write_str("unregistered"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryAuthoritativeNameserversDelegationStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "valid" => Ok(Self::Valid),
                "partially-valid" => Ok(Self::PartiallyValid),
                "invalid" => Ok(Self::Invalid),
                "lame" => Ok(Self::Lame),
                "unregistered" => Ok(Self::Unregistered),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryAuthoritativeNameserversDelegationStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ZoneSecondaryAuthoritativeNameserversDelegationStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Mode of the [Zone](#tag/zones).

    For more information, see [Zone Modes](#tag/zones/zone-modes).
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZoneSecondaryMode {
        #[serde(rename = "secondary")]
        Secondary,
    }
    impl ::std::fmt::Display for ZoneSecondaryMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Secondary => f.write_str("secondary"),
            }
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "secondary" => Ok(Self::Secondary),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZoneSecondaryMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Name of the [Zone](#tag/zones).

    All names with [well-known public suffixes](https://publicsuffix.org/) (e.g. `.de`,
    `.com`, `.co.uk`) are supported. Subdomains are not supported.

    The name must be in lower case and must not end with a dot.
    [Internationalized domain
    names](https://en.wikipedia.org/wiki/Internationalized_domain_name) must be
    transcribed to [Punycode](https://wikipedia.org/wiki/Punycode) representation with
    ACE prefix, e.g. `xn--mnchen-3ya.de` (`münchen.de`).
    */
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ZoneSecondaryName(::std::string::String);
    impl ::std::ops::Deref for ZoneSecondaryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ZoneSecondaryName> for ::std::string::String {
        fn from(value: ZoneSecondaryName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 255usize {
                return Err("longer than 255 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZoneSecondaryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ZoneSecondaryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**Primary nameserver that returns [Zones](#tag/zones) via `AXFR`.

    Must allow queries from and may send `NOTIFY` queries to [Hetzner's secondary
    nameservers](https://docs.hetzner.com/dns-console/dns/general/authoritative-name-servers#secondary-dns-servers-old-name-servers-for-robot-customers).
    */
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ZoneSecondaryPrimaryNameserversItem {
        /**Public IPv4 or IPv6 address of the primary nameserver.

        Must be unique across all primary nameservers of this zone.
        */
        pub address: ::std::string::String,
        ///Port of the primary nameserver.
        #[serde(default = "defaults::default_u64::<i64, 53>")]
        pub port: i64,
        /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) algorithm
        used to generate the TSIG key.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub tsig_algorithm: ::std::option::Option<ZoneSecondaryPrimaryNameserversItemTsigAlgorithm>,
        /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) key to use
        for the zone transfer. Must be base64 encoded.
        */
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub tsig_key: ::std::option::Option<::std::string::String>,
    }
    /**[Transaction signature (TSIG)](https://en.wikipedia.org/wiki/TSIG) algorithm
    used to generate the TSIG key.
    */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZoneSecondaryPrimaryNameserversItemTsigAlgorithm {
        #[serde(rename = "hmac-md5")]
        HmacMd5,
        #[serde(rename = "hmac-sha1")]
        HmacSha1,
        #[serde(rename = "hmac-sha256")]
        HmacSha256,
    }
    impl ::std::fmt::Display for ZoneSecondaryPrimaryNameserversItemTsigAlgorithm {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::HmacMd5 => f.write_str("hmac-md5"),
                Self::HmacSha1 => f.write_str("hmac-sha1"),
                Self::HmacSha256 => f.write_str("hmac-sha256"),
            }
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryPrimaryNameserversItemTsigAlgorithm {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "hmac-md5" => Ok(Self::HmacMd5),
                "hmac-sha1" => Ok(Self::HmacSha1),
                "hmac-sha256" => Ok(Self::HmacSha256),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryPrimaryNameserversItemTsigAlgorithm {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for ZoneSecondaryPrimaryNameserversItemTsigAlgorithm
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Protection configuration for the Resource.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ZoneSecondaryProtection {
        ///Prevent the Resource from being deleted.
        pub delete: bool,
    }
    ///Registrar of the domain.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZoneSecondaryRegistrar {
        #[serde(rename = "hetzner")]
        Hetzner,
        #[serde(rename = "other")]
        Other,
        #[serde(rename = "unknown")]
        Unknown,
    }
    impl ::std::fmt::Display for ZoneSecondaryRegistrar {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Hetzner => f.write_str("hetzner"),
                Self::Other => f.write_str("other"),
                Self::Unknown => f.write_str("unknown"),
            }
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryRegistrar {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "hetzner" => Ok(Self::Hetzner),
                "other" => Ok(Self::Other),
                "unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryRegistrar {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZoneSecondaryRegistrar {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Status of the [Zone](#tag/zones).
     */
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ZoneSecondaryStatus {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "error")]
        Error,
    }
    impl ::std::fmt::Display for ZoneSecondaryStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
                Self::Updating => f.write_str("updating"),
                Self::Error => f.write_str("error"),
            }
        }
    }
    impl ::std::str::FromStr for ZoneSecondaryStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "updating" => Ok(Self::Updating),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ZoneSecondaryStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ZoneSecondaryStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: ::std::convert::TryFrom<u64>,
            <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }
        pub(super) fn create_image_request_type() -> super::CreateImageRequestType {
            super::CreateImageRequestType::Snapshot
        }
        pub(super) fn enable_server_rescue_body_type() -> super::EnableServerRescueBodyType {
            super::EnableServerRescueBodyType::Linux64
        }
        pub(super) fn load_balancer_service_health_check_http_status_codes(
        ) -> ::std::vec::Vec<::std::string::String> {
            vec!["2??".to_string(), "3??".to_string()]
        }
    }
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Hetzner Cloud API

# Overview

This is the official documentation for the Hetzner Cloud API.

## Introduction

The Hetzner Cloud API operates over HTTPS and uses JSON as its data format. The API is a RESTful API and utilizes HTTP methods and HTTP status codes to specify requests and responses.

As an alternative to working directly with our API you may also consider to use:

- Our CLI program [hcloud](https://github.com/hetznercloud/cli)
- Our [library for Go](https://github.com/hetznercloud/hcloud-go)
- Our [library for Python](https://github.com/hetznercloud/hcloud-python)

You can find even more libraries, tools and integrations on our [Awesome List on GitHub](https://github.com/hetznercloud/awesome-hcloud).

## Getting Started

To get started using the API you first need an API token. Sign in into the [Hetzner Console](https://console.hetzner.com/) choose a Project, go to `Security` → `API Tokens`, and generate a new token. Make sure to copy the token because it won’t be shown to you again. A token is bound to a Project, to interact with the API of another Project you have to create a new token inside the Project. Let’s say your new token is `LRK9DAWQ1ZAEFSrCNEEzLCUwhYX1U3g7wMg4dTlkkDC96fyDuyJ39nVbVjCKSDfj`.

You’re now ready to do your first request against the API. To get a list of all Servers in your Project, issue the example request on the right side using [curl](https://curl.se/).

Make sure to replace the token in the example command with the token you have just created. Since your Project probably does not contain any Servers yet, the example response will look like the response on the right side. We will almost always provide a resource root like `servers` inside the example response. A response can also contain a `meta` object with information like [Pagination](#description/pagination).

**Example Request**

```shell
curl -H "Authorization: Bearer LRK9DAWQ1ZAEFSrCNEEzLCUwhYX1U3g7wMg4dTlkkDC96fyDuyJ39nVbVjCKSDfj" \
  https://api.hetzner.cloud/v1/servers
```

**Example Response**

```json
{
  "servers": [],
  "meta": {
    "pagination": {
      "page": 1,
      "per_page": 25,
      "previous_page": null,
      "next_page": null,
      "last_page": 1,
      "total_entries": 0
    }
  }
}
```

## Authentication

All requests to the Hetzner Cloud API must be authenticated via a API token. Include your secret API token in every request you send to the API with the `Authorization` HTTP header.

To create a new API token for your Project, switch into the [Hetzner Console](https://console.hetzner.com/) choose a Project, go to `Security` → `API Tokens`, and generate a new token.

**Example Authorization header**

```http
Authorization: Bearer LRK9DAWQ1ZAEFSrCNEEzLCUwhYX1U3g7wMg4dTlkkDC96fyDuyJ39nVbVjCKSDfj
```

## Query Parameters

The API makes use of query parameters to sort and filter responses. The parameter names and values must be URI encoded according to [RFC2396](https://datatracker.ietf.org/doc/html/rfc2396). Query parameters of type `array` can be used multiple times:

**Example query parameters for pagination**

```
https://api.hetzner.cloud/v1/certificates?page=1&page_size=12
```

**Example use of multiple values for a parameter**

```
https://api.hetzner.cloud/v1/certificates?type=uploaded&type=managed
```

**Example use of an encoded parameter**

```
https://api.hetzner.cloud/v1/certificates?label_selector=key%3Dvalue
```

## Errors

Errors are indicated by HTTP status codes. Further, the response of the request which generated the error contains an error code, an error message, and, optionally, error details. The schema of the error details object depends on the error code.

The error response contains the following keys:

| Keys      | Meaning                                                               |
| --------- | --------------------------------------------------------------------- |
| `code`    | Short string indicating the type of error (machine-parsable)          |
| `message` | Textual description on what has gone wrong                            |
| `details` | An object providing for details on the error (schema depends on code) |

**Example response**

```json
{
  "error": {
    "code": "invalid_input",
    "message": "invalid input in field 'broken_field': is too long",
    "details": {
      "fields": [
        {
          "name": "broken_field",
          "messages": ["is too long"]
        }
      ]
    }
  }
}
```

### Error Codes

| Status | Code | Description |
| --- | --- | --- |
| `400` | `json_error` | Invalid JSON input in your request. |
| `401` | `unauthorized` | Request was made with an invalid or unknown token. |
| `403` | `forbidden` | Insufficient permissions for this request. |
| `403` | `maintenance` | Cannot perform operation due to maintenance. |
| `403` | `resource_limit_exceeded` | Error when exceeding the maximum quantity of a resource for an account. |
| `404` | `not_found` | Entity not found. |
| `405` | `method_not_allowed` | The request method is not allowed |
| `409` | `uniqueness_error` | One or more of the objects fields must be unique. |
| `409` | `conflict` | The resource has changed during the request, please retry. |
| `410` | `deprecated_api_endpoint` | The API endpoint functionality was removed. |
| `412` | `resource_unavailable` | The requested resource is currently unavailable (e.g. not available for order). |
| `422` | `invalid_input` | Error while parsing or processing the input. |
| `422` | `service_error` | Error within a service. |
| `422` | `unsupported_error` | The corresponding resource does not support the Action. |
| `423` | `locked` | The item you are trying to access is locked (there is already an Action running). |
| `423` | `protected` | The Action you are trying to start is protected for this resource. |
| `429` | `rate_limit_exceeded` | Error when sending too many requests. |
| `500` | `server_error` | Error within the API backend. |
| `502` | `bad_gateway` | The request could not be answered by the API backend, please retry. |
| `503` | `unavailable` | A service or product is currently not available. |
| `504` | `timeout` | The request could not be answered in time, please retry. |

**invalid_input**

```json
{
  "error": {
    "code": "invalid_input",
    "message": "invalid input in field 'broken_field': is too long",
    "details": {
      "fields": [
        {
          "name": "broken_field",
          "messages": ["is too long"]
        }
      ]
    }
  }
}
```

**uniqueness_error**

```json
{
  "error": {
    "code": "uniqueness_error",
    "message": "SSH key with the same fingerprint already exists",
    "details": {
      "fields": [
        {
          "name": "public_key"
        }
      ]
    }
  }
}
```

**resource_limit_exceeded**

```json
{
  "error": {
    "code": "resource_limit_exceeded",
    "message": "project limit exceeded",
    "details": {
      "limits": [
        {
          "name": "project_limit"
        }
      ]
    }
  }
}
```

**deprecated_api_endpoint**

```json
{
  "error": {
    "code": "deprecated_api_endpoint",
    "message": "API functionality was removed",
    "details": {
      "announcement": "https://docs.hetzner.cloud/changelog#2023-07-20-foo-endpoint-is-deprecated"
    }
  }
}
```

## Actions

Actions represent asynchronous tasks within the API, targeting one or more resources. Triggering changes in the API may return a `running` action.

An action should be waited upon, until it reaches either the `success` or `error` state. Avoid polling the action's state too frequently to reduce the risk of exhausting your API requests and hitting the [rate limit](#description/rate-limiting).

If an action fails, it will contain details about the underlying error.

Once the asynchronous tasks have completed and the targeted resources are in a consistent state, the action is marked as succeeded.

In some cases, you may trigger multiple changes at once, and only wait for the returned actions at a later stage.

## Labels

Labels are `key/value` pairs that can be attached to all resources.

Valid label keys have two segments: an optional prefix and name, separated by a slash (`/`). The name segment is required and must be a string of 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (`.`), not longer than 253 characters in total, followed by a slash (`/`).

Valid label values must be a string of 63 characters or less and must be empty or begin and end with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between.

The `hetzner.cloud/` prefix is reserved and cannot be used.

**Example Labels**

```json
{
  "labels": {
    "environment": "development",
    "service": "backend",
    "example.com/my": "label",
    "just-a-key": ""
  }
}
```

## Label Selector

For resources with labels, you can filter resources by their labels using the label selector query language.

| Expression           | Meaning                                              |
| -------------------- | ---------------------------------------------------- |
| `k==v` / `k=v`       | Value of key `k` does equal value `v`                |
| `k!=v`               | Value of key `k` does not equal value `v`            |
| `k`                  | Key `k` is present                                   |
| `!k`                 | Key `k` is not present                               |
| `k in (v1,v2,v3)`    | Value of key `k` is `v1`, `v2`, or `v3`              |
| `k notin (v1,v2,v3)` | Value of key `k` is neither `v1`, nor `v2`, nor `v3` |
| `k1==v,!k2`          | Value of key `k1` is `v` and key `k2` is not present |

### Examples

- Returns all resources that have a `env=production` label and that don't have a `type=database` label:

  `env=production,type!=database`

- Returns all resources that have a `env=testing` or `env=staging` label:

  `env in (testing,staging)`

- Returns all resources that don't have a `type` label:

  `!type`

## Pagination

Responses which return multiple items support pagination. If they do support pagination, it can be controlled with following query string parameters:

- A `page` parameter specifies the page to fetch. The number of the first page is 1. For invalid values, page 1 is fetched.
- A `per_page` parameter specifies the number of items returned per page. The default value is 25 and the maximum is 50, unless otherwise specified in the documentation. For invalid values, the default is used.

Responses contain a `Link` header with pagination information. This header is deprecated and will be removed in the future as [announced in the changelog](https://docs.hetzner.cloud/changelog#2026-10-12-link-header-deprecation).

Additionally, if the response body is JSON and the root object is an object, that object has a `pagination` object inside the `meta` object with pagination information:

**Example Pagination**

```json
{
    "servers": [...],
    "meta": {
        "pagination": {
            "page": 2,
            "per_page": 25,
            "previous_page": 1,
            "next_page": 3,
            "last_page": 4,
            "total_entries": 100
        }
    }
}
```

The keys `previous_page`, `next_page`, `last_page`, and `total_entries` may be `null` when on the first page, last page, or when the total number of entries is unknown.

**Example Pagination Link header**

```http
Link: <https://api.hetzner.cloud/v1/actions?page=2&per_page=5>; rel="prev",
      <https://api.hetzner.cloud/v1/actions?page=4&per_page=5>; rel="next",
      <https://api.hetzner.cloud/v1/actions?page=6&per_page=5>; rel="last"
```

Line breaks have been added for display purposes only and responses may only contain some of the above `rel` values.

## Rate Limiting

All requests, whether they are authenticated or not, are subject to rate limiting. If you have reached your limit, your requests will be handled with a `429 Too Many Requests` error. Burst requests are allowed. Responses contain several headers which provide information about your current rate limit status.

- The `RateLimit-Limit` header contains the total number of requests you can perform per hour.
- The `RateLimit-Remaining` header contains the number of requests remaining in the current rate limit time frame.
- The `RateLimit-Reset` header contains a UNIX timestamp of the point in time when your rate limit will have recovered, and you will have the full number of requests available again.

The default limit is 3600 requests per hour and per Project. The number of remaining requests increases gradually. For example, when your limit is 3600 requests per hour, the number of remaining requests will increase by 1 every second.

## Server Metadata

Your Server can discover metadata about itself by doing a HTTP request to specific URLs. The following data is available:

| Data              | Format | Contents                                                     |
| ----------------- | ------ | ------------------------------------------------------------ |
| hostname          | text   | Name of the Server as set in the api                         |
| instance-id       | number | ID of the server                                             |
| public-ipv4       | text   | Primary public IPv4 address                                  |
| private-networks  | yaml   | Details about the private networks the Server is attached to |
| availability-zone | text   | Name of the availability zone that Server runs in            |
| region            | text   | Network zone, e.g. eu-central                                |

**Example: Summary**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata
```

```yaml
availability-zone: hel1-dc2
hostname: my-server
instance-id: 42
public-ipv4: 1.2.3.4
region: eu-central
```

**Example: Hostname**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/hostname
my-server
```

**Example: Instance ID**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/instance-id
42
```

**Example: Public IPv4**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/public-ipv4
1.2.3.4
```

**Example: Private Networks**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/private-networks
```

```yaml
- ip: 10.0.0.2
  alias_ips: [10.0.0.3, 10.0.0.4]
  interface_num: 1
  mac_address: 86:00:00:2a:7d:e0
  network_id: 1234
  network_name: nw-test1
  network: 10.0.0.0/8
  subnet: 10.0.0.0/24
  gateway: 10.0.0.1
- ip: 192.168.0.2
  alias_ips: []
  interface_num: 2
  mac_address: 86:00:00:2a:7d:e1
  network_id: 4321
  network_name: nw-test2
  network: 192.168.0.0/16
  subnet: 192.168.0.0/24
  gateway: 192.168.0.1
```

**Example: Availability Zone**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/availability-zone
hel1-dc2
```

**Example: Region**

```shell
$ curl http://169.254.169.254/hetzner/v1/metadata/region
eu-central
```

## Sorting

Some responses which return multiple items support sorting. If they do support sorting the documentation states which fields can be used for sorting. You specify sorting with the `sort` query string parameter. You can sort by multiple fields. You can set the sort direction by appending `:asc` or `:desc` to the field name. By default, ascending sorting is used.

**Example: Sorting**

```
https://api.hetzner.cloud/v1/actions?sort=status
https://api.hetzner.cloud/v1/actions?sort=status:asc
https://api.hetzner.cloud/v1/actions?sort=status:desc
https://api.hetzner.cloud/v1/actions?sort=status:asc&sort=command:desc
```

Sorting parameters are applied in the order they are provided.
Invalid sorting parameters (e.g. providing contrary sorting orders like `sort=status:asc&sort=status:desc`) are ignored.

## Deprecation Notices

You can find all announced deprecations in our [Changelog](/changelog).


Version: 1.0.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**Get multiple Actions

    Returns multiple Action objects specified by the `id` parameter.

    **Note**: This endpoint previously allowed listing all actions in the project. This functionality was deprecated in July 2023 and removed on 30 January 2025.

    - Announcement: https://docs.hetzner.cloud/changelog#2023-07-20-actions-list-endpoint-is-deprecated
    - Removal: https://docs.hetzner.cloud/changelog#2025-01-30-listing-arbitrary-actions-in-the-actions-list-endpoint-is-removed


    Sends a `GET` request to `/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    */
    pub async fn get_actions<'a>(
        &'a self,
        id: &'a ::std::vec::Vec<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ActionListResponse>, Error<types::GetActionsResponse>> {
        let url = format!("{}/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns a specific Action object.


    Sends a `GET` request to `/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetActionResponse>> {
        let url = format!("{}/actions/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Firewalls

    Returns all [Firewalls](#tag/firewalls).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/firewalls`

    Arguments:
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    */
    pub async fn list_firewalls<'a>(
        &'a self,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListFirewallsSortItem>>,
    ) -> Result<ResponseValue<types::FirewallsResponse>, Error<types::ListFirewallsResponse>> {
        let url = format!("{}/firewalls", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_firewalls",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a Firewall

    Create a [Firewall](#tag/firewalls).

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_already_added` | The [Server](#tag/servers) was applied more than once. |
    | `422` | `incompatible_network_type` | The resources network type is not supported by [Firewalls](#tag/firewalls). |
    | `422` | `firewall_resource_not_found` | The resource the [Firewall](#tag/firewalls) should be attached to was not found. |


    Sends a `POST` request to `/firewalls`

    */
    pub async fn create_firewall<'a>(
        &'a self,
        body: &'a types::CreateFirewallRequest,
    ) -> Result<ResponseValue<types::CreateFirewallResponse>, Error<types::CreateFirewallResponse>>
    {
        let url = format!("{}/firewalls", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions

    Returns all [Actions](#tag/actions) for [Firewalls](#tag/firewalls).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/firewalls/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_firewalls_actions<'a>(
        &'a self,
        id: Option<&'a ::std::vec::Vec<::std::num::NonZeroU64>>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListFirewallsActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListFirewallsActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListFirewallsActionsResponse>,
    > {
        let url = format!("{}/firewalls/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_firewalls_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns the specific [Action](#tag/actions).


    Sends a `GET` request to `/firewalls/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_firewalls_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetFirewallsActionResponse>>
    {
        let url = format!(
            "{}/firewalls/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewalls_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Firewall

    Returns a single [Firewall](#tag/firewalls).


    Sends a `GET` request to `/firewalls/{id}`

    Arguments:
    - `id`: ID of the Firewall.
    */
    pub async fn get_firewall<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::FirewallResponse>, Error<types::GetFirewallResponse>> {
        let url = format!(
            "{}/firewalls/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a Firewall

    Update a [Firewall](#tag/firewalls).

    In case of a parallel running change on the [Firewall](#tag/firewalls) a `conflict` error will be returned.


    Sends a `PUT` request to `/firewalls/{id}`

    Arguments:
    - `id`: ID of the Firewall.
    - `body`
    */
    pub async fn update_firewall<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::UpdateFirewallRequest,
    ) -> Result<ResponseValue<types::FirewallResponse>, Error<types::UpdateFirewallResponse>> {
        let url = format!(
            "{}/firewalls/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a Firewall

    Deletes the [Firewall](#tag/firewalls).

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `resource_in_use` | [Firewall](#tag/firewalls) still applied to a resource |


    Sends a `DELETE` request to `/firewalls/{id}`

    Arguments:
    - `id`: ID of the Firewall.
    */
    pub async fn delete_firewall<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<()>, Error<types::DeleteFirewallResponse>> {
        let url = format!(
            "{}/firewalls/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions for a Firewall

    Returns all [Actions](#tag/actions) for the [Firewall](#tag/firewalls).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/firewalls/{id}/actions`

    Arguments:
    - `id`: ID of the Firewall.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_firewall_actions<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListFirewallActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListFirewallActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListFirewallActionsResponse>,
    > {
        let url = format!(
            "{}/firewalls/{}/actions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_firewall_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Apply to Resources

    Applies a [Firewall](#tag/firewalls) to multiple resources.

    Supported resources:
    - [Servers](#tag/servers) (with a public network interface)
    - [Label Selectors](#description/label-selector)

    A [Server](#tag/servers) can be applied to [a maximum of 5 Firewalls](https://docs.hetzner.com/cloud/firewalls/overview#limits). This limit
    applies to [Servers](#tag/servers) applied via a matching [Label Selector](#description/label-selector) as well.

    Updates to resources matching or no longer matching a [Label Selector](#description/label-selector) can take up to a few seconds
    to be processed.

    A [Firewall](#tag/firewalls) is applied to a resource once the related [Action](#tag/actions) with command `apply_firewall` successfully finished.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `404` | `firewall_resource_not_found` | The resource the [Firewall](#tag/firewalls) should be applied to was not found |
    | `422` | `firewall_already_applied` | [Firewall](#tag/firewalls) is already applied to resource |
    | `422` | `incompatible_network_type` | The network type of the resource is not supported by [Firewalls](#tag/firewalls) |
    | `422` | `private_net_only_server` | The [Server](#tag/servers) the [Firewall](#tag/firewalls) should be applied to has no public interface |


    Sends a `POST` request to `/firewalls/{id}/actions/apply_to_resources`

    Arguments:
    - `id`: ID of the Firewall.
    - `body`
    */
    pub async fn apply_firewall_to_resources<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ApplyToResourcesRequest,
    ) -> Result<
        ResponseValue<types::ActionListResponse>,
        Error<types::ApplyFirewallToResourcesResponse>,
    > {
        let url = format!(
            "{}/firewalls/{}/actions/apply_to_resources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apply_firewall_to_resources",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Remove from Resources

    Removes a [Firewall](#tag/firewalls) from multiple resources.

    Supported resources:
    - [Servers](#tag/servers) (with a public network interface)

    A [Firewall](#tag/firewalls) is removed from a resource once the related [Action](#tag/actions) with command `remove_firewall` successfully finished.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `404` | `firewall_resource_not_found` | The resource the [Firewall](#tag/firewalls) should be removed from was not found |
    | `422` | `firewall_managed_by_label_selector` | [Firewall](#tag/firewall) is applied via a [Label Selector](#description/label-selector) and cannot be removed manually |


    Sends a `POST` request to `/firewalls/{id}/actions/remove_from_resources`

    Arguments:
    - `id`: ID of the Firewall.
    - `body`
    */
    pub async fn remove_firewall_from_resources<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::RemoveFromResourcesRequest,
    ) -> Result<
        ResponseValue<types::ActionListResponse>,
        Error<types::RemoveFirewallFromResourcesResponse>,
    > {
        let url = format!(
            "{}/firewalls/{}/actions/remove_from_resources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "remove_firewall_from_resources",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Set Rules

    Set the rules of a [Firewall](#tag/firewalls).

    Overwrites the existing rules with the given ones. Pass an empty array to remove all rules.

    Rules are limited to 50 entries per [Firewall](#tag/firewalls) and [500 effective rules](https://docs.hetzner.com/cloud/firewalls/overview#limits).


    Sends a `POST` request to `/firewalls/{id}/actions/set_rules`

    Arguments:
    - `id`: ID of the Firewall.
    - `body`
    */
    pub async fn set_firewall_rules<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::SetRulesRequest,
    ) -> Result<ResponseValue<types::ActionListResponse>, Error<types::SetFirewallRulesResponse>>
    {
        let url = format!(
            "{}/firewalls/{}/actions/set_rules",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "set_firewall_rules",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action for a Firewall

    **Deprecated**: This operation is deprecated, see our [changelog](https://docs.hetzner.cloud/changelog#2026-04-30-deprecate-get-resource-action-endpoints) for more details.

    Returns a specific [Action](#tag/actions) for a [Firewall](#tag/firewalls).


    Sends a `GET` request to `/firewalls/{id}/actions/{action_id}`

    Arguments:
    - `id`: ID of the Firewall.
    - `action_id`: ID of the Action.
    */
    pub async fn get_firewall_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        action_id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetFirewallActionResponse>> {
        let url = format!(
            "{}/firewalls/{}/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&action_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Images

    Returns all Image objects. You can select specific Image types only and sort the results by using URI parameters.


    Sends a `GET` request to `/images`

    Arguments:
    - `architecture`: Filter resources by cpu architecture.

    The response will only contain the resources with the specified cpu architecture.

    - `bound_to`: Filter Images by their linked Server ID. May be used multiple times.

    Only available for Images of type `backup`.

    - `include_deprecated`: Include deprecated Images.
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter resources by status. May be used multiple times.

    The response will only contain the resources with the specified status.

    - `type_`: Filter resources by type. May be used multiple times.

    The response will only contain the resources with the specified type.

    */
    pub async fn list_images<'a>(
        &'a self,
        architecture: Option<types::ListImagesArchitecture>,
        bound_to: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include_deprecated: Option<bool>,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListImagesSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListImagesStatusItem>>,
        type_: Option<&'a ::std::vec::Vec<types::ListImagesTypeItem>>,
    ) -> Result<ResponseValue<types::ListImagesResponse>, Error<types::ListImagesResponse>> {
        let url = format!("{}/images", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "architecture",
                &architecture,
            ))
            .query(&progenitor_client::QueryParam::new("bound_to", &bound_to))
            .query(&progenitor_client::QueryParam::new(
                "include_deprecated",
                &include_deprecated,
            ))
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_images",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions

    Returns all Action objects. You can `sort` the results by using the sort URI parameter, and filter them with the `status` and `id` parameter.


    Sends a `GET` request to `/images/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_images_actions<'a>(
        &'a self,
        id: Option<&'a ::std::vec::Vec<::std::num::NonZeroU64>>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListImagesActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListImagesActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListImagesActionsResponse>,
    > {
        let url = format!("{}/images/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_images_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns a specific Action object.


    Sends a `GET` request to `/images/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_images_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetImagesActionResponse>> {
        let url = format!(
            "{}/images/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_images_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Image

    Returns a specific Image object.


    Sends a `GET` request to `/images/{id}`

    Arguments:
    - `id`: ID of the Image.
    */
    pub async fn get_image<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetImageResponse>, Error<types::GetImageResponse>> {
        let url = format!("{}/images/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update an Image

    Updates the Image. You may change the description, convert a Backup Image to a Snapshot Image or change the Image labels. Only Images of type `snapshot` and `backup` can be updated.


    Sends a `PUT` request to `/images/{id}`

    Arguments:
    - `id`: ID of the Image.
    - `body`
    */
    pub async fn update_image<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::UpdateImageRequest,
    ) -> Result<ResponseValue<types::UpdateImageResponse>, Error<types::UpdateImageResponse>> {
        let url = format!("{}/images/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete an Image

    Deletes an Image. Only Images of type `snapshot` and `backup` can be deleted.


    Sends a `DELETE` request to `/images/{id}`

    Arguments:
    - `id`: ID of the Image.
    */
    pub async fn delete_image<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<()>, Error<types::DeleteImageResponse>> {
        let url = format!("{}/images/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions for an Image

    Returns all Action objects for an Image. You can sort the results by using the `sort` URI parameter, and filter them with the `status` parameter.


    Sends a `GET` request to `/images/{id}/actions`

    Arguments:
    - `id`: ID of the Image.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_image_actions<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListImageActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListImageActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListImageActionsResponse>,
    > {
        let url = format!(
            "{}/images/{}/actions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_image_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change Image Protection

    Changes the protection configuration of the Image. Can only be used on snapshots.


    Sends a `POST` request to `/images/{id}/actions/change_protection`

    Arguments:
    - `id`: ID of the Image.
    - `body`
    */
    pub async fn change_image_protection<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeImageProtectionBody,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeImageProtectionResponse>>
    {
        let url = format!(
            "{}/images/{}/actions/change_protection",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_image_protection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action for an Image

    **Deprecated**: This operation is deprecated, see our [changelog](https://docs.hetzner.cloud/changelog#2026-04-30-deprecate-get-resource-action-endpoints) for more details.

    Returns a specific Action for an Image.


    Sends a `GET` request to `/images/{id}/actions/{action_id}`

    Arguments:
    - `id`: ID of the Image.
    - `action_id`: ID of the Action.
    */
    pub async fn get_image_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        action_id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetImageActionResponse>> {
        let url = format!(
            "{}/images/{}/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&action_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_image_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Locations

    Returns all [Locations](#tag/locations).


    Sends a `GET` request to `/locations`

    Arguments:
    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    */
    pub async fn list_locations<'a>(
        &'a self,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListLocationsSortItem>>,
    ) -> Result<ResponseValue<types::ListLocationsResponse>, Error<types::ListLocationsResponse>>
    {
        let url = format!("{}/locations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_locations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Location

    Returns a [Location](#tag/locations).


    Sends a `GET` request to `/locations/{id}`

    Arguments:
    - `id`: ID of the Location.
    */
    pub async fn get_location<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetLocationResponse>, Error<types::GetLocationResponse>> {
        let url = format!(
            "{}/locations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_location",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Networks

    List multiple [Networks](#tag/networks).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/networks`

    Arguments:
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    */
    pub async fn list_networks<'a>(
        &'a self,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListNetworksSortItem>>,
    ) -> Result<ResponseValue<types::ListNetworksResponse>, Error<types::ListNetworksResponse>>
    {
        let url = format!("{}/networks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_networks",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a Network

    Creates a [Network](#tag/networks).

    The provided `ip_range` can only be extended later on, but not reduced.

    Subnets can be added now or later on using the [add subnet action](#tag/network-actions/add_network_subnet). If you do not specify an `ip_range` for the subnet the first available /24 range will be used.

    Routes can be added now or later by using the [add route action](#tag/network-actions/add_network_route).


    Sends a `POST` request to `/networks`

    */
    pub async fn create_network<'a>(
        &'a self,
        body: &'a types::NetworkCreateRequest,
    ) -> Result<ResponseValue<types::CreateNetworkResponse>, Error<types::CreateNetworkResponse>>
    {
        let url = format!("{}/networks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions

    Lists multiple [Actions](#tag/actions).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/networks/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_networks_actions<'a>(
        &'a self,
        id: Option<&'a ::std::vec::Vec<::std::num::NonZeroU64>>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListNetworksActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListNetworksActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListNetworksActionsResponse>,
    > {
        let url = format!("{}/networks/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_networks_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns a single [Action](#tag/actions).


    Sends a `GET` request to `/networks/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_networks_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetNetworksActionResponse>> {
        let url = format!(
            "{}/networks/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_networks_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Network

    Get a specific [Network](#tag/networks).


    Sends a `GET` request to `/networks/{id}`

    Arguments:
    - `id`: ID of the Network.
    */
    pub async fn get_network<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetNetworkResponse>, Error<types::GetNetworkResponse>> {
        let url = format!("{}/networks/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a Network

    Update a [Network](#tag/networks).

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `PUT` request to `/networks/{id}`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn update_network<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::NetworkUpdateRequest,
    ) -> Result<ResponseValue<types::UpdateNetworkResponse>, Error<types::UpdateNetworkResponse>>
    {
        let url = format!("{}/networks/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a Network

    Deletes a [Network](#tag/networks).

    Attached resources will be detached automatically.


    Sends a `DELETE` request to `/networks/{id}`

    Arguments:
    - `id`: ID of the Network.
    */
    pub async fn delete_network<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<()>, Error<types::DeleteNetworkResponse>> {
        let url = format!("{}/networks/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions for a Network

    Lists [Actions](#tag/actions) for a [Network](#tag/networks).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/networks/{id}/actions`

    Arguments:
    - `id`: ID of the Network.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_network_actions<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListNetworkActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListNetworkActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListNetworkActionsResponse>,
    > {
        let url = format!(
            "{}/networks/{}/actions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_network_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add a route to a Network

    Adds a route entry to a [Network](#tag/networks).

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/add_route`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn add_network_route<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::AddRouteRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AddNetworkRouteResponse>> {
        let url = format!(
            "{}/networks/{}/actions/add_route",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "add_network_route",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add a subnet to a Network

    Adds a new subnet to the [Network](#tag/networks).

    If the subnet `ip_range` is not provided, the first available `/24` IP range will be used.

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/add_subnet`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn add_network_subnet<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::AddSubnetRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AddNetworkSubnetResponse>> {
        let url = format!(
            "{}/networks/{}/actions/add_subnet",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "add_network_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change IP range of a Network

    Changes the IP range of a [Network](#tag/networks).

    The following restrictions apply to changing the IP range:
    - IP ranges can only be extended and never shrunk.
    - IPs can only be added to the end of the existing range, therefore only the netmask is allowed to be changed.

    To update the routes on the connected [Servers](#tag/servers), they need to be rebooted or the routes to be updated manually.

    For example if the [Network](#tag/networks) has a range of `10.0.0.0/16` to extend it the new range has to start with the IP `10.0.0.0` as well. The netmask `/16` can be changed to a smaller one then `16` therefore increasing the IP range. A valid entry would be `10.0.0.0/15`, `10.0.0.0/14` or `10.0.0.0/13` and so on.

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/change_ip_range`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn change_network_ip_range<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeIpRangeRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeNetworkIpRangeResponse>>
    {
        let url = format!(
            "{}/networks/{}/actions/change_ip_range",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_network_ip_range",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change Network Protection

    Changes the protection settings of a [Network](#tag/networks).

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/change_protection`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn change_network_protection<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeProtectionRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeNetworkProtectionResponse>>
    {
        let url = format!(
            "{}/networks/{}/actions/change_protection",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_network_protection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a route from a Network

    Delete a route entry from a [Network](#tag/networks).

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/delete_route`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn delete_network_route<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::DeleteRouteRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DeleteNetworkRouteResponse>>
    {
        let url = format!(
            "{}/networks/{}/actions/delete_route",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_network_route",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a subnet from a Network

    Deletes a single subnet entry from a [Network](#tag/networks).

    Subnets containing attached resources can not be deleted, they must be detached beforehand.

    If a change is currently being performed on this [Network](#tag/networks), a error response with code `conflict` will be returned.


    Sends a `POST` request to `/networks/{id}/actions/delete_subnet`

    Arguments:
    - `id`: ID of the Network.
    - `body`
    */
    pub async fn delete_network_subnet<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::DeleteSubnetRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DeleteNetworkSubnetResponse>>
    {
        let url = format!(
            "{}/networks/{}/actions/delete_subnet",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_network_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action for a Network

    **Deprecated**: This operation is deprecated, see our [changelog](https://docs.hetzner.cloud/changelog#2026-04-30-deprecate-get-resource-action-endpoints) for more details.

    Returns a specific [Action](#tag/actions) for a [Network](#tag/networks).


    Sends a `GET` request to `/networks/{id}/actions/{action_id}`

    Arguments:
    - `id`: ID of the Network.
    - `action_id`: ID of the Action.
    */
    pub async fn get_network_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        action_id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetNetworkActionResponse>> {
        let url = format!(
            "{}/networks/{}/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&action_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_network_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Members of a Network

    List the resources attached to a specific [Network](#tag/networks).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/networks/{id}/members`

    Arguments:
    - `id`: ID of the Network.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter members by status. May be used multiple times.

    The response will only contain members matching the specified statuses.

    - `subnet`: Filter members by the subnet they are attached to. May be used multiple times.

    The response will only contain members attached to the subnets with the
    specified IP ranges. Uses CIDR notation.

    - `type_`: Filter members by type. May be used multiple times.

    The response will only contain members matching the specified types.

    */
    pub async fn list_network_members<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListNetworkMembersSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListNetworkMembersStatusItem>>,
        subnet: Option<&'a ::std::vec::Vec<::std::string::String>>,
        type_: Option<&'a ::std::vec::Vec<types::ListNetworkMembersTypeItem>>,
    ) -> Result<
        ResponseValue<types::ListNetworkMembersResponse>,
        Error<types::ListNetworkMembersResponse>,
    > {
        let url = format!(
            "{}/networks/{}/members",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .query(&progenitor_client::QueryParam::new("subnet", &subnet))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_network_members",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get all prices

    Returns prices for all resources available on the platform. VAT and currency of the Project owner are used for calculations.

    Both net and gross prices are included in the response.


    Sends a `GET` request to `/pricing`

    */
    pub async fn get_pricing<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetPricingResponse>, Error<types::GetPricingResponse>> {
        let url = format!("{}/pricing", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_pricing",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Primary IPs

    List multiple [Primary IPs](#tag/primary-ips).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/primary_ips`

    Arguments:
    - `ip`: Filter results by IP address.
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    */
    pub async fn list_primary_ips<'a>(
        &'a self,
        ip: Option<&'a str>,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListPrimaryIpsSortItem>>,
    ) -> Result<ResponseValue<types::PrimaryIPsResponse>, Error<types::ListPrimaryIpsResponse>>
    {
        let url = format!("{}/primary_ips", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ip", &ip))
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_primary_ips",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a Primary IP

    Create a new [Primary IP](#tag/primary-ips).

    Can optionally be assigned to a resource by providing an `assignee_id` and `assignee_type`.

    If not assigned to a resource the `location` key needs to be provided. This can be either the ID or the name of the [Location](#tag/locations) this [Primary IP](#tag/primary-ips) shall be created in.

    A [Primary IP](#tag/primary-ips) can only be assigned to resource in the same [Location](#tag/locations) later on.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_not_stopped` | The specified [Server](#tag/servers) is running, but needs to be powered off |
    | `422` | `server_has_ipv4` | The [Server](#tag/servers) already has an ipv4 address |
    | `422` | `server_has_ipv6` | The [Server](#tag/servers) already has an ipv6 address |


    Sends a `POST` request to `/primary_ips`

    Arguments:
    - `body`: Request Body for creating a new [Primary IP](#tag/primary-ips).

    The `location` and `assignee_id`/`assignee_type` attributes are mutually exclusive.

    */
    pub async fn create_primary_ip<'a>(
        &'a self,
        body: &'a types::PrimaryIpCreateRequest,
    ) -> Result<ResponseValue<types::CreatePrimaryIpResponse>, Error<types::CreatePrimaryIpResponse>>
    {
        let url = format!("{}/primary_ips", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions

    Lists multiple [Actions](#tag/actions).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/primary_ips/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_primary_ips_actions<'a>(
        &'a self,
        id: Option<&'a ::std::vec::Vec<::std::num::NonZeroU64>>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListPrimaryIpsActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListPrimaryIpsActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListPrimaryIpsActionsResponse>,
    > {
        let url = format!("{}/primary_ips/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_primary_ips_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns a single [Action](#tag/actions).


    Sends a `GET` request to `/primary_ips/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_primary_ips_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetPrimaryIpsActionResponse>>
    {
        let url = format!(
            "{}/primary_ips/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_primary_ips_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions for a Primary IP

    Returns all [Actions](#tag/actions) for a [Primary IP](#tag/primary-ips).

    Use the provided URI parameters to modify the result.


    Sends a `GET` request to `/primary_ips/{id}/actions`

    Arguments:
    - `id`: ID of the Primary IP.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_primary_ip_actions<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListPrimaryIpActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListPrimaryIpActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListPrimaryIpActionsResponse>,
    > {
        let url = format!(
            "{}/primary_ips/{}/actions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_primary_ip_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action for a Primary IP

    **Deprecated**: This operation is deprecated, see our [changelog](https://docs.hetzner.cloud/changelog#2026-04-30-deprecate-get-resource-action-endpoints) for more details.

    Returns a specific [Action](#tag/actions) for a [Primary IP](#tag/primary-ips).


    Sends a `GET` request to `/primary_ips/{id}/actions/{action_id}`

    Arguments:
    - `id`: ID of the Primary IP.
    - `action_id`: ID of the Action.
    */
    pub async fn get_primary_ip_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        action_id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetPrimaryIpActionResponse>>
    {
        let url = format!(
            "{}/primary_ips/{}/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&action_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_primary_ip_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Primary IP

    Returns a [Primary IP](#tag/primary-ips).


    Sends a `GET` request to `/primary_ips/{id}`

    Arguments:
    - `id`: ID of the Primary IP.
    */
    pub async fn get_primary_ip<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::PrimaryIpResponse>, Error<types::GetPrimaryIpResponse>> {
        let url = format!(
            "{}/primary_ips/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a Primary IP

    Update a [Primary IP](#tag/primary-ips).

    If another change is concurrently performed on this [Primary IP](#tag/primary-ips), a error response with code `conflict` will be returned.


    Sends a `PUT` request to `/primary_ips/{id}`

    Arguments:
    - `id`: ID of the Primary IP.
    - `body`
    */
    pub async fn update_primary_ip<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::PrimaryIpUpdateRequest,
    ) -> Result<ResponseValue<types::PrimaryIpResponse>, Error<types::UpdatePrimaryIpResponse>>
    {
        let url = format!(
            "{}/primary_ips/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a Primary IP

    Deletes a [Primary IP](#tag/primary-ips).

    The [Server](#tag/servers) must be powered off (status `off`) in order for this operation to succeed.
    If assigned to a [Server](#tag/servers) the [Primary IP](#tag/primary-ips) will be unassigned automatically until 1 May 2026. After this date, the [Primary IP](#tag/primary-ips) needs to be unassigned before it can be deleted.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `must_be_unassigned` | Error when IP is still assigned to a Resource. This error will appear as of 1 May 2026. |


    Sends a `DELETE` request to `/primary_ips/{id}`

    Arguments:
    - `id`: ID of the Primary IP.
    */
    pub async fn delete_primary_ip<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<()>, Error<types::DeletePrimaryIpResponse>> {
        let url = format!(
            "{}/primary_ips/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Assign a Primary IP to a resource

    Assign a [Primary IP](#tag/primary-ips) to a resource.

    A [Server](#tag/servers) can only have one [Primary IP](#tag/primary-ips) of type `ipv4` and one of type `ipv6` assigned. If you need more IPs use [Floating IPs](#tag/floating-ips).

    A [Server](#tag/servers) must be powered off (status `off`) in order for this operation to succeed.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_not_stopped` | The [Server](#tag/servers) is running, but needs to be powered off |
    | `422` | `primary_ip_already_assigned` | [Primary IP](#tag/primary-ips) is already assigned to a different [Server](#tag/servers) |
    | `422` | `server_has_ipv4` | The [Server](#tag/servers) already has an IPv4 address |
    | `422` | `server_has_ipv6` | The [Server](#tag/servers) already has an IPv6 address |


    Sends a `POST` request to `/primary_ips/{id}/actions/assign`

    Arguments:
    - `id`: ID of the Primary IP.
    - `body`
    */
    pub async fn assign_primary_ip<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::PrimaryIpActionsAssignRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AssignPrimaryIpResponse>> {
        let url = format!(
            "{}/primary_ips/{}/actions/assign",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "assign_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change reverse DNS records for a Primary IP

    Change the reverse DNS records for this [Primary IP](#tag/primary-ips).

    Allows to modify the PTR records set for the IP address.


    Sends a `POST` request to `/primary_ips/{id}/actions/change_dns_ptr`

    Arguments:
    - `id`: ID of the Primary IP.
    - `body`: The `ip` attributes specifies for which IP address the record is set. For IPv4 addresses this must be the exact address of the [Primary IP](#tag/primary-ips). For IPv6 addresses this must be a single address within the `/64` subnet of the [Primary IP](#tag/primary-ips).

    The `dns_ptr` attribute specifies the hostname used for the IP address. Must be a fully qualified domain name (FQDN) without trailing dot.

    For IPv6 [Primary IPs](#tag/primary-ips) up to 100 entries can be created.

    */
    pub async fn change_primary_ip_dns_ptr<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::PrimaryIpActionsChangeDnsPtrRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangePrimaryIpDnsPtrResponse>>
    {
        let url = format!(
            "{}/primary_ips/{}/actions/change_dns_ptr",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_primary_ip_dns_ptr",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change Primary IP Protection

    Changes the protection configuration of a [Primary IP](#tag/primary-ips).

    A [Primary IPs](#tag/primary-ips) deletion protection can only be enabled if its `auto_delete` property is set to `false`.


    Sends a `POST` request to `/primary_ips/{id}/actions/change_protection`

    Arguments:
    - `id`: ID of the Primary IP.
    - `body`
    */
    pub async fn change_primary_ip_protection<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::PrimaryIpActionsChangeProtectionRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangePrimaryIpProtectionResponse>>
    {
        let url = format!(
            "{}/primary_ips/{}/actions/change_protection",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_primary_ip_protection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Unassign a Primary IP from a resource

    Unassign a [Primary IP](#tag/primary-ips) from a resource.

    A [Server](#tag/servers) must be powered off (status `off`) in order for this operation to succeed.

    A [Server](#tag/servers) requires at least one network interface (public or private) to be powered on.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_not_stopped` | The [Server](#tag/servers) is running, but needs to be powered off |
    | `422` | `server_is_load_balancer_target` | The [Server](#tag/servers) IPv4 address is a loadbalancer target |


    Sends a `POST` request to `/primary_ips/{id}/actions/unassign`

    Arguments:
    - `id`: ID of the Primary IP.
    */
    pub async fn unassign_primary_ip<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::UnassignPrimaryIpResponse>> {
        let url = format!(
            "{}/primary_ips/{}/actions/unassign",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "unassign_primary_ip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Server Types

    Gets all Server type objects.


    Sends a `GET` request to `/server_types`

    Arguments:
    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    */
    pub async fn list_server_types<'a>(
        &'a self,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListServerTypesResponse>, Error<types::ListServerTypesResponse>>
    {
        let url = format!("{}/server_types", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_server_types",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Server Type

    Gets a specific Server type object.


    Sends a `GET` request to `/server_types/{id}`

    Arguments:
    - `id`: ID of the Server Type.
    */
    pub async fn get_server_type<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetServerTypeResponse>, Error<types::GetServerTypeResponse>>
    {
        let url = format!(
            "{}/server_types/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_server_type",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Servers

    Returns all existing Server objects.


    Sends a `GET` request to `/servers`

    Arguments:
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter resources by status. May be used multiple times.

    The response will only contain the resources with the specified status.

    */
    pub async fn list_servers<'a>(
        &'a self,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListServersSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListServersStatusItem>>,
    ) -> Result<ResponseValue<types::ListServersResponse>, Error<types::ListServersResponse>> {
        let url = format!("{}/servers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_servers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a Server

    Creates a new Server. Returns preliminary information about the Server as well as an Action that covers progress of creation.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `412` | `primary_ip_version_mismatch` | The specified Primary IP has the wrong IP Version |
    | `422` | `placement_error` | An error during the placement occurred |
    | `422` | `primary_ip_assigned` | The specified Primary IP is already assigned to a server |
    | `422` | `primary_ip_datacenter_mismatch` | The specified Primary IP is in a different location |


    Sends a `POST` request to `/servers`

    Arguments:
    - `body`: Please note that Server names must be unique per Project and valid hostnames as per RFC 1123 (i.e. may only contain letters, digits, periods, and dashes).

    For `server_type` you can either use the ID as listed in `/server_types` or its name.

    For `image` you can either use the ID as listed in `/images` or its name.

    If you want to create the Server in a Location, you must set `location` to the ID or name as listed in `/locations`.

    Some properties like `start_after_create` or `automount` will trigger Actions after the Server is created. Those Actions are listed in the `next_actions` field in the response.

    For accessing your Server we strongly recommend to use SSH keys by passing the respective key IDs in `ssh_keys`. If you do not specify any `ssh_keys` we will generate a root password for you and return it in the response.

    Please note that provided user-data is stored in our systems. While we take measures to protect it we highly recommend that you don’t use it to store passwords or other sensitive information.

    */
    pub async fn create_server<'a>(
        &'a self,
        body: &'a types::CreateServerRequest,
    ) -> Result<ResponseValue<types::CreateServerResponse>, Error<types::CreateServerResponse>>
    {
        let url = format!("{}/servers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions

    Returns all Action objects. You can `sort` the results by using the sort URI parameter, and filter them with the `status` and `id` parameter.


    Sends a `GET` request to `/servers/actions`

    Arguments:
    - `id`: Filter the actions by ID. May be used multiple times.

    The response will only contain actions matching the specified IDs.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_servers_actions<'a>(
        &'a self,
        id: Option<&'a ::std::vec::Vec<::std::num::NonZeroU64>>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListServersActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListServersActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListServersActionsResponse>,
    > {
        let url = format!("{}/servers/actions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_servers_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action

    Returns a specific Action object.


    Sends a `GET` request to `/servers/actions/{id}`

    Arguments:
    - `id`: ID of the Action.
    */
    pub async fn get_servers_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetServersActionResponse>> {
        let url = format!(
            "{}/servers/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_servers_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a Server

    Returns a specific Server object. The Server must exist inside the Project.


    Sends a `GET` request to `/servers/{id}`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn get_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetServerResponse>, Error<types::GetServerResponse>> {
        let url = format!("{}/servers/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a Server

    Updates a Server. You can update a Server’s name and a Server’s labels.

    Please note that Server names must be unique per Project and valid hostnames as per RFC 1123 (i.e. may only contain letters, digits, periods, and dashes).


    Sends a `PUT` request to `/servers/{id}`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn update_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::UpdateServerRequest,
    ) -> Result<ResponseValue<types::UpdateServerResponse>, Error<types::UpdateServerResponse>>
    {
        let url = format!("{}/servers/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a Server

    Deletes a Server.

    This immediately removes the Server from your account, and it is no longer
    accessible. Any resources attached to the server (like Volumes, Primary IPs,
    Floating IPs, Firewalls, Placement Groups) are detached while the server is deleted.


    Sends a `DELETE` request to `/servers/{id}`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn delete_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::DeleteServerResponse>, Error<types::DeleteServerResponse>>
    {
        let url = format!("{}/servers/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List Actions for a Server

    Returns all Action objects for a Server. You can `sort` the results by using the sort URI parameter, and filter them with the `status` parameter.


    Sends a `GET` request to `/servers/{id}/actions`

    Arguments:
    - `id`: ID of the Server.
    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort actions by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    - `status`: Filter the actions by status. May be used multiple times.

    The response will only contain actions matching the specified statuses.

    */
    pub async fn list_server_actions<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListServerActionsSortItem>>,
        status: Option<&'a ::std::vec::Vec<types::ListServerActionsStatusItem>>,
    ) -> Result<
        ResponseValue<types::ActionListResponseWithMeta>,
        Error<types::ListServerActionsResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_server_actions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add a Server to a Placement Group

    Adds a Server to a Placement Group.

    Server must be powered off for this command to succeed.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_not_stopped` | The action requires a stopped server |
    | `422` | `already_in_placement_group` | The server is already part of a placement group |


    Sends a `POST` request to `/servers/{id}/actions/add_to_placement_group`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn add_server_to_placement_group<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::AddToPlacementGroupRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AddServerToPlacementGroupResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/add_to_placement_group",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "add_server_to_placement_group",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Attach an ISO to a Server

    Attaches an ISO to a Server. The Server will immediately see it as a new disk. An already attached ISO will automatically be detached before the new ISO is attached.

    Servers with attached ISOs have a modified boot order: They will try to boot from the ISO first before falling back to hard disk.


    Sends a `POST` request to `/servers/{id}/actions/attach_iso`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn attach_server_iso<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::AttachServerIsoBody,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AttachServerIsoResponse>> {
        let url = format!(
            "{}/servers/{}/actions/attach_iso",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attach_server_iso",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Attach a Server to a Network

    Attaches a Server to a network. This will complement the fixed public Server interface by adding an additional ethernet interface to the Server which is connected to the specified network.

    The Server will get an IP auto assigned from a subnet of type `server` in the same `network_zone`.

    Using the `alias_ips` attribute you can also define one or more additional IPs to the Servers. Please note that you will have to configure these IPs by hand on your Server since only the primary IP will be given out by DHCP.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `server_already_attached` | The server is already attached to the network |
    | `422` | `ip_not_available` | The provided Network IP is not available |
    | `422` | `no_subnet_available` | No Subnet or IP is available for the Server within the network |
    | `422` | `networks_overlap` | The network IP range overlaps with one of the server networks |


    Sends a `POST` request to `/servers/{id}/actions/attach_to_network`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn attach_server_to_network<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::AttachToNetworkRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::AttachServerToNetworkResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/attach_to_network",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attach_server_to_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change alias IPs of a Network

    Changes the alias IPs of an already attached Network. Note that the existing aliases for the specified Network will be replaced with these provided in the request body. So if you want to add an alias IP, you have to provide the existing ones from the Network plus the new alias IP in the request body.


    Sends a `POST` request to `/servers/{id}/actions/change_alias_ips`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn change_server_alias_ips<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeServerAliasIpsBody,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeServerAliasIpsResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/change_alias_ips",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_server_alias_ips",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change reverse DNS entry for this Server

    Changes the hostname that will appear when getting the hostname belonging to the primary IPs (IPv4 and IPv6) of this Server.

    Floating IPs assigned to the Server are not affected by this.


    Sends a `POST` request to `/servers/{id}/actions/change_dns_ptr`

    Arguments:
    - `id`: ID of the Server.
    - `body`: Select the IP address for which to change the DNS entry by passing `ip`. It can be either IPv4 or IPv6. The target hostname is set by passing `dns_ptr`, which must be a fully qualified domain name (FQDN) without trailing dot.
    */
    pub async fn change_server_dns_ptr<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ServerChangeDnsPtrRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeServerDnsPtrResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/change_dns_ptr",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_server_dns_ptr",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change Server Protection

    Changes the protection configuration of the Server.


    Sends a `POST` request to `/servers/{id}/actions/change_protection`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn change_server_protection<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeServerProtectionBody,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeServerProtectionResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/change_protection",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_server_protection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change the Type of a Server

    Changes the type (Cores, RAM and disk sizes) of a Server.

    Server must be powered off for this command to succeed.

    This copies the content of its disk, and starts it again.

    You can only migrate to Server types with the same `storage_type` and equal or bigger disks. Shrinking disks is not possible as it might destroy data.

    If the disk gets upgraded, the Server type can not be downgraded any more. If you plan to downgrade the Server type, set `upgrade_disk` to `false`.

    #### Operation specific errors

    | Status | Code | Description |
    | --- | --- | --- |
    | `422` | `invalid_server_type` | The server type does not fit for the given server or is deprecated |
    | `422` | `server_not_stopped` | The action requires a stopped server |


    Sends a `POST` request to `/servers/{id}/actions/change_type`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn change_server_type<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::ChangeServerTypeBody,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ChangeServerTypeResponse>> {
        let url = format!(
            "{}/servers/{}/actions/change_type",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_server_type",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create Image from a Server

    Creates an Image (snapshot) from a Server by copying the contents of its disks. This creates a snapshot of the current state of the disk and copies it into an Image. If the Server is currently running you must make sure that its disk content is consistent. Otherwise, the created Image may not be readable.

    To make sure disk content is consistent, we recommend to shut down the Server prior to creating an Image.

    You can either create a `backup` Image that is bound to the Server and therefore will be deleted when the Server is deleted, or you can create a `snapshot` Image which is completely independent of the Server it was created from and will survive Server deletion. Backup Images are only available when the backup option is enabled for the Server. Snapshot Images are billed on a per GB basis.


    Sends a `POST` request to `/servers/{id}/actions/create_image`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn create_server_image<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::CreateImageRequest,
    ) -> Result<
        ResponseValue<types::CreateServerImageResponse>,
        Error<types::CreateServerImageResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions/create_image",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_server_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Detach a Server from a Network

    Detaches a Server from a network. The interface for this network will vanish.


    Sends a `POST` request to `/servers/{id}/actions/detach_from_network`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn detach_server_from_network<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::DetachFromNetworkRequest,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DetachServerFromNetworkResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/detach_from_network",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "detach_server_from_network",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Detach an ISO from a Server

    Detaches an ISO from a Server. In case no ISO Image is attached to the Server, the status of the returned Action is immediately set to `success`.


    Sends a `POST` request to `/servers/{id}/actions/detach_iso`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn detach_server_iso<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DetachServerIsoResponse>> {
        let url = format!(
            "{}/servers/{}/actions/detach_iso",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "detach_server_iso",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Disable Backups for a Server

    Disables the automatic backup option and deletes all existing Backups for a Server. No more additional charges for backups will be made.

    Caution: This immediately removes all existing backups for the Server!


    Sends a `POST` request to `/servers/{id}/actions/disable_backup`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn disable_server_backup<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DisableServerBackupResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/disable_backup",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disable_server_backup",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Disable Rescue Mode for a Server

    Disables the Hetzner Rescue System for a Server. This makes a Server start from its disks on next reboot.

    Rescue Mode is automatically disabled when you first boot into it or if you do not use it for 60 minutes.

    Disabling rescue mode will not reboot your Server — you will have to do this yourself.


    Sends a `POST` request to `/servers/{id}/actions/disable_rescue`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn disable_server_rescue<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::DisableServerRescueResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/disable_rescue",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disable_server_rescue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Enable and Configure Backups for a Server

    Enables and configures the automatic daily backup option for the Server. Enabling automatic backups will increase the price of the Server by 20%. In return, you will get seven slots where Images of type backup can be stored.

    Backups are automatically created daily.


    Sends a `POST` request to `/servers/{id}/actions/enable_backup`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn enable_server_backup<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::EnableServerBackupResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/enable_backup",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "enable_server_backup",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Enable Rescue Mode for a Server

    Enable the Hetzner Rescue System for this Server. The next time a Server with enabled rescue mode boots it will start a special minimal Linux distribution designed for repair and reinstall.

    In case a Server cannot boot on its own you can use this to access a Server’s disks.

    Rescue Mode is automatically disabled when you first boot into it or if you do not use it for 60 minutes.

    Enabling rescue mode will not [reboot](https://docs.hetzner.cloud/#server-actions-soft-reboot-a-server) your Server — you will have to do this yourself.


    Sends a `POST` request to `/servers/{id}/actions/enable_rescue`

    Arguments:
    - `id`: ID of the Server.
    - `body`
    */
    pub async fn enable_server_rescue<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::EnableServerRescueBody,
    ) -> Result<
        ResponseValue<types::EnableServerRescueResponse>,
        Error<types::EnableServerRescueResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions/enable_rescue",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "enable_server_rescue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Power off a Server

    Cuts power to the Server. This forcefully stops it without giving the Server operating system time to gracefully stop. May lead to data loss, equivalent to pulling the power cord. Power off should only be used when shutdown does not work.


    Sends a `POST` request to `/servers/{id}/actions/poweroff`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn poweroff_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::PoweroffServerResponse>> {
        let url = format!(
            "{}/servers/{}/actions/poweroff",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "poweroff_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Power on a Server

    Starts a Server by turning its power on.


    Sends a `POST` request to `/servers/{id}/actions/poweron`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn poweron_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::PoweronServerResponse>> {
        let url = format!(
            "{}/servers/{}/actions/poweron",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "poweron_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Soft-reboot a Server

    Reboots a Server gracefully by sending an ACPI request. The Server operating system must support ACPI and react to the request, otherwise the Server will not reboot.


    Sends a `POST` request to `/servers/{id}/actions/reboot`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn reboot_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::RebootServerResponse>> {
        let url = format!(
            "{}/servers/{}/actions/reboot",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "reboot_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Rebuild a Server from an Image

    Rebuilds a Server overwriting its disk with the content of an Image, thereby **destroying all data** on the target Server

    The Image can either be one you have created earlier (`backup` or `snapshot` Image) or it can be a completely fresh `system` Image provided by us. You can get a list of all available Images with `GET /images`.

    Your Server will automatically be powered off before the rebuild command executes.


    Sends a `POST` request to `/servers/{id}/actions/rebuild`

    Arguments:
    - `id`: ID of the Server.
    - `body`: To select which Image to rebuild from you can either pass an ID or a name as the `image` argument. Passing a name only works for `system` Images since the other Image types do not have a name set.
    */
    pub async fn rebuild_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::RebuildServerRequest,
    ) -> Result<ResponseValue<types::RebuildServerResponse>, Error<types::RebuildServerResponse>>
    {
        let url = format!(
            "{}/servers/{}/actions/rebuild",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rebuild_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Remove from Placement Group

    Removes a Server from a Placement Group.


    Sends a `POST` request to `/servers/{id}/actions/remove_from_placement_group`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn remove_server_from_placement_group<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<
        ResponseValue<types::ActionResponse>,
        Error<types::RemoveServerFromPlacementGroupResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions/remove_from_placement_group",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "remove_server_from_placement_group",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request Console for a Server

    Requests credentials for remote access via VNC over websocket to keyboard, monitor, and mouse for a Server. The provided URL is valid for 1 minute, after this period a new url needs to be created to connect to the Server. How long the connection is open after the initial connect is not subject to this timeout.


    Sends a `POST` request to `/servers/{id}/actions/request_console`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn request_server_console<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<
        ResponseValue<types::RequestServerConsoleResponse>,
        Error<types::RequestServerConsoleResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions/request_console",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "request_server_console",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Reset a Server

    Cuts power to a Server and starts it again. This forcefully stops it without giving the Server operating system time to gracefully stop. This may lead to data loss, it’s equivalent to pulling the power cord and plugging it in again. Reset should only be used when reboot does not work.


    Sends a `POST` request to `/servers/{id}/actions/reset`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn reset_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ResetServerResponse>> {
        let url = format!(
            "{}/servers/{}/actions/reset",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "reset_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Reset root Password of a Server

    Resets the root password. Only works for Linux systems that are running the qemu guest agent. Server must be powered on (status `running`) in order for this operation to succeed.

    This will generate a new password for this Server and return it.

    If this does not succeed you can use the rescue system to netboot the Server and manually change your Server password by hand.


    Sends a `POST` request to `/servers/{id}/actions/reset_password`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn reset_server_password<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<
        ResponseValue<types::ResetServerPasswordResponse>,
        Error<types::ResetServerPasswordResponse>,
    > {
        let url = format!(
            "{}/servers/{}/actions/reset_password",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "reset_server_password",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Shutdown a Server

    Shuts down a Server gracefully by sending an ACPI shutdown request. The Server operating system must support ACPI
    and react to the request, otherwise the Server will not shut down. Please note that the `action` status in this case
    only reflects whether the action was sent to the server. It does not mean that the server actually shut down
    successfully. If you need to ensure that the server is off, use the `poweroff` action.


    Sends a `POST` request to `/servers/{id}/actions/shutdown`

    Arguments:
    - `id`: ID of the Server.
    */
    pub async fn shutdown_server<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::ShutdownServerResponse>> {
        let url = format!(
            "{}/servers/{}/actions/shutdown",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "shutdown_server",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an Action for a Server

    **Deprecated**: This operation is deprecated, see our [changelog](https://docs.hetzner.cloud/changelog#2026-04-30-deprecate-get-resource-action-endpoints) for more details.

    Returns a specific Action object for a Server.


    Sends a `GET` request to `/servers/{id}/actions/{action_id}`

    Arguments:
    - `id`: ID of the Server.
    - `action_id`: ID of the Action.
    */
    pub async fn get_server_action<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        action_id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::ActionResponse>, Error<types::GetServerActionResponse>> {
        let url = format!(
            "{}/servers/{}/actions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&action_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_server_action",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Metrics for a Server

    Get Metrics for specified Server.

    You must specify the type of metric to get: cpu, disk or network. You can also specify more than one type by comma separation, e.g. cpu,disk.

    Depending on the type you will get different time series data

    | Type    | Timeseries              | Unit      | Description                                          |
    |---------|-------------------------|-----------|------------------------------------------------------|
    | cpu     | cpu                     | percent   | Percent CPU usage                                    |
    | disk    | disk.0.iops.read        | iop/s     | Number of read IO operations per second              |
    |         | disk.0.iops.write       | iop/s     | Number of write IO operations per second             |
    |         | disk.0.bandwidth.read   | bytes/s   | Bytes read per second                                |
    |         | disk.0.bandwidth.write  | bytes/s   | Bytes written per second                             |
    | network | network.0.pps.in        | packets/s | Public Network interface packets per second received |
    |         | network.0.pps.out       | packets/s | Public Network interface packets per second sent     |
    |         | network.0.bandwidth.in  | bytes/s   | Public Network interface bytes/s received            |
    |         | network.0.bandwidth.out | bytes/s   | Public Network interface bytes/s sent                |

    Note that the CPU value is not normalized to a fixed 0–100% scale. It is calculated per vCPU: a value of 100 represents full utilization on a 1-vCPU server, and 25% utilization on a 4-vCPU server. To calculate the overall CPU usage, divide the metric value by the number of vCPUs.

    Metrics are available for the last 30 days only.

    If you do not provide the step argument we will automatically adjust it so that a maximum of 200 samples are returned.

    We limit the number of samples returned to a maximum of 500 and will adjust the step parameter accordingly.


    Sends a `GET` request to `/servers/{id}/metrics`

    Arguments:
    - `id`: ID of the Server.
    - `end`: End of period to get Metrics for (must be in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
    - `start`: Start of period to get Metrics for (must be in [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) format).
    - `step`: Resolution of results in seconds.
    - `type_`: Type of metrics to get.
    */
    pub async fn get_server_metrics<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        end: &'a str,
        start: &'a str,
        step: Option<&'a str>,
        type_: &'a ::std::vec::Vec<types::GetServerMetricsTypeItem>,
    ) -> Result<
        ResponseValue<types::GetServerMetricsResponse>,
        Error<types::GetServerMetricsResponse>,
    > {
        let url = format!(
            "{}/servers/{}/metrics",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .query(&progenitor_client::QueryParam::new("step", &step))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_server_metrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List SSH keys

    Returns all SSH key objects.


    Sends a `GET` request to `/ssh_keys`

    Arguments:
    - `fingerprint`: May be used to filter SSH keys by their fingerprint. The response will only contain the SSH key matching the specified fingerprint.
    - `label_selector`: Filter resources by labels.

    The response will only contain resources matching the label selector.
    For more information, see "[Label Selector](#description/label-selector)".

    - `name`: Filter resources by their name.

    The response will only contain the resources
    matching exactly the specified name.

    - `page`: Page number to return. For more information, see "[Pagination](#description/pagination)".
    - `per_page`: Maximum number of entries returned per page. For more information, see "[Pagination](#description/pagination)".
    - `sort`: Sort resources by field and direction. May be used multiple times.

    For more information, see "[Sorting](#description/sorting)".

    */
    pub async fn list_ssh_keys<'a>(
        &'a self,
        fingerprint: Option<&'a str>,
        label_selector: Option<&'a str>,
        name: Option<&'a str>,
        page: Option<::std::num::NonZeroU64>,
        per_page: Option<::std::num::NonZeroU64>,
        sort: Option<&'a ::std::vec::Vec<types::ListSshKeysSortItem>>,
    ) -> Result<ResponseValue<types::ListSshKeysResponse>, Error<types::ListSshKeysResponse>> {
        let url = format!("{}/ssh_keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fingerprint",
                &fingerprint,
            ))
            .query(&progenitor_client::QueryParam::new(
                "label_selector",
                &label_selector,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("per_page", &per_page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_ssh_keys",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an SSH key

    Creates a new SSH key with the given `name` and `public_key`. Once an SSH key is created, it can be used in other calls such as creating Servers.


    Sends a `POST` request to `/ssh_keys`

    */
    pub async fn create_ssh_key<'a>(
        &'a self,
        body: &'a types::CreateSshKeyBody,
    ) -> Result<ResponseValue<types::CreateSshKeyResponse>, Error<types::CreateSshKeyResponse>>
    {
        let url = format!("{}/ssh_keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_ssh_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a SSH key

    Returns a specific SSH key object.


    Sends a `GET` request to `/ssh_keys/{id}`

    Arguments:
    - `id`: ID of the SSH Key.
    */
    pub async fn get_ssh_key<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<types::GetSshKeyResponse>, Error<types::GetSshKeyResponse>> {
        let url = format!("{}/ssh_keys/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_ssh_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update an SSH key

    Updates an SSH key. You can update an SSH key name and an SSH key labels.


    Sends a `PUT` request to `/ssh_keys/{id}`

    Arguments:
    - `id`: ID of the SSH Key.
    - `body`
    */
    pub async fn update_ssh_key<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
        body: &'a types::UpdateSshKeyBody,
    ) -> Result<ResponseValue<types::UpdateSshKeyResponse>, Error<types::UpdateSshKeyResponse>>
    {
        let url = format!("{}/ssh_keys/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_ssh_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete an SSH key

    Deletes an SSH key. It cannot be used anymore.


    Sends a `DELETE` request to `/ssh_keys/{id}`

    Arguments:
    - `id`: ID of the SSH Key.
    */
    pub async fn delete_ssh_key<'a>(
        &'a self,
        id: ::std::num::NonZeroU64,
    ) -> Result<ResponseValue<()>, Error<types::DeleteSshKeyResponse>> {
        let url = format!("{}/ssh_keys/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_ssh_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
