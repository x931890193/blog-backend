// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.19.4
// source: all.proto

package proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type LoginAdminRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Username string `protobuf:"bytes,1,opt,name=username,proto3" json:"username,omitempty"`
	Password string `protobuf:"bytes,2,opt,name=password,proto3" json:"password,omitempty"`
}

func (x *LoginAdminRequest) Reset() {
	*x = LoginAdminRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_all_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *LoginAdminRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*LoginAdminRequest) ProtoMessage() {}

func (x *LoginAdminRequest) ProtoReflect() protoreflect.Message {
	mi := &file_all_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use LoginAdminRequest.ProtoReflect.Descriptor instead.
func (*LoginAdminRequest) Descriptor() ([]byte, []int) {
	return file_all_proto_rawDescGZIP(), []int{0}
}

func (x *LoginAdminRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *LoginAdminRequest) GetPassword() string {
	if x != nil {
		return x.Password
	}
	return ""
}

type LoginAdminResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Code  uint32 `protobuf:"varint,1,opt,name=code,proto3" json:"code,omitempty"`
	Msg   string `protobuf:"bytes,2,opt,name=msg,proto3" json:"msg,omitempty"`
	Token string `protobuf:"bytes,3,opt,name=token,proto3" json:"token,omitempty"`
}

func (x *LoginAdminResp) Reset() {
	*x = LoginAdminResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_all_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *LoginAdminResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*LoginAdminResp) ProtoMessage() {}

func (x *LoginAdminResp) ProtoReflect() protoreflect.Message {
	mi := &file_all_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use LoginAdminResp.ProtoReflect.Descriptor instead.
func (*LoginAdminResp) Descriptor() ([]byte, []int) {
	return file_all_proto_rawDescGZIP(), []int{1}
}

func (x *LoginAdminResp) GetCode() uint32 {
	if x != nil {
		return x.Code
	}
	return 0
}

func (x *LoginAdminResp) GetMsg() string {
	if x != nil {
		return x.Msg
	}
	return ""
}

func (x *LoginAdminResp) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

type LogoutAdminRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
}

func (x *LogoutAdminRequest) Reset() {
	*x = LogoutAdminRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_all_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *LogoutAdminRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*LogoutAdminRequest) ProtoMessage() {}

func (x *LogoutAdminRequest) ProtoReflect() protoreflect.Message {
	mi := &file_all_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use LogoutAdminRequest.ProtoReflect.Descriptor instead.
func (*LogoutAdminRequest) Descriptor() ([]byte, []int) {
	return file_all_proto_rawDescGZIP(), []int{2}
}

func (x *LogoutAdminRequest) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

type LogoutAdminResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Code uint32 `protobuf:"varint,1,opt,name=code,proto3" json:"code,omitempty"`
	Msg  string `protobuf:"bytes,2,opt,name=msg,proto3" json:"msg,omitempty"`
}

func (x *LogoutAdminResp) Reset() {
	*x = LogoutAdminResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_all_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *LogoutAdminResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*LogoutAdminResp) ProtoMessage() {}

func (x *LogoutAdminResp) ProtoReflect() protoreflect.Message {
	mi := &file_all_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use LogoutAdminResp.ProtoReflect.Descriptor instead.
func (*LogoutAdminResp) Descriptor() ([]byte, []int) {
	return file_all_proto_rawDescGZIP(), []int{3}
}

func (x *LogoutAdminResp) GetCode() uint32 {
	if x != nil {
		return x.Code
	}
	return 0
}

func (x *LogoutAdminResp) GetMsg() string {
	if x != nil {
		return x.Msg
	}
	return ""
}

type AdminInfoResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name             string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	Avatar           string `protobuf:"bytes,2,opt,name=avatar,proto3" json:"avatar,omitempty"`
	Job              string `protobuf:"bytes,3,opt,name=job,proto3" json:"job,omitempty"`
	Organization     string `protobuf:"bytes,4,opt,name=organization,proto3" json:"organization,omitempty"`
	Location         string `protobuf:"bytes,5,opt,name=location,proto3" json:"location,omitempty"`
	Email            string `protobuf:"bytes,6,opt,name=email,proto3" json:"email,omitempty"`
	Introduction     string `protobuf:"bytes,7,opt,name=introduction,proto3" json:"introduction,omitempty"`
	PersonalWebsite  string `protobuf:"bytes,8,opt,name=personalWebsite,proto3" json:"personalWebsite,omitempty"`
	JobName          string `protobuf:"bytes,9,opt,name=jobName,proto3" json:"jobName,omitempty"`
	OrganizationName string `protobuf:"bytes,10,opt,name=organizationName,proto3" json:"organizationName,omitempty"`
	LocationName     string `protobuf:"bytes,11,opt,name=locationName,proto3" json:"locationName,omitempty"`
	Phone            string `protobuf:"bytes,12,opt,name=phone,proto3" json:"phone,omitempty"`
	RegistrationDate string `protobuf:"bytes,13,opt,name=registrationDate,proto3" json:"registrationDate,omitempty"`
	AccountId        string `protobuf:"bytes,14,opt,name=accountId,proto3" json:"accountId,omitempty"`
	Certification    string `protobuf:"bytes,15,opt,name=certification,proto3" json:"certification,omitempty"`
	Role             string `protobuf:"bytes,16,opt,name=role,proto3" json:"role,omitempty"`
	Code             uint32 `protobuf:"varint,17,opt,name=code,proto3" json:"code,omitempty"`
	Msg              string `protobuf:"bytes,18,opt,name=msg,proto3" json:"msg,omitempty"`
}

func (x *AdminInfoResp) Reset() {
	*x = AdminInfoResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_all_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AdminInfoResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AdminInfoResp) ProtoMessage() {}

func (x *AdminInfoResp) ProtoReflect() protoreflect.Message {
	mi := &file_all_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AdminInfoResp.ProtoReflect.Descriptor instead.
func (*AdminInfoResp) Descriptor() ([]byte, []int) {
	return file_all_proto_rawDescGZIP(), []int{4}
}

func (x *AdminInfoResp) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *AdminInfoResp) GetAvatar() string {
	if x != nil {
		return x.Avatar
	}
	return ""
}

func (x *AdminInfoResp) GetJob() string {
	if x != nil {
		return x.Job
	}
	return ""
}

func (x *AdminInfoResp) GetOrganization() string {
	if x != nil {
		return x.Organization
	}
	return ""
}

func (x *AdminInfoResp) GetLocation() string {
	if x != nil {
		return x.Location
	}
	return ""
}

func (x *AdminInfoResp) GetEmail() string {
	if x != nil {
		return x.Email
	}
	return ""
}

func (x *AdminInfoResp) GetIntroduction() string {
	if x != nil {
		return x.Introduction
	}
	return ""
}

func (x *AdminInfoResp) GetPersonalWebsite() string {
	if x != nil {
		return x.PersonalWebsite
	}
	return ""
}

func (x *AdminInfoResp) GetJobName() string {
	if x != nil {
		return x.JobName
	}
	return ""
}

func (x *AdminInfoResp) GetOrganizationName() string {
	if x != nil {
		return x.OrganizationName
	}
	return ""
}

func (x *AdminInfoResp) GetLocationName() string {
	if x != nil {
		return x.LocationName
	}
	return ""
}

func (x *AdminInfoResp) GetPhone() string {
	if x != nil {
		return x.Phone
	}
	return ""
}

func (x *AdminInfoResp) GetRegistrationDate() string {
	if x != nil {
		return x.RegistrationDate
	}
	return ""
}

func (x *AdminInfoResp) GetAccountId() string {
	if x != nil {
		return x.AccountId
	}
	return ""
}

func (x *AdminInfoResp) GetCertification() string {
	if x != nil {
		return x.Certification
	}
	return ""
}

func (x *AdminInfoResp) GetRole() string {
	if x != nil {
		return x.Role
	}
	return ""
}

func (x *AdminInfoResp) GetCode() uint32 {
	if x != nil {
		return x.Code
	}
	return 0
}

func (x *AdminInfoResp) GetMsg() string {
	if x != nil {
		return x.Msg
	}
	return ""
}

var File_all_proto protoreflect.FileDescriptor

var file_all_proto_rawDesc = []byte{
	0x0a, 0x09, 0x61, 0x6c, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0x4b, 0x0a, 0x11, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x41, 0x64, 0x6d, 0x69, 0x6e,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e,
	0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e,
	0x61, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x22,
	0x4c, 0x0a, 0x0e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x73,
	0x70, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
	0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
	0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x2a, 0x0a,
	0x12, 0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75,
	0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x37, 0x0a, 0x0f, 0x4c, 0x6f, 0x67,
	0x6f, 0x75, 0x74, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x12, 0x12, 0x0a, 0x04,
	0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65,
	0x12, 0x10, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6d,
	0x73, 0x67, 0x22, 0x9b, 0x04, 0x0a, 0x0d, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
	0x52, 0x65, 0x73, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x76, 0x61, 0x74,
	0x61, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72,
	0x12, 0x10, 0x0a, 0x03, 0x6a, 0x6f, 0x62, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6a,
	0x6f, 0x62, 0x12, 0x22, 0x0a, 0x0c, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69,
	0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12, 0x22, 0x0a, 0x0c, 0x69, 0x6e, 0x74, 0x72,
	0x6f, 0x64, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c,
	0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x0f,
	0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x6c, 0x57, 0x65, 0x62, 0x73, 0x69, 0x74, 0x65, 0x18,
	0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x6c, 0x57,
	0x65, 0x62, 0x73, 0x69, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6a, 0x6f, 0x62, 0x4e, 0x61, 0x6d,
	0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6a, 0x6f, 0x62, 0x4e, 0x61, 0x6d, 0x65,
	0x12, 0x2a, 0x0a, 0x10, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x4e, 0x61, 0x6d, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x6f, 0x72, 0x67, 0x61,
	0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x22, 0x0a, 0x0c,
	0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x0c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4e, 0x61, 0x6d, 0x65,
	0x12, 0x14, 0x0a, 0x05, 0x70, 0x68, 0x6f, 0x6e, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x05, 0x70, 0x68, 0x6f, 0x6e, 0x65, 0x12, 0x2a, 0x0a, 0x10, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
	0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x10, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61,
	0x74, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x18,
	0x0e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64,
	0x12, 0x24, 0x0a, 0x0d, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69,
	0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x10,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f,
	0x64, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x10,
	0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x12, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67,
	0x42, 0x0a, 0x5a, 0x08, 0x2e, 0x2e, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_all_proto_rawDescOnce sync.Once
	file_all_proto_rawDescData = file_all_proto_rawDesc
)

func file_all_proto_rawDescGZIP() []byte {
	file_all_proto_rawDescOnce.Do(func() {
		file_all_proto_rawDescData = protoimpl.X.CompressGZIP(file_all_proto_rawDescData)
	})
	return file_all_proto_rawDescData
}

var file_all_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_all_proto_goTypes = []interface{}{
	(*LoginAdminRequest)(nil),  // 0: proto.LoginAdminRequest
	(*LoginAdminResp)(nil),     // 1: proto.LoginAdminResp
	(*LogoutAdminRequest)(nil), // 2: proto.LogoutAdminRequest
	(*LogoutAdminResp)(nil),    // 3: proto.LogoutAdminResp
	(*AdminInfoResp)(nil),      // 4: proto.AdminInfoResp
}
var file_all_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_all_proto_init() }
func file_all_proto_init() {
	if File_all_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_all_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*LoginAdminRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_all_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*LoginAdminResp); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_all_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*LogoutAdminRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_all_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*LogoutAdminResp); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_all_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AdminInfoResp); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_all_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_all_proto_goTypes,
		DependencyIndexes: file_all_proto_depIdxs,
		MessageInfos:      file_all_proto_msgTypes,
	}.Build()
	File_all_proto = out.File
	file_all_proto_rawDesc = nil
	file_all_proto_goTypes = nil
	file_all_proto_depIdxs = nil
}
