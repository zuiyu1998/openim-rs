// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub notification: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub introduction: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub face_url: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub owner_user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub create_time: i64,
    #[prost(uint32, tag = "8")]
    pub member_count: u32,
    #[prost(string, tag = "9")]
    pub ex: ::prost::alloc::string::String,
    #[prost(int32, tag = "10")]
    pub status: i32,
    #[prost(string, tag = "11")]
    pub creator_user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "12")]
    pub group_type: i32,
    #[prost(int32, tag = "13")]
    pub need_verification: i32,
    #[prost(int32, tag = "14")]
    pub look_member_info: i32,
    #[prost(int32, tag = "15")]
    pub apply_member_friend: i32,
    #[prost(int64, tag = "16")]
    pub notification_update_time: i64,
    #[prost(string, tag = "17")]
    pub notification_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoForSet {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub notification: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub introduction: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub face_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub ex: ::core::option::Option<super::openim_protobuf::StringValue>,
    #[prost(message, optional, tag = "7")]
    pub need_verification: ::core::option::Option<super::openim_protobuf::Int32Value>,
    #[prost(message, optional, tag = "8")]
    pub look_member_info: ::core::option::Option<super::openim_protobuf::Int32Value>,
    #[prost(message, optional, tag = "9")]
    pub apply_member_friend: ::core::option::Option<super::openim_protobuf::Int32Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberFullInfo {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub role_level: i32,
    #[prost(int64, tag = "4")]
    pub join_time: i64,
    #[prost(string, tag = "5")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub face_url: ::prost::alloc::string::String,
    /// if >0
    #[prost(int32, tag = "7")]
    pub app_manger_level: i32,
    #[prost(int32, tag = "8")]
    pub join_source: i32,
    #[prost(string, tag = "9")]
    pub operator_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub ex: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub mute_end_time: i64,
    #[prost(string, tag = "12")]
    pub inviter_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicUserInfo {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub face_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ex: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub face_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ex: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    #[prost(int32, tag = "6")]
    pub app_manger_level: i32,
    #[prost(int32, tag = "7")]
    pub global_recv_msg_opt: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoWithEx {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub nickname: ::core::option::Option<super::openim_protobuf::StringValue>,
    #[prost(message, optional, tag = "3")]
    pub face_url: ::core::option::Option<super::openim_protobuf::StringValue>,
    #[prost(message, optional, tag = "4")]
    pub ex: ::core::option::Option<super::openim_protobuf::StringValue>,
    #[prost(message, optional, tag = "7")]
    pub global_recv_msg_opt: ::core::option::Option<super::openim_protobuf::Int32Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendInfo {
    #[prost(string, tag = "1")]
    pub owner_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub remark: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub create_time: i64,
    #[prost(message, optional, tag = "4")]
    pub friend_user: ::core::option::Option<UserInfo>,
    #[prost(int32, tag = "5")]
    pub add_source: i32,
    #[prost(string, tag = "6")]
    pub operator_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub ex: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub is_pinned: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlackInfo {
    #[prost(string, tag = "1")]
    pub owner_user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub create_time: i64,
    #[prost(message, optional, tag = "3")]
    pub black_user_info: ::core::option::Option<PublicUserInfo>,
    #[prost(int32, tag = "4")]
    pub add_source: i32,
    #[prost(string, tag = "5")]
    pub operator_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub ex: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRequest {
    #[prost(message, optional, tag = "1")]
    pub user_info: ::core::option::Option<PublicUserInfo>,
    #[prost(message, optional, tag = "2")]
    pub group_info: ::core::option::Option<GroupInfo>,
    #[prost(int32, tag = "3")]
    pub handle_result: i32,
    #[prost(string, tag = "4")]
    pub req_msg: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub handle_msg: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub req_time: i64,
    #[prost(string, tag = "7")]
    pub handle_user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "8")]
    pub handle_time: i64,
    #[prost(string, tag = "9")]
    pub ex: ::prost::alloc::string::String,
    #[prost(int32, tag = "10")]
    pub join_source: i32,
    #[prost(string, tag = "11")]
    pub inviter_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendRequest {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from_nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub from_face_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub to_nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub to_face_url: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub handle_result: i32,
    #[prost(string, tag = "8")]
    pub req_msg: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub create_time: i64,
    #[prost(string, tag = "10")]
    pub handler_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub handle_msg: ::prost::alloc::string::String,
    #[prost(int64, tag = "12")]
    pub handle_time: i64,
    #[prost(string, tag = "13")]
    pub ex: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullMessageBySeqsReq {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub seq_ranges: ::prost::alloc::vec::Vec<SeqRange>,
    #[prost(enumeration = "PullOrder", tag = "3")]
    pub order: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeqRange {
    #[prost(string, tag = "1")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub begin: i64,
    #[prost(int64, tag = "3")]
    pub end: i64,
    #[prost(int64, tag = "4")]
    pub num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullMsgs {
    #[prost(message, repeated, tag = "1")]
    pub msgs: ::prost::alloc::vec::Vec<MsgData>,
    #[prost(bool, tag = "2")]
    pub is_end: bool,
    #[prost(int64, tag = "3")]
    pub end_seq: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullMessageBySeqsResp {
    #[prost(map = "string, message", tag = "1")]
    pub msgs: ::std::collections::HashMap<::prost::alloc::string::String, PullMsgs>,
    #[prost(map = "string, message", tag = "2")]
    pub notification_msgs: ::std::collections::HashMap<::prost::alloc::string::String, PullMsgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaxSeqReq {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaxSeqResp {
    #[prost(map = "string, int64", tag = "1")]
    pub max_seqs: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map = "string, int64", tag = "2")]
    pub min_seqs: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSendMsgResp {
    #[prost(string, tag = "1")]
    pub server_msg_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_msg_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub send_time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgData {
    #[prost(string, tag = "1")]
    pub send_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recv_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub client_msg_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub server_msg_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub sender_platform_id: i32,
    #[prost(string, tag = "7")]
    pub sender_nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub sender_face_url: ::prost::alloc::string::String,
    #[prost(int32, tag = "9")]
    pub session_type: i32,
    #[prost(int32, tag = "10")]
    pub msg_from: i32,
    #[prost(int32, tag = "11")]
    pub content_type: i32,
    #[prost(bytes = "vec", tag = "12")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "14")]
    pub seq: i64,
    #[prost(int64, tag = "15")]
    pub send_time: i64,
    #[prost(int64, tag = "16")]
    pub create_time: i64,
    #[prost(int32, tag = "17")]
    pub status: i32,
    #[prost(bool, tag = "18")]
    pub is_read: bool,
    #[prost(map = "string, bool", tag = "19")]
    pub options: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
    #[prost(message, optional, tag = "20")]
    pub offline_push_info: ::core::option::Option<OfflinePushInfo>,
    #[prost(string, repeated, tag = "21")]
    pub at_user_id_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "22")]
    pub attached_info: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub ex: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMessages {
    #[prost(map = "string, message", tag = "1")]
    pub msgs: ::std::collections::HashMap<::prost::alloc::string::String, PullMsgs>,
    #[prost(map = "string, message", tag = "2")]
    pub notification_msgs: ::std::collections::HashMap<::prost::alloc::string::String, PullMsgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflinePushInfo {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ex: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub i_os_push_sound: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub i_os_badge_count: bool,
    #[prost(string, tag = "6")]
    pub signal_info: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TipsComm {
    #[prost(bytes = "vec", tag = "1")]
    pub detail: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub default_tips: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub json_detail: ::prost::alloc::string::String,
}
/// 	OnGroupCreated()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreatedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, repeated, tag = "3")]
    pub member_list: ::prost::alloc::vec::Vec<GroupMemberFullInfo>,
    #[prost(int64, tag = "4")]
    pub operation_time: i64,
    #[prost(message, optional, tag = "5")]
    pub group_owner_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(uint64, tag = "6")]
    pub group_member_version: u64,
    #[prost(string, tag = "7")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
/// 	OnGroupInfoSet()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoSetTips {
    /// who do this
    #[prost(message, optional, tag = "1")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "2")]
    pub mute_time: i64,
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(uint64, tag = "4")]
    pub group_member_version: u64,
    #[prost(string, tag = "5")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoSetNameTips {
    /// who do this
    #[prost(message, optional, tag = "1")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(uint64, tag = "3")]
    pub group_member_version: u64,
    #[prost(string, tag = "4")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfoSetAnnouncementTips {
    /// who do this
    #[prost(message, optional, tag = "1")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(uint64, tag = "3")]
    pub group_member_version: u64,
    #[prost(string, tag = "4")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
/// 	OnJoinGroupApplication()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupApplicationTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub applicant: ::core::option::Option<PublicUserInfo>,
    #[prost(string, tag = "3")]
    pub req_msg: ::prost::alloc::string::String,
}
/// 	OnQuitGroup()
/// Actively leave the group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberQuitTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub quit_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(uint64, tag = "4")]
    pub group_member_version: u64,
    #[prost(string, tag = "5")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
/// 	OnApplicationGroupAccepted()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupApplicationAcceptedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(string, tag = "4")]
    pub handle_msg: ::prost::alloc::string::String,
    /// admin(==1) or applicant(==0)
    #[prost(int32, tag = "5")]
    pub receiver_as: i32,
}
/// 	OnApplicationGroupRejected()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupApplicationRejectedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(string, tag = "4")]
    pub handle_msg: ::prost::alloc::string::String,
    /// admin(==1) or applicant(==0)
    #[prost(int32, tag = "5")]
    pub receiver_as: i32,
}
/// 	OnTransferGroupOwner()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupOwnerTransferredTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, optional, tag = "3")]
    pub new_group_owner: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(string, tag = "4")]
    pub old_group_owner: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub operation_time: i64,
    #[prost(message, optional, tag = "6")]
    pub old_group_owner_info: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(uint64, tag = "7")]
    pub group_member_version: u64,
    #[prost(string, tag = "8")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
/// 	OnMemberKicked()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberKickedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, repeated, tag = "3")]
    pub kicked_user_list: ::prost::alloc::vec::Vec<GroupMemberFullInfo>,
    #[prost(int64, tag = "4")]
    pub operation_time: i64,
    #[prost(uint64, tag = "5")]
    pub group_member_version: u64,
    #[prost(string, tag = "6")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
/// 	OnMemberInvited()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberInvitedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(message, repeated, tag = "3")]
    pub invited_user_list: ::prost::alloc::vec::Vec<GroupMemberFullInfo>,
    #[prost(int64, tag = "4")]
    pub operation_time: i64,
    #[prost(uint64, tag = "5")]
    pub group_member_version: u64,
    #[prost(string, tag = "6")]
    pub group_member_version_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub inviter_user: ::core::option::Option<GroupMemberFullInfo>,
}
/// Actively join the group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberEnterTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub entrant_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(uint64, tag = "5")]
    pub group_member_version: u64,
    #[prost(string, tag = "6")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDismissedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberMutedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(message, optional, tag = "4")]
    pub muted_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(uint32, tag = "5")]
    pub muted_seconds: u32,
    #[prost(uint64, tag = "6")]
    pub group_member_version: u64,
    #[prost(string, tag = "7")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberCancelMutedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(message, optional, tag = "4")]
    pub muted_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(uint64, tag = "5")]
    pub group_member_version: u64,
    #[prost(string, tag = "6")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMutedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(uint64, tag = "4")]
    pub group_member_version: u64,
    #[prost(string, tag = "5")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCancelMutedTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(uint64, tag = "4")]
    pub group_member_version: u64,
    #[prost(string, tag = "5")]
    pub group_member_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberInfoSetTips {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
    #[prost(message, optional, tag = "2")]
    pub op_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(int64, tag = "3")]
    pub operation_time: i64,
    #[prost(message, optional, tag = "4")]
    pub changed_user: ::core::option::Option<GroupMemberFullInfo>,
    #[prost(uint64, tag = "5")]
    pub group_member_version: u64,
    #[prost(string, tag = "6")]
    pub group_member_version_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub group_sort_version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApplication {
    #[prost(int64, tag = "1")]
    pub add_time: i64,
    #[prost(string, tag = "2")]
    pub add_source: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub add_wording: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromToUserId {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
}
/// FromUserID apply to add ToUserID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApplicationTips {
    /// from：发起者； to：接收者
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
}
/// FromUserID accept or reject ToUserID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApplicationApprovedTips {
    /// from：同意者；to：请求发起者
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
    #[prost(string, tag = "2")]
    pub handle_msg: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub friend_version: u64,
    #[prost(string, tag = "4")]
    pub friend_version_id: ::prost::alloc::string::String,
}
/// FromUserID accept or reject ToUserID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApplicationRejectedTips {
    /// from：拒绝者；to：请求发起者
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
    #[prost(string, tag = "2")]
    pub handle_msg: ::prost::alloc::string::String,
}
/// FromUserID  Added a friend ToUserID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendAddedTips {
    #[prost(message, optional, tag = "1")]
    pub friend: ::core::option::Option<FriendInfo>,
    #[prost(int64, tag = "2")]
    pub operation_time: i64,
    /// who do this
    #[prost(message, optional, tag = "3")]
    pub op_user: ::core::option::Option<PublicUserInfo>,
    #[prost(uint64, tag = "4")]
    pub friend_version: u64,
    #[prost(string, tag = "5")]
    pub friend_version_id: ::prost::alloc::string::String,
}
/// FromUserID  deleted a friend ToUserID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendDeletedTips {
    /// from：owner； to：friend
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
    #[prost(uint64, tag = "2")]
    pub friend_version: u64,
    #[prost(string, tag = "3")]
    pub friend_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlackAddedTips {
    /// from：owner； to：black
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlackDeletedTips {
    /// from：owner； to：black
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendInfoChangedTips {
    /// from：changed； to：friend
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
    #[prost(uint64, tag = "2")]
    pub friend_version: u64,
    #[prost(string, tag = "3")]
    pub friend_version_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub friend_sort_version: u64,
}
/// ////////////////////user/////////////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfoUpdatedTips {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStatusChangeTips {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub status: i32,
    #[prost(int32, tag = "4")]
    pub platform_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCommandAddTips {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCommandUpdateTips {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCommandDeleteTips {
    #[prost(string, tag = "1")]
    pub from_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
}
/// ////////////////////conversation/////////////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationUpdateTips {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub conversation_id_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationSetPrivateTips {
    #[prost(string, tag = "1")]
    pub recv_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub send_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_private: bool,
    #[prost(string, tag = "4")]
    pub conversation_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationHasReadTips {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub has_read_seq: i64,
    #[prost(int64, tag = "4")]
    pub unread_count_time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationElem {
    #[prost(string, tag = "1")]
    pub detail: ::prost::alloc::string::String,
}
/// //////////////////message///////////////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Seqs {
    #[prost(int64, repeated, tag = "1")]
    pub seqs: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageTips {
    #[prost(string, tag = "1")]
    pub op_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "3")]
    pub seqs: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeMsgTips {
    #[prost(string, tag = "1")]
    pub revoker_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_msg_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub revoke_time: i64,
    #[prost(int32, tag = "5")]
    pub sesstion_type: i32,
    #[prost(int64, tag = "6")]
    pub seq: i64,
    #[prost(string, tag = "7")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub is_admin_revoke: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRevokedContent {
    #[prost(string, tag = "1")]
    pub revoker_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub revoker_role: i32,
    #[prost(string, tag = "3")]
    pub client_msg_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub revoker_nickname: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub revoke_time: i64,
    #[prost(int64, tag = "6")]
    pub source_message_send_time: i64,
    #[prost(string, tag = "7")]
    pub source_message_send_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub source_message_sender_nickname: ::prost::alloc::string::String,
    #[prost(int32, tag = "10")]
    pub session_type: i32,
    #[prost(int64, tag = "11")]
    pub seq: i64,
    #[prost(string, tag = "12")]
    pub ex: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearConversationTips {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub conversation_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMsgsTips {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "3")]
    pub seqs: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAsReadTips {
    #[prost(string, tag = "1")]
    pub mark_as_read_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "3")]
    pub seqs: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "4")]
    pub has_read_seq: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAppBackgroundStatusReq {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_background: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SetAppBackgroundStatusResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessUserCommand {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    #[prost(int64, tag = "3")]
    pub create_time: i64,
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RequestPagination {
    #[prost(int32, tag = "1")]
    pub page_number: i32,
    #[prost(int32, tag = "2")]
    pub show_number: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendsInfoUpdateTips {
    #[prost(message, optional, tag = "1")]
    pub from_to_user_id: ::core::option::Option<FromToUserId>,
    #[prost(string, repeated, tag = "2")]
    pub friend_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "3")]
    pub friend_version: u64,
    #[prost(string, tag = "4")]
    pub friend_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubUserOnlineStatusElem {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag = "2")]
    pub online_platform_i_ds: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubUserOnlineStatusTips {
    #[prost(message, repeated, tag = "1")]
    pub subscribers: ::prost::alloc::vec::Vec<SubUserOnlineStatusElem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubUserOnlineStatus {
    #[prost(string, repeated, tag = "1")]
    pub subscribe_user_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub unsubscribe_user_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMsgTips {
    #[prost(string, tag = "1")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_msg_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub start_index: i64,
    #[prost(string, repeated, tag = "4")]
    pub packets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "5")]
    pub end: bool,
}
/// /////////////////////////////////base end/////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PullOrder {
    Asc = 0,
    Desc = 1,
}
impl PullOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Asc => "PullOrderAsc",
            Self::Desc => "PullOrderDesc",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PullOrderAsc" => Some(Self::Asc),
            "PullOrderDesc" => Some(Self::Desc),
            _ => None,
        }
    }
}
