package service

import (
	"blog-backend/cache"
	"blog-backend/config"
	"blog-backend/model/conn"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"encoding/json"
	"net"
	"path"
	"sort"
	"strconv"
	"strings"
	"time"

	"gorm.io/gorm"
)

type AdminListRequest struct {
	PageNum  int
	PageSize int
}

func adminPage(pageSize, pageNum int) (int, int) {
	if pageSize <= 0 {
		pageSize = 10
	}
	if pageSize > 100 {
		pageSize = 100
	}
	if pageNum <= 0 {
		pageNum = 1
	}
	return pageSize, pageNum
}

func adminOffset(pageSize, pageNum int) int {
	return (pageNum - 1) * pageSize
}

func applyCreatedAtRange(query *gorm.DB, beginTime, endTime string) *gorm.DB {
	if v := strings.TrimSpace(beginTime); v != "" {
		query = query.Where("created_at >= ?", v+" 00:00:00")
	}
	if v := strings.TrimSpace(endTime); v != "" {
		query = query.Where("created_at <= ?", v+" 23:59:59")
	}
	return query
}

func formatAdminTime(value time.Time) string {
	if value.IsZero() {
		return ""
	}
	return value.Format("2006-01-02 15:04:05")
}

func menuIDsToString(ids []uint32) string {
	if len(ids) == 0 {
		return ""
	}
	parts := make([]string, 0, len(ids))
	for _, id := range ids {
		parts = append(parts, strconv.Itoa(int(id)))
	}
	return strings.Join(parts, ",")
}

func stringToMenuIDs(raw string) []uint32 {
	if strings.TrimSpace(raw) == "" {
		return nil
	}
	parts := strings.Split(raw, ",")
	ids := make([]uint32, 0, len(parts))
	for _, part := range parts {
		id, err := strconv.Atoi(strings.TrimSpace(part))
		if err == nil && id > 0 {
			ids = append(ids, uint32(id))
		}
	}
	return ids
}

func EnsureDefaultAdminRoles() error {
	var count int64
	if err := conn.MysqlConn.Model(&entity.AdminRole{}).Count(&count).Error; err != nil {
		return err
	}
	if count > 0 {
		return nil
	}
	roles := []entity.AdminRole{
		{RoleName: "管理员", RoleKey: "admin", RoleSort: 1, Status: "0", Remark: "系统管理员", MenuIds: "1,2,3,4,5"},
		{RoleName: "普通用户", RoleKey: "user", RoleSort: 2, Status: "0", Remark: "普通用户", MenuIds: "1,2"},
	}
	return conn.MysqlConn.Create(&roles).Error
}

func AdminRoleList(pageSize, pageNum int, roleName, roleKey, status, beginTime, endTime string) (*pb.AdminRoleListResp, error) {
	if err := EnsureDefaultAdminRoles(); err != nil {
		return nil, err
	}
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var roles []entity.AdminRole
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminRole{})
	if strings.TrimSpace(roleName) != "" {
		query = query.Where("role_name LIKE ?", "%"+strings.TrimSpace(roleName)+"%")
	}
	if strings.TrimSpace(roleKey) != "" {
		query = query.Where("role_key LIKE ?", "%"+strings.TrimSpace(roleKey)+"%")
	}
	if strings.TrimSpace(status) != "" {
		query = query.Where("status = ?", strings.TrimSpace(status))
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("role_sort ASC, id ASC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&roles).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminRoleBase, 0, len(roles))
	for _, role := range roles {
		rows = append(rows, &pb.AdminRoleBase{
			Id:         uint32(role.ID),
			RoleName:   role.RoleName,
			RoleKey:    role.RoleKey,
			Status:     role.Status,
			CreateTime: formatAdminTime(role.CreatedAt),
		})
	}
	return &pb.AdminRoleListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminRoleOne(id int) (*pb.AdminTableOneResp, error) {
	if err := EnsureDefaultAdminRoles(); err != nil {
		return nil, err
	}
	role := entity.AdminRole{}
	if err := conn.MysqlConn.First(&role, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: roleToTableRow(role)}, nil
}

func SaveAdminRole(req *pb.AdminTableRow) error {
	if strings.TrimSpace(req.RoleName) == "" {
		req.RoleName = req.Title
	}
	if strings.TrimSpace(req.RoleKey) == "" {
		req.RoleKey = strings.ToLower(strings.ReplaceAll(req.RoleName, " ", "_"))
	}
	role := entity.AdminRole{
		RoleName: req.RoleName,
		RoleKey:  req.RoleKey,
		RoleSort: uint(req.RoleSort),
		Status:   firstNonEmpty(req.Status, "0"),
		Remark:   req.Remark,
		MenuIds:  menuIDsToString(req.MenuIds),
	}
	return conn.MysqlConn.Create(&role).Error
}

func UpdateAdminRole(req *pb.AdminTableRow) error {
	values := map[string]interface{}{
		"role_name":  firstNonEmpty(req.RoleName, req.Title),
		"role_key":   req.RoleKey,
		"role_sort":  uint(req.RoleSort),
		"status":     firstNonEmpty(req.Status, "0"),
		"remark":     req.Remark,
		"menu_ids":   menuIDsToString(req.MenuIds),
		"updated_at": time.Now(),
	}
	return conn.MysqlConn.Model(&entity.AdminRole{}).Where("id = ?", req.Id).Updates(values).Error
}

func ChangeAdminRoleStatus(id int, status string) error {
	return conn.MysqlConn.Model(&entity.AdminRole{}).Where("id = ?", id).Updates(map[string]interface{}{
		"status":     firstNonEmpty(status, "0"),
		"updated_at": time.Now(),
	}).Error
}

func DeleteAdminRoles(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminRole{}, ids).Error
}

func roleToTableRow(role entity.AdminRole) *pb.AdminTableRow {
	return &pb.AdminTableRow{
		Id:         uint32(role.ID),
		Title:      role.RoleName,
		RoleName:   role.RoleName,
		RoleKey:    role.RoleKey,
		RoleSort:   uint32(role.RoleSort),
		Status:     role.Status,
		Remark:     role.Remark,
		MenuIds:    stringToMenuIDs(role.MenuIds),
		CreateTime: formatAdminTime(role.CreatedAt),
	}
}

func AdminNoticeList(pageSize, pageNum int, title, createBy, noticeType, status, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var notices []entity.AdminNotice
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminNotice{})
	if strings.TrimSpace(title) != "" {
		query = query.Where("title LIKE ?", "%"+strings.TrimSpace(title)+"%")
	}
	if strings.TrimSpace(createBy) != "" {
		query = query.Where("create_by LIKE ?", "%"+strings.TrimSpace(createBy)+"%")
	}
	if strings.TrimSpace(noticeType) != "" {
		query = query.Where("type = ?", noticeType)
	}
	if strings.TrimSpace(status) != "" {
		query = query.Where("status = ?", status)
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&notices).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(notices))
	for _, notice := range notices {
		rows = append(rows, &pb.AdminTableRow{
			Id:         uint32(notice.ID),
			Title:      notice.Title,
			Type:       notice.Type,
			Status:     notice.Status,
			Content:    notice.Content,
			CreateBy:   notice.CreateBy,
			CreateTime: formatAdminTime(notice.CreatedAt),
		})
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminNoticeOne(id int) (*pb.AdminTableOneResp, error) {
	notice := entity.AdminNotice{}
	if err := conn.MysqlConn.First(&notice, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: &pb.AdminTableRow{
		Id:         uint32(notice.ID),
		Title:      notice.Title,
		Type:       notice.Type,
		Status:     notice.Status,
		Content:    notice.Content,
		CreateBy:   notice.CreateBy,
		CreateTime: formatAdminTime(notice.CreatedAt),
	}}, nil
}

func SaveAdminNotice(req *pb.AdminTableRow) error {
	notice := entity.AdminNotice{
		Title:    req.Title,
		Type:     firstNonEmpty(req.Type, "1"),
		Status:   firstNonEmpty(req.Status, "0"),
		Content:  req.Content,
		CreateBy: firstNonEmpty(req.CreateBy, "admin"),
	}
	return conn.MysqlConn.Create(&notice).Error
}

func UpdateAdminNotice(req *pb.AdminTableRow) error {
	return conn.MysqlConn.Model(&entity.AdminNotice{}).Where("id = ?", req.Id).Updates(map[string]interface{}{
		"title":      req.Title,
		"type":       firstNonEmpty(req.Type, "1"),
		"status":     firstNonEmpty(req.Status, "0"),
		"content":    req.Content,
		"create_by":  firstNonEmpty(req.CreateBy, "admin"),
		"updated_at": time.Now(),
	}).Error
}

func DeleteAdminNotices(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminNotice{}, ids).Error
}

func AdminCarouselList(pageSize, pageNum int, title, description, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.AdminCarousel
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminCarousel{})
	if strings.TrimSpace(title) != "" {
		query = query.Where("title LIKE ?", "%"+strings.TrimSpace(title)+"%")
	}
	if strings.TrimSpace(description) != "" {
		query = query.Where("description LIKE ?", "%"+strings.TrimSpace(description)+"%")
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, carouselToTableRow(item))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminCarouselOne(id int) (*pb.AdminTableOneResp, error) {
	item := entity.AdminCarousel{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: carouselToTableRow(item)}, nil
}

func SaveAdminCarousel(req *pb.AdminTableRow) error {
	item := entity.AdminCarousel{
		Title:       req.Title,
		Description: req.Description,
		URL:         req.Url,
		HeaderImg:   req.HeaderImg,
		Display:     req.Display,
		Target:      req.Target,
		Click:       uint(req.Click),
	}
	return conn.MysqlConn.Create(&item).Error
}

func UpdateAdminCarousel(req *pb.AdminTableRow) error {
	return conn.MysqlConn.Model(&entity.AdminCarousel{}).Where("id = ?", req.Id).Updates(map[string]interface{}{
		"title":       req.Title,
		"description": req.Description,
		"url":         req.Url,
		"header_img":  req.HeaderImg,
		"display":     req.Display,
		"target":      req.Target,
		"click":       uint(req.Click),
		"updated_at":  time.Now(),
	}).Error
}

func ChangeAdminCarouselBool(id int, field string, value bool) error {
	return conn.MysqlConn.Model(&entity.AdminCarousel{}).Where("id = ?", id).Updates(map[string]interface{}{
		field:        value,
		"updated_at": time.Now(),
	}).Error
}

func DeleteAdminCarousels(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminCarousel{}, ids).Error
}

func carouselToTableRow(item entity.AdminCarousel) *pb.AdminTableRow {
	return &pb.AdminTableRow{
		Id:          uint32(item.ID),
		Title:       item.Title,
		Description: item.Description,
		Url:         item.URL,
		HeaderImg:   item.HeaderImg,
		Display:     item.Display,
		Target:      item.Target,
		Click:       uint32(item.Click),
		CreateTime:  formatAdminTime(item.CreatedAt),
	}
}

func AdminBlacklistList(pageSize, pageNum int, ip, description, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.AdminBlacklist
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminBlacklist{})
	if strings.TrimSpace(ip) != "" {
		query = query.Where("ip LIKE ?", "%"+strings.TrimSpace(ip)+"%")
	}
	if strings.TrimSpace(description) != "" {
		query = query.Where("description LIKE ?", "%"+strings.TrimSpace(description)+"%")
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, blacklistToTableRow(item))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func MatchAdminBlacklist(ip, url string) (bool, error) {
	ip = strings.TrimSpace(ip)
	if ip == "" {
		return false, nil
	}
	var items []entity.AdminBlacklist
	if err := conn.MysqlConn.Find(&items).Error; err != nil {
		return false, err
	}
	for _, item := range items {
		if blacklistIPMatches(strings.TrimSpace(item.IP), ip) {
			return true, conn.MysqlConn.Model(&entity.AdminBlacklist{}).Where("id = ?", item.ID).Updates(map[string]interface{}{
				"last_access_url":  truncateValue(url, 255),
				"last_access_time": time.Now(),
				"intercept_count":  item.InterceptCount + 1,
				"updated_at":       time.Now(),
			}).Error
		}
	}
	return false, nil
}

func blacklistIPMatches(rule, ip string) bool {
	if rule == "" || ip == "" {
		return false
	}
	if strings.Contains(rule, "/") {
		_, network, err := net.ParseCIDR(rule)
		return err == nil && network.Contains(net.ParseIP(ip))
	}
	if strings.HasSuffix(rule, "*") {
		return strings.HasPrefix(ip, strings.TrimSuffix(rule, "*"))
	}
	return rule == ip
}

func truncateValue(value string, max int) string {
	value = strings.TrimSpace(value)
	if max <= 0 {
		return value
	}
	runes := []rune(value)
	if len(runes) <= max {
		return value
	}
	return string(runes[:max])
}

func AdminBlacklistOne(id int) (*pb.AdminTableOneResp, error) {
	item := entity.AdminBlacklist{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: blacklistToTableRow(item)}, nil
}

func SaveAdminBlacklist(req *pb.AdminTableRow) error {
	item := entity.AdminBlacklist{
		IP:          req.Ip,
		Description: req.Description,
	}
	return conn.MysqlConn.Create(&item).Error
}

func UpdateAdminBlacklist(req *pb.AdminTableRow) error {
	return conn.MysqlConn.Model(&entity.AdminBlacklist{}).Where("id = ?", req.Id).Updates(map[string]interface{}{
		"ip":          req.Ip,
		"description": req.Description,
		"updated_at":  time.Now(),
	}).Error
}

func DeleteAdminBlacklists(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminBlacklist{}, ids).Error
}

func blacklistToTableRow(item entity.AdminBlacklist) *pb.AdminTableRow {
	return &pb.AdminTableRow{
		Id:             uint32(item.ID),
		Ip:             item.IP,
		Description:    item.Description,
		LastAccessUrl:  item.LastAccessURL,
		LastAccessTime: formatAdminTime(item.LastAccessTime),
		InterceptCount: uint32(item.InterceptCount),
		CreateTime:     formatAdminTime(item.CreatedAt),
	}
}

func AdminQuartzJobList(pageSize, pageNum int, jobName, methodName, status, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.AdminQuartzJob
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminQuartzJob{})
	if strings.TrimSpace(jobName) != "" {
		query = query.Where("job_name LIKE ?", "%"+strings.TrimSpace(jobName)+"%")
	}
	if strings.TrimSpace(methodName) != "" {
		query = query.Where("method_name LIKE ?", "%"+strings.TrimSpace(methodName)+"%")
	}
	if strings.TrimSpace(status) != "" {
		if status == "true" || status == "1" || status == "0" {
			query = query.Where("status = ?", status == "true" || status == "1")
		}
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, quartzJobToTableRow(item))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminQuartzJobOne(id int) (*pb.AdminTableOneResp, error) {
	item := entity.AdminQuartzJob{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: quartzJobToTableRow(item)}, nil
}

func SaveAdminQuartzJob(req *pb.AdminTableRow) error {
	item := entity.AdminQuartzJob{
		JobName:        req.JobName,
		BeanName:       req.BeanName,
		MethodName:     req.MethodName,
		MethodParams:   req.MethodParams,
		CronExpression: req.CronExpression,
		Status:         req.Enabled,
		Remark:         req.Remark,
	}
	return conn.MysqlConn.Create(&item).Error
}

func UpdateAdminQuartzJob(req *pb.AdminTableRow) error {
	return conn.MysqlConn.Model(&entity.AdminQuartzJob{}).Where("id = ?", req.Id).Updates(map[string]interface{}{
		"job_name":        req.JobName,
		"bean_name":       req.BeanName,
		"method_name":     req.MethodName,
		"method_params":   req.MethodParams,
		"cron_expression": req.CronExpression,
		"status":          req.Enabled,
		"remark":          req.Remark,
		"updated_at":      time.Now(),
	}).Error
}

func ToggleAdminQuartzJobStatus(id int) error {
	item := entity.AdminQuartzJob{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return err
	}
	return conn.MysqlConn.Model(&entity.AdminQuartzJob{}).Where("id = ?", id).Updates(map[string]interface{}{
		"status":     !item.Status,
		"updated_at": time.Now(),
	}).Error
}

func ExecuteAdminQuartzJob(id int) error {
	item := entity.AdminQuartzJob{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return err
	}
	log := entity.AdminQuartzLog{
		JobName:        item.JobName,
		BeanName:       item.BeanName,
		MethodName:     item.MethodName,
		MethodParams:   item.MethodParams,
		CronExpression: item.CronExpression,
		Status:         true,
		Cost:           0,
		Result:         "手动触发成功",
	}
	return conn.MysqlConn.Create(&log).Error
}

func DeleteAdminQuartzJobs(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminQuartzJob{}, ids).Error
}

func quartzJobToTableRow(item entity.AdminQuartzJob) *pb.AdminTableRow {
	return &pb.AdminTableRow{
		Id:             uint32(item.ID),
		JobName:        item.JobName,
		BeanName:       item.BeanName,
		MethodName:     item.MethodName,
		MethodParams:   item.MethodParams,
		CronExpression: item.CronExpression,
		Enabled:        item.Status,
		Status:         strconv.FormatBool(item.Status),
		Remark:         item.Remark,
		CreateTime:     formatAdminTime(item.CreatedAt),
	}
}

func AdminQuartzLogList(pageSize, pageNum int, jobName, methodName, status, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.AdminQuartzLog
	var total int64
	query := conn.MysqlConn.Model(&entity.AdminQuartzLog{})
	if strings.TrimSpace(jobName) != "" {
		query = query.Where("job_name LIKE ?", "%"+strings.TrimSpace(jobName)+"%")
	}
	if strings.TrimSpace(methodName) != "" {
		query = query.Where("method_name LIKE ?", "%"+strings.TrimSpace(methodName)+"%")
	}
	if strings.TrimSpace(status) != "" {
		query = query.Where("status = ?", status == "true" || status == "1")
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, quartzLogToTableRow(item))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminQuartzLogOne(id int) (*pb.AdminTableOneResp, error) {
	item := entity.AdminQuartzLog{}
	if err := conn.MysqlConn.First(&item, id).Error; err != nil {
		return nil, err
	}
	return &pb.AdminTableOneResp{Data: quartzLogToTableRow(item)}, nil
}

func DeleteAdminQuartzLogs(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.AdminQuartzLog{}, ids).Error
}

func CleanAdminQuartzLogs() error {
	return conn.MysqlConn.Where("1 = 1").Delete(&entity.AdminQuartzLog{}).Error
}

func quartzLogToTableRow(item entity.AdminQuartzLog) *pb.AdminTableRow {
	return &pb.AdminTableRow{
		Id:             uint32(item.ID),
		JobName:        item.JobName,
		BeanName:       item.BeanName,
		MethodName:     item.MethodName,
		MethodParams:   item.MethodParams,
		CronExpression: item.CronExpression,
		Enabled:        item.Status,
		Success:        item.Status,
		Status:         strconv.FormatBool(item.Status),
		Cost:           uint32(item.Cost),
		Result:         item.Result,
		Exception:      item.Exception,
		CreateTime:     formatAdminTime(item.CreatedAt),
	}
}

func AdminStorageList(pageSize, pageNum int, name, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.Resource
	var total int64
	query := conn.MysqlConn.Model(&entity.Resource{})
	if strings.TrimSpace(name) != "" {
		query = query.Where("`key` LIKE ?", "%"+strings.TrimSpace(name)+"%")
	}
	query = applyCreatedAtRange(query, beginTime, endTime)
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, resourceToTableRow(item))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func AdminOnlineList(pageSize, pageNum int, ipaddr, userName string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	logs := latestRequestByUser(500)
	rows := []*pb.AdminTableRow{}
	cursor := uint64(0)
	index := uint32(1)
	for {
		keys, next, err := cache.Client.Scan(cursor, "*", 100).Result()
		if err != nil {
			return nil, err
		}
		cursor = next
		for _, key := range keys {
			raw, err := cache.Client.Get(key).Result()
			if err != nil || strings.TrimSpace(raw) == "" {
				continue
			}
			user := entity.User{}
			if err := json.Unmarshal([]byte(raw), &user); err != nil || strings.TrimSpace(user.UserName) == "" {
				continue
			}
			if strings.TrimSpace(userName) != "" && !strings.Contains(user.UserName, strings.TrimSpace(userName)) {
				continue
			}
			row := &pb.AdminTableRow{
				Id:         index,
				Title:      key,
				UserName:   user.UserName,
				NickName:   user.UserName,
				Avatar:     user.Avatar,
				CreateTime: formatAdminTime(user.LastLogin),
				Success:    true,
				Status:     "true",
			}
			if last, ok := logs[user.UserName]; ok {
				row.Ip = last.IP
				row.Location = last.RemoteAddr
				row.Browser = strings.TrimSpace(last.UserAgent.Client["name"] + " " + last.UserAgent.Client["version"])
				row.Os = strings.TrimSpace(last.UserAgent.OS.Name + " " + last.UserAgent.OS.Version)
				if row.CreateTime == "" {
					row.CreateTime = formatAdminTime(last.CreatedAt)
				}
			}
			if strings.TrimSpace(ipaddr) != "" && !strings.Contains(row.Ip, strings.TrimSpace(ipaddr)) {
				continue
			}
			rows = append(rows, row)
			index++
		}
		if cursor == 0 {
			break
		}
	}
	sort.Slice(rows, func(i, j int) bool {
		return rows[i].CreateTime > rows[j].CreateTime
	})
	total := len(rows)
	start := adminOffset(pageSize, pageNum)
	if start >= total {
		return &pb.AdminTableListResp{Total: uint32(total), Rows: []*pb.AdminTableRow{}}, nil
	}
	end := start + pageSize
	if end > total {
		end = total
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows[start:end]}, nil
}

func AdminOnlineDelete(token string) error {
	return cache.Client.Del(token).Err()
}

func latestRequestByUser(limit int) map[string]entity.Request {
	res := map[string]entity.Request{}
	if limit <= 0 {
		limit = 500
	}
	var items []entity.Request
	if err := conn.MysqlConn.Model(&entity.Request{}).
		Where("user_name <> ?", "").
		Order("created_at DESC").
		Limit(limit).
		Find(&items).Error; err != nil {
		return res
	}
	for _, item := range items {
		if _, ok := res[item.UserName]; !ok {
			res[item.UserName] = item
		}
	}
	return res
}

func DeleteAdminResources(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.Resource{}, ids).Error
}

func resourceToTableRow(item entity.Resource) *pb.AdminTableRow {
	key := item.Key
	name := path.Base(key)
	url := config.Cfg.Qiniu.Host + key
	suffix := ""
	if dot := strings.LastIndex(name, "."); dot >= 0 && dot < len(name)-1 {
		suffix = name[dot+1:]
	}
	return &pb.AdminTableRow{
		Id:         uint32(item.ID),
		Name:       name,
		Url:        url,
		Path:       url,
		Suffix:     suffix,
		Bucket:     config.Cfg.Qiniu.Bucket,
		Size:       "N/A",
		Type:       strconv.Itoa(item.Type),
		CreateTime: formatAdminTime(item.CreatedAt),
	}
}

func SaveAdminRequestLog(req *entity.Request) error {
	return conn.MysqlConn.Create(req).Error
}

func AdminRequestLogList(kind string, pageSize, pageNum int, filters map[string]string) (*pb.AdminTableListResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	var items []entity.Request
	var total int64
	query := conn.MysqlConn.Model(&entity.Request{})
	if condition, args := requestLogKindCondition(kind); condition != "" {
		query = query.Where(condition, args...)
	}
	if v := strings.TrimSpace(filters["ip"]); v != "" {
		query = query.Where("ip LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(filters["location"]); v != "" {
		query = query.Where("remote_addr LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(filters["userName"]); v != "" {
		query = query.Where("user_name LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(filters["title"]); v != "" {
		query = query.Where("url LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(filters["businessType"]); v != "" {
		query = query.Where("op_type = ?", v)
	}
	if v := strings.TrimSpace(filters["status"]); v != "" {
		if v == "0" || v == "true" {
			query = query.Where("status_code >= 200 AND status_code < 400")
		} else if v == "1" || v == "false" {
			query = query.Where("status_code >= 400")
		}
	}
	query = applyCreatedAtRange(query, filters["beginTime"], filters["endTime"])
	if err := query.Count(&total).Error; err != nil {
		return nil, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(adminOffset(pageSize, pageNum)).Find(&items).Error; err != nil {
		return nil, err
	}
	rows := make([]*pb.AdminTableRow, 0, len(items))
	for _, item := range items {
		rows = append(rows, requestToTableRow(item, kind))
	}
	return &pb.AdminTableListResp{Total: uint32(total), Rows: rows}, nil
}

func DeleteAdminRequestLogs(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&entity.Request{}, ids).Error
}

func CleanAdminRequestLogs(kind string) error {
	query := conn.MysqlConn.Where("1 = 1")
	if condition, args := requestLogKindCondition(kind); condition != "" {
		query = query.Where(condition, args...)
	}
	return query.Delete(&entity.Request{}).Error
}

func requestLogKindCondition(kind string) (string, []interface{}) {
	switch kind {
	case "loginLog":
		return "(is_login = ? OR url LIKE ? OR url LIKE ?)", []interface{}{true, "%/login%", "%/logout%"}
	case "operateLog":
		return "url LIKE ? AND is_login = ? AND method <> ? AND url NOT LIKE ? AND url NOT LIKE ? AND url NOT LIKE ? AND url NOT LIKE ? AND url NOT LIKE ? AND url NOT LIKE ?", []interface{}{
			"/admin/%", false, "GET", "%/dashboard/%", "%/routers%", "%/info%", "%/login%", "%/logout%", "%/oauth%",
		}
	case "visitLog":
		return "url NOT LIKE ? AND is_login = ? AND url NOT LIKE ? AND url NOT LIKE ? AND url NOT LIKE ?", []interface{}{
			"/admin/%", false, "%/login%", "%/logout%", "%/oauth%",
		}
	default:
		return "", nil
	}
}

func requestToTableRow(item entity.Request, kind string) *pb.AdminTableRow {
	browser := strings.TrimSpace(item.UserAgent.Client["name"] + " " + item.UserAgent.Client["version"])
	os := strings.TrimSpace(item.UserAgent.OS.Name + " " + item.UserAgent.OS.Version)
	success := item.StatusCode >= 200 && item.StatusCode < 400
	title := requestLogTitle(item, kind)
	return &pb.AdminTableRow{
		Id:            uint32(item.ID),
		Title:         title,
		Url:           item.URL,
		Path:          item.URL,
		Method:        item.Method,
		RequestMethod: item.Method,
		BusinessType:  item.OpType,
		UserName:      item.UserName,
		OperateName:   item.UserName,
		Ip:            item.IP,
		Location:      item.RemoteAddr,
		Browser:       browser,
		Os:            os,
		Success:       success,
		Status:        strconv.FormatBool(success),
		Msg:           statusMessage(success),
		ErrorMsg:      item.ErrorMsg,
		Cost:          uint32(item.RequestTime),
		CreateTime:    formatAdminTime(item.CreatedAt),
	}
}

func requestLogTitle(item entity.Request, kind string) string {
	url := strings.TrimSpace(item.URL)
	lowerURL := strings.ToLower(url)
	switch kind {
	case "loginLog":
		if strings.Contains(lowerURL, "logout") {
			return "退出登录"
		}
		if strings.Contains(lowerURL, "github") || strings.Contains(lowerURL, "oauth") {
			return "GitHub 登录"
		}
		return "后台登录"
	case "operateLog":
		return operationLogTitle(item.Method, url)
	case "visitLog":
		if url == "" || url == "/" {
			return "访问首页"
		}
		return "访问 " + url
	default:
		return strings.TrimPrefix(url, "/")
	}
}

func operationLogTitle(method, url string) string {
	url = strings.TrimSpace(url)
	if url == "" {
		return "后台操作"
	}
	action := map[string]string{
		"POST":   "新增/提交",
		"PUT":    "修改",
		"PATCH":  "修改",
		"DELETE": "删除",
	}[strings.ToUpper(method)]
	if action == "" {
		action = "后台操作"
	}
	return action + " " + url
}

func statusMessage(success bool) string {
	if success {
		return "成功"
	}
	return "失败"
}

func AdminDashboardAccessData(limit int) (*pb.DashboardAccessResp, error) {
	items, err := recentRequestLogs(limit)
	if err != nil {
		return nil, err
	}
	browserCounts := map[string]uint32{}
	osCounts := map[string]uint32{}
	legendSet := map[string]bool{}
	for _, item := range items {
		browser := firstNonEmpty(item.UserAgent.Client["name"], "未知浏览器")
		os := firstNonEmpty(item.UserAgent.OS.Name, "未知系统")
		browserCounts[browser]++
		osCounts[os]++
		legendSet[browser] = true
		legendSet[os] = true
	}
	legend := make([]string, 0, len(legendSet))
	for item := range legendSet {
		legend = append(legend, item)
	}
	sort.Strings(legend)
	return &pb.DashboardAccessResp{
		Legend: legend,
		Inner:  chartItemsFromCounts(browserCounts),
		Out:    chartItemsFromCounts(osCounts),
	}, nil
}

func AdminDashboardSpiderData(limit int) (*pb.DashboardSpiderResp, error) {
	items, err := recentRequestLogs(limit)
	if err != nil {
		return nil, err
	}
	counts := map[string]uint32{
		"普通访问": 0,
		"爬虫访问": 0,
		"其他请求": 0,
	}
	for _, item := range items {
		if item.UserAgent.Robot.Name != "" || strings.Contains(strings.ToLower(item.UserAgent.Client["name"]), "bot") {
			counts["爬虫访问"]++
			continue
		}
		if item.UserAgent.Client["name"] != "" {
			counts["普通访问"]++
			continue
		}
		counts["其他请求"]++
	}
	return &pb.DashboardSpiderResp{Data: chartItemsFromCounts(counts)}, nil
}

func AdminDashboardLogRows(kind string, pageSize, pageNum int) (*pb.LogResp, error) {
	pageSize, pageNum = adminPage(pageSize, pageNum)
	resp := &pb.LogResp{}
	switch kind {
	case "taskLog":
		result, err := AdminQuartzLogList(pageSize, pageNum, "", "", "", "", "")
		if err != nil {
			return nil, err
		}
		resp.Total = result.Total
		for _, row := range result.Rows {
			resp.Rows = append(resp.Rows, dashboardLogLine(row))
		}
	default:
		if kind == "" {
			kind = "visitLog"
		}
		result, err := AdminRequestLogList(kind, pageSize, pageNum, map[string]string{})
		if err != nil {
			return nil, err
		}
		resp.Total = result.Total
		for _, row := range result.Rows {
			resp.Rows = append(resp.Rows, dashboardLogLine(row))
		}
	}
	return resp, nil
}

func recentRequestLogs(limit int) ([]entity.Request, error) {
	if limit <= 0 {
		limit = 500
	}
	if limit > 2000 {
		limit = 2000
	}
	var items []entity.Request
	err := conn.MysqlConn.Model(&entity.Request{}).
		Where("is_delete = ?", false).
		Order("created_at DESC").
		Limit(limit).
		Find(&items).Error
	return items, err
}

func chartItemsFromCounts(counts map[string]uint32) []*pb.DashboardChartItem {
	type pair struct {
		name  string
		value uint32
	}
	pairs := make([]pair, 0, len(counts))
	for name, value := range counts {
		if strings.TrimSpace(name) == "" {
			name = "未知"
		}
		pairs = append(pairs, pair{name: name, value: value})
	}
	sort.Slice(pairs, func(i, j int) bool {
		if pairs[i].value == pairs[j].value {
			return pairs[i].name < pairs[j].name
		}
		return pairs[i].value > pairs[j].value
	})
	items := make([]*pb.DashboardChartItem, 0, len(pairs))
	for _, item := range pairs {
		items = append(items, &pb.DashboardChartItem{Name: item.name, Value: item.value})
	}
	return items
}

func dashboardLogLine(row *pb.AdminTableRow) string {
	if row == nil {
		return ""
	}
	prefix := strings.TrimSpace(row.CreateTime)
	if prefix != "" {
		prefix += " "
	}
	method := firstNonEmpty(row.RequestMethod, row.Method, row.BusinessType)
	if method != "" {
		method = "[" + method + "] "
	}
	title := firstNonEmpty(row.Title, row.Url, row.JobName, row.MethodName)
	status := firstNonEmpty(row.Msg, row.Result, row.Status)
	user := firstNonEmpty(row.UserName, row.OperateName)
	if user != "" {
		user = " - " + user
	}
	if status != "" {
		status = " - " + status
	}
	return prefix + method + title + user + status
}

func AdminTableRowFromJSON(raw string) (*pb.AdminTableRow, error) {
	row := &pb.AdminTableRow{}
	err := json.Unmarshal([]byte(raw), row)
	return row, err
}
