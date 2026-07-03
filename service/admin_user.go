package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/utils/crypt"
	"encoding/base64"
	"errors"
	"strings"
	"time"
)

const defaultAdminAvatar = "https://www.bytealien.com/favicon.ico"

func AdminUserList(pageSize, currentPage int, username, phone, status, beginTime, endTime string) (*pb.AdminUserListResp, error) {
	pageSize, currentPage = normalizeAdminPagination(pageSize, currentPage)
	users, total, err := entity.GetAdminUsers(
		pageSize,
		currentPage,
		strings.TrimSpace(username),
		strings.TrimSpace(phone),
		strings.TrimSpace(status),
		strings.TrimSpace(beginTime),
		strings.TrimSpace(endTime),
	)
	if err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminUserBase, 0, len(users))
	for _, user := range users {
		rows = append(rows, adminUserToPB(user))
	}
	return &pb.AdminUserListResp{
		Total: uint32(total),
		Rows:  rows,
	}, nil
}

func AdminUserOne(id int) (*pb.AdminUserOneResp, error) {
	user, err := entity.GetAdminUserByID(id)
	if err != nil {
		return nil, err
	}
	roleIDs := adminUserRoleIDs(user)
	return &pb.AdminUserOneResp{
		Data:    adminUserToPB(user),
		RoleIds: roleIDs,
	}, nil
}

func AdminUserProfile(token string) (*pb.AdminUserOneResp, error) {
	user, err := ParseToken(token)
	if err != nil {
		return nil, err
	}
	current, err := entity.GetAdminUserByID(user.ID)
	if err != nil {
		return nil, err
	}
	return &pb.AdminUserOneResp{
		Data:    adminUserToPB(current),
		RoleIds: adminUserRoleIDs(current),
	}, nil
}

func AdminUpdateUserProfile(token string, req *pb.AdminUserRequest) error {
	user, err := ParseToken(token)
	if err != nil {
		return err
	}
	values := map[string]interface{}{
		"receive_update": req.ReceiveUpdate,
		"updated_at":     time.Now(),
	}
	if strings.TrimSpace(req.NickName) != "" {
		values["user_name"] = strings.TrimSpace(req.NickName)
	} else if strings.TrimSpace(req.UserName) != "" {
		values["user_name"] = strings.TrimSpace(req.UserName)
	}
	if strings.TrimSpace(req.Email) != "" {
		values["email"] = strings.TrimSpace(req.Email)
	}
	if strings.TrimSpace(req.Avatar) != "" {
		values["avatar"] = strings.TrimSpace(req.Avatar)
	}
	_, err = entity.UpdateAdminUserByID(user.ID, values)
	return err
}

func AdminUpdateUserPassword(token, oldPassword, newPassword string) error {
	user, err := ParseToken(token)
	if err != nil {
		return err
	}
	current, err := entity.GetAdminUserByID(user.ID)
	if err != nil {
		return err
	}
	if strings.TrimSpace(newPassword) == "" {
		return errors.New("新密码不能为空")
	}
	if strings.TrimSpace(oldPassword) != "" && current.Password != hashAdminPassword(oldPassword) {
		return errors.New("旧密码错误")
	}
	_, err = entity.UpdateAdminUserByID(user.ID, map[string]interface{}{
		"password":   hashAdminPassword(newPassword),
		"updated_at": time.Now(),
	})
	return err
}

func AdminCreateUser(req *pb.AdminUserRequest) error {
	username := firstNonEmpty(req.UserName, req.NickName)
	password := strings.TrimSpace(req.Password)
	if password == "" {
		password = "123456"
	}
	email := strings.TrimSpace(req.Email)
	if email == "" {
		email = username + "@bytealien.local"
	}
	avatar := strings.TrimSpace(req.Avatar)
	if avatar == "" {
		avatar = defaultAdminAvatar
	}
	user := &entity.User{
		UserName:      username,
		Password:      hashAdminPassword(password),
		Avatar:        avatar,
		Phone:         strings.TrimSpace(req.Phone),
		Email:         email,
		IsAdmin:       statusToAdmin(req.Status, req.IsAdmin),
		ReceiveUpdate: true,
		LastLogin:     time.Now(),
	}
	return entity.CreateAdminUser(user)
}

func AdminUpdateUser(req *pb.AdminUserRequest) error {
	values := map[string]interface{}{
		"user_name":      firstNonEmpty(req.UserName, req.NickName),
		"email":          req.Email,
		"phone":          strings.TrimSpace(req.Phone),
		"is_admin":       statusToAdmin(req.Status, req.IsAdmin),
		"receive_update": req.ReceiveUpdate,
		"updated_at":     time.Now(),
	}
	if strings.TrimSpace(req.Avatar) != "" {
		values["avatar"] = req.Avatar
	}
	if strings.TrimSpace(req.Password) != "" {
		values["password"] = hashAdminPassword(req.Password)
	}
	_, err := entity.UpdateAdminUserByID(int(req.Id), values)
	return err
}

func AdminResetUserPassword(id int, password string) error {
	if strings.TrimSpace(password) == "" {
		password = "123456"
	}
	_, err := entity.UpdateAdminUserByID(id, map[string]interface{}{
		"password":   hashAdminPassword(password),
		"updated_at": time.Now(),
	})
	return err
}

func AdminChangeUserStatus(id int, status string) error {
	_, err := entity.UpdateAdminUserByID(id, map[string]interface{}{
		"is_admin":   statusToAdmin(status, false),
		"updated_at": time.Now(),
	})
	return err
}

func AdminDeleteUsers(ids []int) error {
	return entity.DeleteAdminUsersByIds(ids)
}

func adminUserToPB(user *entity.User) *pb.AdminUserBase {
	if user == nil {
		return &pb.AdminUserBase{}
	}
	status := "1"
	if user.IsAdmin {
		status = "0"
	}
	return &pb.AdminUserBase{
		Id:            uint32(user.ID),
		UserName:      user.UserName,
		NickName:      user.UserName,
		Email:         user.Email,
		Phone:         user.Phone,
		Status:        status,
		Avatar:        user.Avatar,
		IsAdmin:       user.IsAdmin,
		ReceiveUpdate: user.ReceiveUpdate,
		GithubUrl:     user.GitHubUrl,
		CreateTime:    user.CreatedAt.Format("2006-01-02 15:04:05"),
		LastLogin:     formatTime(user.LastLogin),
		RoleIds:       adminUserRoleIDs(user),
		Sex:           "2",
	}
}

func adminUserRoleIDs(user *entity.User) []uint32 {
	if user != nil && user.IsAdmin {
		return []uint32{1}
	}
	return []uint32{2}
}

func hashAdminPassword(password string) string {
	return crypt.Sha256(base64.StdEncoding.EncodeToString([]byte(password)))
}

func statusToAdmin(status string, fallback bool) bool {
	switch strings.TrimSpace(status) {
	case "0", "true", "admin":
		return true
	case "1", "false", "user":
		return false
	default:
		return fallback
	}
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		value = strings.TrimSpace(value)
		if value != "" {
			return value
		}
	}
	return ""
}

func normalizeAdminPagination(pageSize, currentPage int) (int, int) {
	if pageSize <= 0 {
		pageSize = 10
	}
	if pageSize > 100 {
		pageSize = 100
	}
	if currentPage <= 0 {
		currentPage = 1
	}
	return pageSize, currentPage
}

func formatTime(value time.Time) string {
	if value.IsZero() {
		return ""
	}
	return value.Format("2006-01-02 15:04:05")
}
