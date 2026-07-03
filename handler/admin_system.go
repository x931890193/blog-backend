package handler

import (
	"blog-backend/cache"
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net"
	"net/http"
	"runtime"
	"strconv"
	"strings"
	"syscall"
	"time"
)

var adminServerStartedAt = time.Now()

type AdminUserListRequest struct {
	PageSize    int    `form:"pageSize"`
	PageNum     int    `form:"pageNum"`
	CurrentPage int    `form:"currentPage"`
	UserName    string `form:"userName"`
	Phone       string `form:"phone"`
	Status      string `form:"status"`
}

type AdminTableListRequest struct {
	PageSize     int    `form:"pageSize"`
	PageNum      int    `form:"pageNum"`
	CurrentPage  int    `form:"currentPage"`
	Title        string `form:"title"`
	CreateBy     string `form:"createBy"`
	Type         string `form:"type"`
	Status       string `form:"status"`
	Description  string `form:"description"`
	IP           string `form:"ip"`
	Location     string `form:"location"`
	UserName     string `form:"userName"`
	OperName     string `form:"operName"`
	RoleName     string `form:"roleName"`
	RoleKey      string `form:"roleKey"`
	JobName      string `form:"jobName"`
	MethodName   string `form:"methodName"`
	Name         string `form:"name"`
	BusinessType string `form:"businessType"`
}

func adminCurrentPage(req AdminTableListRequest) int {
	if req.CurrentPage != 0 {
		return req.CurrentPage
	}
	return req.PageNum
}

func bindAdminTableRow(c *gin.Context, resp *pb.BaseResp) (*pb.AdminTableRow, bool) {
	req := pb.AdminTableRow{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return nil, false
	}
	return &req, true
}

func writeBaseError(c *gin.Context, code ErrorCode, err error) {
	resp := pb.BaseResp{Code: uint32(code)}
	if err != nil {
		resp.Msg = ConvertMsg(code, err.Error())
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserList(c *gin.Context) {
	req := AdminUserListRequest{}
	resp := pb.AdminUserListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	currentPage := req.CurrentPage
	if currentPage == 0 {
		currentPage = req.PageNum
	}
	result, err := service.AdminUserList(
		req.PageSize,
		currentPage,
		req.UserName,
		req.Phone,
		req.Status,
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminUserOne(c *gin.Context) {
	resp := pb.AdminUserOneResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminUserOne(id)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminUserProfile(c *gin.Context) {
	resp := pb.AdminUserOneResp{}
	token, exists := c.Get("admin")
	if !exists {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, "")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminUserProfile(token.(string))
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminUserAdd(c *gin.Context) {
	req := pb.AdminUserRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if req.UserName == "" && req.NickName == "" {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "用户名称不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminCreateUser(&req); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserProfileUpdate(c *gin.Context) {
	req := pb.AdminUserRequest{}
	resp := pb.BaseResp{}
	token, exists := c.Get("admin")
	if !exists {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, "")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminUpdateUserProfile(token.(string), &req); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserProfileUpdatePassword(c *gin.Context) {
	resp := pb.BaseResp{}
	token, exists := c.Get("admin")
	if !exists {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, "")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminUpdateUserPassword(token.(string), c.Query("oldPassword"), c.Query("newPassword")); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserEdit(c *gin.Context) {
	req := pb.AdminUserRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "用户ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminUpdateUser(&req); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserDelete(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminDeleteUsers(ids); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserResetPassword(c *gin.Context) {
	req := pb.AdminPasswordRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "用户ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminResetUserPassword(int(req.Id), req.Password); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminUserChangeStatus(c *gin.Context) {
	req := pb.AdminUserRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "用户ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminChangeUserStatus(int(req.Id), req.Status); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminRoleList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminRoleListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminRoleList(
		req.PageSize,
		adminCurrentPage(req),
		req.RoleName,
		req.RoleKey,
		req.Status,
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminRoleOne(c *gin.Context) {
	resp := pb.AdminTableOneResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminRoleOne(id)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminRoleAdd(c *gin.Context) {
	resp := pb.BaseResp{}
	req, ok := bindAdminTableRow(c, &resp)
	if !ok {
		return
	}
	if err := service.SaveAdminRole(req); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminRoleUpdate(c *gin.Context) {
	resp := pb.BaseResp{}
	req, ok := bindAdminTableRow(c, &resp)
	if !ok {
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "角色ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.UpdateAdminRole(req); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminRoleChangeStatus(c *gin.Context) {
	resp := pb.BaseResp{}
	req, ok := bindAdminTableRow(c, &resp)
	if !ok {
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "角色ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.ChangeAdminRoleStatus(int(req.Id), req.Status); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminRoleDelete(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.DeleteAdminRoles(ids); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminDictData(c *gin.Context) {
	dictType := c.Param("dictType")
	dicts := map[string][]gin.H{
		"sys_normal_disable": {
			{"dictLabel": "正常", "dictValue": "0"},
			{"dictLabel": "停用", "dictValue": "1"},
		},
		"sys_common_status": {
			{"dictLabel": "成功", "dictValue": "0"},
			{"dictLabel": "失败", "dictValue": "1"},
		},
		"sys_user_sex": {
			{"dictLabel": "男", "dictValue": "0"},
			{"dictLabel": "女", "dictValue": "1"},
			{"dictLabel": "未知", "dictValue": "2"},
		},
		"sys_notice_status": {
			{"dictLabel": "正常", "dictValue": "0"},
			{"dictLabel": "关闭", "dictValue": "1"},
		},
		"sys_notice_type": {
			{"dictLabel": "通知", "dictValue": "1"},
			{"dictLabel": "公告", "dictValue": "2"},
		},
		"sys_oper_type": {
			{"dictLabel": "其它", "dictValue": "0"},
			{"dictLabel": "新增", "dictValue": "1"},
			{"dictLabel": "修改", "dictValue": "2"},
			{"dictLabel": "删除", "dictValue": "3"},
		},
		"bg_blog_status": {
			{"dictLabel": "发布", "dictValue": true},
			{"dictLabel": "草稿", "dictValue": false},
		},
		"bg_blog_support": {
			{"dictLabel": "推荐", "dictValue": true},
			{"dictLabel": "普通", "dictValue": false},
		},
	}
	data := dicts[dictType]
	if data == nil {
		data = []gin.H{}
	}
	c.JSON(http.StatusOK, gin.H{
		"code": 0,
		"msg":  "",
		"data": data,
	})
}

func AdminEmptyTableList(c *gin.Context) {
	c.ProtoBuf(http.StatusOK, &pb.AdminTableListResp{})
}

func AdminBaseOK(c *gin.Context) {
	c.ProtoBuf(http.StatusOK, &pb.BaseResp{})
}

func AdminOnlineList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	ipaddr := strings.TrimSpace(req.IP)
	if ipaddr == "" {
		ipaddr = strings.TrimSpace(c.Query("ipaddr"))
	}
	result, err := service.AdminOnlineList(req.PageSize, adminCurrentPage(req), ipaddr, req.UserName)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminOnlineDelete(c *gin.Context) {
	resp := pb.BaseResp{}
	tokenID := strings.TrimSpace(c.Param("tokenId"))
	if tokenID == "" {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "tokenId不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AdminOnlineDelete(tokenID); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminRedisInfo(c *gin.Context) {
	info, err := cache.Client.Info().Result()
	if err != nil {
		c.JSON(http.StatusOK, gin.H{"code": uint32(DbError), "msg": ConvertMsg(DbError, err.Error()), "data": []gin.H{}})
		return
	}
	rows := []gin.H{}
	for _, line := range strings.Split(info, "\n") {
		line = strings.TrimSpace(line)
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		parts := strings.SplitN(line, ":", 2)
		if len(parts) != 2 {
			continue
		}
		rows = append(rows, gin.H{
			"key":         parts[0],
			"description": redisInfoDescription(parts[0]),
			"value":       parts[1],
		})
		if len(rows) >= 80 {
			break
		}
	}
	c.JSON(http.StatusOK, gin.H{"code": 0, "msg": "", "data": rows})
}

func AdminServerInfo(c *gin.Context) {
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	var disk syscall.Statfs_t
	_ = syscall.Statfs("/", &disk)
	totalDisk := disk.Blocks * uint64(disk.Bsize)
	freeDisk := disk.Bavail * uint64(disk.Bsize)
	usedDisk := totalDisk - freeDisk
	memoryUsed := mem.Sys
	memoryTotal := memoryUsed + mem.HeapIdle + mem.HeapReleased
	if memoryTotal == 0 {
		memoryTotal = memoryUsed
	}
	c.JSON(http.StatusOK, gin.H{
		"code": 0,
		"msg":  "",
		"data": gin.H{
			"time": time.Now().Format("15:04:05"),
			"sys": gin.H{
				"os":  runtime.GOOS + "/" + runtime.GOARCH,
				"ip":  firstLocalIP(),
				"day": humanDuration(time.Since(adminServerStartedAt)),
			},
			"cpu": gin.H{
				"name":       runtime.GOOS,
				"package":    "Go runtime",
				"core":       strconv.Itoa(runtime.NumCPU()) + " CPU",
				"logic":      strconv.Itoa(runtime.NumGoroutine()) + " goroutines",
				"used":       "0",
				"coreNumber": strconv.Itoa(runtime.NumCPU()),
			},
			"memory": gin.H{
				"total":     formatBytes(memoryTotal),
				"used":      formatBytes(memoryUsed),
				"available": formatBytes(memoryTotal - memoryUsed),
				"usageRate": percent(memoryUsed, memoryTotal),
			},
			"swap": gin.H{
				"total":     "N/A",
				"used":      "0",
				"available": "N/A",
				"usageRate": "0",
			},
			"disk": gin.H{
				"total":     formatBytes(totalDisk),
				"used":      formatBytes(usedDisk),
				"available": formatBytes(freeDisk),
				"usageRate": percent(usedDisk, totalDisk),
			},
		},
	})
}

func redisInfoDescription(key string) string {
	descriptions := map[string]string{
		"redis_version":             "Redis 版本",
		"connected_clients":         "客户端连接数",
		"used_memory_human":         "已用内存",
		"total_commands_processed":  "已处理命令数",
		"instantaneous_ops_per_sec": "每秒操作数",
		"keyspace_hits":             "键命中次数",
		"keyspace_misses":           "键未命中次数",
		"expired_keys":              "过期键数量",
		"evicted_keys":              "淘汰键数量",
		"role":                      "实例角色",
		"uptime_in_days":            "运行天数",
		"blocked_clients":           "阻塞客户端",
		"connected_slaves":          "从库数量",
		"used_memory_peak_human":    "内存峰值",
		"mem_fragmentation_ratio":   "内存碎片率",
		"rdb_last_bgsave_status":    "RDB 状态",
		"aof_last_bgrewrite_status": "AOF 重写状态",
		"latest_fork_usec":          "最近 fork 耗时",
	}
	if v, ok := descriptions[key]; ok {
		return v
	}
	return "Redis INFO"
}

func firstLocalIP() string {
	ifaces, err := net.Interfaces()
	if err != nil {
		return ""
	}
	fallback := ""
	for _, iface := range ifaces {
		if iface.Flags&net.FlagUp == 0 || iface.Flags&net.FlagLoopback != 0 {
			continue
		}
		addrs, err := iface.Addrs()
		if err != nil {
			continue
		}
		isVirtual := virtualInterfaceName(iface.Name)
		for _, addr := range addrs {
			ipNet, ok := addr.(*net.IPNet)
			if !ok {
				continue
			}
			ip := ipNet.IP.To4()
			if ip == nil || ip.IsLoopback() || ip.IsUnspecified() {
				continue
			}
			if fallback == "" {
				fallback = ip.String()
			}
			if !isVirtual && !ip.IsLinkLocalUnicast() {
				return ip.String()
			}
		}
	}
	return fallback
}

func virtualInterfaceName(name string) bool {
	name = strings.ToLower(name)
	prefixes := []string{"lo", "docker", "br-", "veth", "virbr", "tun", "tap"}
	for _, prefix := range prefixes {
		if strings.HasPrefix(name, prefix) {
			return true
		}
	}
	return false
}

func humanDuration(d time.Duration) string {
	days := int(d.Hours()) / 24
	hours := int(d.Hours()) % 24
	minutes := int(d.Minutes()) % 60
	return strconv.Itoa(days) + "天 " + strconv.Itoa(hours) + "小时 " + strconv.Itoa(minutes) + "分钟"
}

func formatBytes(value uint64) string {
	const unit = 1024
	if value < unit {
		return strconv.FormatUint(value, 10) + " B"
	}
	div, exp := uint64(unit), 0
	for n := value / unit; n >= unit; n /= unit {
		div *= unit
		exp++
	}
	return strconv.FormatFloat(float64(value)/float64(div), 'f', 1, 64) + " " + string("KMGTPE"[exp]) + "B"
}

func percent(used, total uint64) string {
	if total == 0 {
		return "0"
	}
	return strconv.FormatFloat(float64(used)*100/float64(total), 'f', 2, 64)
}

func AdminNoticeList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminNoticeList(req.PageSize, adminCurrentPage(req), req.Title, req.CreateBy, req.Type, req.Status, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminNoticeOne(c *gin.Context) {
	adminTableOneByID(c, service.AdminNoticeOne)
}

func AdminNoticeAdd(c *gin.Context) {
	adminTableSave(c, service.SaveAdminNotice)
}

func AdminNoticeUpdate(c *gin.Context) {
	adminTableUpdate(c, service.UpdateAdminNotice)
}

func AdminNoticeDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminNotices)
}

func AdminCarouselList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminCarouselList(req.PageSize, adminCurrentPage(req), req.Title, req.Description, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminCarouselOne(c *gin.Context) {
	adminTableOneByID(c, service.AdminCarouselOne)
}

func AdminCarouselAdd(c *gin.Context) {
	adminTableSave(c, service.SaveAdminCarousel)
}

func AdminCarouselUpdate(c *gin.Context) {
	adminTableUpdate(c, service.UpdateAdminCarousel)
}

func AdminCarouselDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminCarousels)
}

func AdminCarouselDisplay(c *gin.Context) {
	adminBoolParam(c, "display", "display", service.ChangeAdminCarouselBool)
}

func AdminCarouselTarget(c *gin.Context) {
	adminBoolParam(c, "target", "target", service.ChangeAdminCarouselBool)
}

func AdminBlacklistList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminBlacklistList(req.PageSize, adminCurrentPage(req), req.IP, req.Description, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminBlacklistOne(c *gin.Context) {
	adminTableOneByID(c, service.AdminBlacklistOne)
}

func AdminBlacklistAdd(c *gin.Context) {
	adminTableSave(c, service.SaveAdminBlacklist)
}

func AdminBlacklistUpdate(c *gin.Context) {
	adminTableUpdate(c, service.UpdateAdminBlacklist)
}

func AdminBlacklistDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminBlacklists)
}

func AdminQuartzJobList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminQuartzJobList(req.PageSize, adminCurrentPage(req), req.JobName, req.MethodName, req.Status, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminQuartzJobOne(c *gin.Context) {
	adminTableOneByID(c, service.AdminQuartzJobOne)
}

func AdminQuartzJobAdd(c *gin.Context) {
	adminTableSave(c, service.SaveAdminQuartzJob)
}

func AdminQuartzJobUpdate(c *gin.Context) {
	adminTableUpdate(c, service.UpdateAdminQuartzJob)
}

func AdminQuartzJobDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminQuartzJobs)
}

func AdminQuartzJobExecute(c *gin.Context) {
	resp := pb.BaseResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.ExecuteAdminQuartzJob(id); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminQuartzJobStatus(c *gin.Context) {
	resp := pb.BaseResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.ToggleAdminQuartzJobStatus(id); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminQuartzLogList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminQuartzLogList(req.PageSize, adminCurrentPage(req), req.JobName, req.MethodName, req.Status, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminQuartzLogOne(c *gin.Context) {
	adminTableOneByID(c, service.AdminQuartzLogOne)
}

func AdminQuartzLogDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminQuartzLogs)
}

func AdminQuartzLogClean(c *gin.Context) {
	if err := service.CleanAdminQuartzLogs(); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &pb.BaseResp{})
}

func AdminStorageList(c *gin.Context) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminStorageList(req.PageSize, adminCurrentPage(req), req.Name, c.Query("params[beginTime]"), c.Query("params[endTime]"))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminStorageDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminResources)
}

func AdminRequestLoginLogList(c *gin.Context) {
	adminRequestLogList(c, "loginLog")
}

func AdminRequestOperateLogList(c *gin.Context) {
	adminRequestLogList(c, "operateLog")
}

func AdminRequestVisitLogList(c *gin.Context) {
	adminRequestLogList(c, "visitLog")
}

func AdminRequestLoginLogDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminRequestLogs)
}

func AdminRequestOperateLogDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminRequestLogs)
}

func AdminRequestVisitLogDelete(c *gin.Context) {
	adminTableDelete(c, service.DeleteAdminRequestLogs)
}

func AdminRequestLoginLogClean(c *gin.Context) {
	adminRequestLogClean(c, "loginLog")
}

func AdminRequestOperateLogClean(c *gin.Context) {
	adminRequestLogClean(c, "operateLog")
}

func AdminRequestVisitLogClean(c *gin.Context) {
	adminRequestLogClean(c, "visitLog")
}

func adminRequestLogList(c *gin.Context, kind string) {
	req := AdminTableListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminRequestLogList(kind, req.PageSize, adminCurrentPage(req), map[string]string{
		"ip":           req.IP,
		"location":     req.Location,
		"userName":     firstNonEmptyLocal(req.UserName, req.OperName),
		"status":       req.Status,
		"title":        req.Title,
		"businessType": req.BusinessType,
		"beginTime":    c.Query("params[beginTime]"),
		"endTime":      c.Query("params[endTime]"),
	})
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func firstNonEmptyLocal(values ...string) string {
	for _, value := range values {
		if strings.TrimSpace(value) != "" {
			return value
		}
	}
	return ""
}

func adminRequestLogClean(c *gin.Context, kind string) {
	if err := service.CleanAdminRequestLogs(kind); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &pb.BaseResp{})
}

func adminTableOneByID(c *gin.Context, fn func(int) (*pb.AdminTableOneResp, error)) {
	resp := pb.AdminTableOneResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := fn(id)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func adminTableSave(c *gin.Context, fn func(*pb.AdminTableRow) error) {
	resp := pb.BaseResp{}
	req, ok := bindAdminTableRow(c, &resp)
	if !ok {
		return
	}
	if err := fn(req); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func adminTableUpdate(c *gin.Context, fn func(*pb.AdminTableRow) error) {
	resp := pb.BaseResp{}
	req, ok := bindAdminTableRow(c, &resp)
	if !ok {
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "ID不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := fn(req); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func adminTableDelete(c *gin.Context, fn func([]int) error) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := fn(ids); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func adminBoolParam(c *gin.Context, fieldName, paramName string, fn func(int, string, bool) error) {
	resp := pb.BaseResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	value, err := strconv.ParseBool(c.Param(paramName))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := fn(id, fieldName, value); err != nil {
		writeBaseError(c, DbError, err)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
