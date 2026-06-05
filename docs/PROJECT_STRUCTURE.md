todo-api
├── migrations/
├── seeds/
│
├── src/
│   ├── config/
│   │   ├── app.rs
│   │   ├── database.rs
│   │   └── env.rs
│   │
│   ├── db/
│   │   └── connection.rs
│   │
│   ├── dto/
│   │   ├── auth/
│   │   ├── user/
│   │   ├── todo/
│   │   ├── category/
│   │   ├── priority/
│   │   ├── reminder/
│   │   └── notification/
│   │
│   ├── error/
│   │   ├── app_error.rs
│   │   └── mod.rs
│   │
│   ├── handlers/
│   │   ├── auth_handler.rs
│   │   ├── user_handler.rs
│   │   ├── todo_handler.rs
│   │   ├── category_handler.rs
│   │   ├── priority_handler.rs
│   │   ├── reminder_handler.rs
│   │   └── notification_handler.rs
│   │
│   ├── middleware/
│   │   ├── auth.rs
│   │   └── logger.rs
│   │
│   ├── models/
│   │   ├── role.rs
│   │   ├── user.rs
│   │   ├── refresh_token.rs
│   │   ├── todo_category.rs
│   │   ├── todo_priority.rs
│   │   ├── todo.rs
│   │   ├── todo_reminder.rs
│   │   ├── notification_log.rs
│   │   └── user_notification_setting.rs
│   │
│   ├── repositories/
│   │   ├── role_repository.rs
│   │   ├── user_repository.rs
│   │   ├── refresh_token_repository.rs
│   │   ├── category_repository.rs
│   │   ├── priority_repository.rs
│   │   ├── todo_repository.rs
│   │   ├── reminder_repository.rs
│   │   └── notification_repository.rs
│   │
│   ├── routes/
│   │   ├── auth_routes.rs
│   │   ├── user_routes.rs
│   │   ├── todo_routes.rs
│   │   ├── category_routes.rs
│   │   ├── priority_routes.rs
│   │   ├── reminder_routes.rs
│   │   ├── notification_routes.rs
│   │   └── mod.rs
│   │
│   ├── services/
│   │   ├── auth_service.rs
│   │   ├── user_service.rs
│   │   ├── todo_service.rs
│   │   ├── category_service.rs
│   │   ├── priority_service.rs
│   │   ├── reminder_service.rs
│   │   └── notification_service.rs
│   │
│   ├── utils/
│   │   ├── jwt.rs
│   │   ├── password.rs
│   │   ├── response.rs
│   │   └── date.rs
│   │
│   ├── app_state.rs
│   ├── lib.rs
│   └── main.rs
│
├── .env
├── .env.example
├── .gitignore
├── Cargo.toml
└── Cargo.lock

# Cấu Trúc Dự Án Todo API (Rust + Axum)

## Tổng Quan

Dự án được xây dựng theo kiến trúc nhiều lớp (Layered Architecture) kết hợp Repository Pattern và Service Pattern nhằm tách biệt trách nhiệm giữa các thành phần:

* **Handler Layer:** Nhận request từ client và trả response.
* **Service Layer:** Xử lý nghiệp vụ của hệ thống.
* **Repository Layer:** Tương tác trực tiếp với cơ sở dữ liệu.
* **Model Layer:** Đại diện cho các bảng dữ liệu trong database.
* **DTO Layer:** Định nghĩa dữ liệu trao đổi giữa client và server.

---

## migrations/

Chứa các file SQL Migration.

### Mục đích:

* Tạo bảng dữ liệu.
* Chỉnh sửa cấu trúc database.
* Thêm hoặc xoá cột.
* Tạo index.
* Tạo foreign key.
* Quản lý version của database.

---

## seeds/

Chứa dữ liệu khởi tạo ban đầu cho hệ thống.

### Mục đích:

* Tạo Role mặc định.
* Tạo Priority mặc định.
* Tạo tài khoản Admin mặc định.
* Sinh dữ liệu mẫu phục vụ phát triển và kiểm thử.

---

## src/

Thư mục chính chứa toàn bộ source code của ứng dụng.

---

## src/config/

Quản lý các cấu hình của hệ thống.

### src/config/app.rs

Chứa các cấu hình chung của ứng dụng:

* App Name.
* App Version.
* Host.
* Port.
* API Prefix.

### src/config/database.rs

Quản lý cấu hình kết nối cơ sở dữ liệu:

* Database URL.
* Connection Pool.
* Timeout.
* Max Connections.

### src/config/env.rs

Đọc và quản lý các biến môi trường từ file `.env`.

Ví dụ:

* DATABASE_URL
* JWT_SECRET
* JWT_EXPIRE
* APP_PORT

---

## src/db/

Quản lý kết nối cơ sở dữ liệu.

### src/db/connection.rs

Khởi tạo PostgreSQL Connection Pool và cung cấp kết nối cho toàn bộ hệ thống.

### Chức năng:

* Kết nối PostgreSQL.
* Tạo Pool kết nối.
* Tái sử dụng kết nối.
* Quản lý số lượng kết nối đồng thời.

---

## src/dto/

Data Transfer Object.

### Mục đích:

* Nhận dữ liệu từ Request.
* Trả dữ liệu về Response.
* Tách biệt dữ liệu API với Model.
* Kiểm soát dữ liệu đầu vào và đầu ra.

### src/dto/auth/

DTO phục vụ xác thực người dùng.

#### Bao gồm:

* RegisterRequest
* LoginRequest
* RefreshTokenRequest
* AuthResponse

### src/dto/user/

DTO liên quan đến người dùng.

#### Bao gồm:

* UpdateProfileRequest
* ChangePasswordRequest
* UserResponse

### src/dto/todo/

DTO liên quan đến công việc.

#### Bao gồm:

* CreateTodoRequest
* UpdateTodoRequest
* TodoResponse

### src/dto/category/

DTO liên quan đến danh mục công việc.

#### Bao gồm:

* CreateCategoryRequest
* UpdateCategoryRequest

### src/dto/priority/

DTO liên quan đến mức độ ưu tiên.

#### Bao gồm:

* CreatePriorityRequest
* UpdatePriorityRequest

### src/dto/reminder/

DTO liên quan đến nhắc việc.

#### Bao gồm:

* CreateReminderRequest
* UpdateReminderRequest

### src/dto/notification/

DTO liên quan đến thông báo.

#### Bao gồm:

* NotificationResponse
* NotificationSettingRequest

---

## src/error/

Quản lý lỗi tập trung của hệ thống.

### src/error/app_error.rs

Định nghĩa các lỗi nghiệp vụ và lỗi hệ thống.

#### Ví dụ:

* Unauthorized
* Forbidden
* ValidationError
* NotFound
* InternalServerError

### src/error/mod.rs

Export toàn bộ module lỗi.

---

## src/handlers/

Lớp tiếp nhận Request từ client.

### Nhiệm vụ:

* Nhận HTTP Request.
* Validate dữ liệu cơ bản.
* Gọi Service tương ứng.
* Trả JSON Response.

### src/handlers/auth_handler.rs

Xử lý API xác thực:

* Đăng ký.
* Đăng nhập.
* Đăng xuất.
* Refresh Token.

### src/handlers/user_handler.rs

Xử lý API người dùng:

* Xem thông tin cá nhân.
* Cập nhật hồ sơ.
* Đổi mật khẩu.

### src/handlers/todo_handler.rs

Xử lý API công việc:

* Tạo công việc.
* Cập nhật công việc.
* Xoá công việc.
* Lấy danh sách công việc.

### src/handlers/category_handler.rs

Xử lý API danh mục công việc.

### src/handlers/priority_handler.rs

Xử lý API mức độ ưu tiên.

### src/handlers/reminder_handler.rs

Xử lý API nhắc việc.

### src/handlers/notification_handler.rs

Xử lý API thông báo.

---

## src/middleware/

Chứa các Middleware của hệ thống.

### src/middleware/auth.rs

Middleware xác thực JWT.

### Chức năng:

* Kiểm tra Access Token.
* Xác thực người dùng.
* Gắn User Context vào Request.

### src/middleware/logger.rs

Middleware ghi log hệ thống.

### Chức năng:

* Ghi log Request.
* Ghi log Response.
* Theo dõi thời gian xử lý.

---

## src/models/

Đại diện cho các bảng trong cơ sở dữ liệu.

### src/models/role.rs

Đại diện bảng:

* roles

### src/models/user.rs

Đại diện bảng:

* users

### src/models/refresh_token.rs

Đại diện bảng:

* refresh_tokens

### src/models/todo_category.rs

Đại diện bảng:

* todo_categories

### src/models/todo_priority.rs

Đại diện bảng:

* todo_priorities

### src/models/todo.rs

Đại diện bảng:

* todos

### src/models/todo_reminder.rs

Đại diện bảng:

* todo_reminders

### src/models/notification_log.rs

Đại diện bảng:

* notification_logs

Lưu lịch sử gửi thông báo.

### src/models/user_notification_setting.rs

Đại diện bảng:

* user_notification_settings

Lưu cấu hình nhận thông báo của người dùng.

---

## src/repositories/

Tầng truy cập dữ liệu.

### Nhiệm vụ:

* Thực hiện truy vấn SQL.
* CRUD dữ liệu.
* Không chứa nghiệp vụ.

### src/repositories/role_repository.rs

Thao tác dữ liệu Role.

### src/repositories/user_repository.rs

Thao tác dữ liệu User.

### src/repositories/refresh_token_repository.rs

Thao tác dữ liệu Refresh Token.

### src/repositories/category_repository.rs

Thao tác dữ liệu Category.

### src/repositories/priority_repository.rs

Thao tác dữ liệu Priority.

### src/repositories/todo_repository.rs

Thao tác dữ liệu Todo.

### src/repositories/reminder_repository.rs

Thao tác dữ liệu Reminder.

### src/repositories/notification_repository.rs

Thao tác dữ liệu Notification.

---

## src/routes/

Định nghĩa các API Endpoint.

### src/routes/auth_routes.rs

Đăng ký các route liên quan đến xác thực.

### src/routes/user_routes.rs

Đăng ký các route liên quan đến người dùng.

### src/routes/todo_routes.rs

Đăng ký các route liên quan đến công việc.

### src/routes/category_routes.rs

Đăng ký các route liên quan đến danh mục.

### src/routes/priority_routes.rs

Đăng ký các route liên quan đến độ ưu tiên.

### src/routes/reminder_routes.rs

Đăng ký các route liên quan đến nhắc việc.

### src/routes/notification_routes.rs

Đăng ký các route liên quan đến thông báo.

### src/routes/mod.rs

Tập hợp toàn bộ route của hệ thống.

---

## src/services/

Tầng xử lý nghiệp vụ của hệ thống.

### src/services/auth_service.rs

Xử lý:

* Đăng ký.
* Đăng nhập.
* Logout.
* Refresh Token.

### src/services/user_service.rs

Xử lý:

* Hồ sơ người dùng.
* Đổi mật khẩu.

### src/services/todo_service.rs

Xử lý:

* Tạo công việc.
* Cập nhật công việc.
* Đánh dấu hoàn thành.
* Lọc công việc.

### src/services/category_service.rs

Xử lý nghiệp vụ Category.

### src/services/priority_service.rs

Xử lý nghiệp vụ Priority.

### src/services/reminder_service.rs

Xử lý nghiệp vụ Reminder.

### src/services/notification_service.rs

Xử lý:

* Gửi thông báo.
* Quản lý cài đặt thông báo.
* Lưu lịch sử gửi thông báo.

---

## src/utils/

Chứa các hàm tiện ích dùng chung.

### src/utils/jwt.rs

Quản lý JWT.

#### Chức năng:

* Tạo Token.
* Kiểm tra Token.
* Giải mã Token.

### src/utils/password.rs

Quản lý mật khẩu.

#### Chức năng:

* Hash Password.
* Verify Password.

### src/utils/response.rs

Chuẩn hóa Response trả về API.

#### Ví dụ:

* Success Response.
* Error Response.
* Pagination Response.

### src/utils/date.rs

Xử lý ngày giờ.

#### Chức năng:

* Format DateTime.
* UTC Conversion.
* Date Helper.

---

## src/app_state.rs

Quản lý Shared State của toàn bộ ứng dụng.

### Chứa:

* Database Pool.
* Config.
* Services dùng chung.

---

## src/lib.rs

Export các module để tái sử dụng trong toàn bộ dự án.

---

## src/main.rs

Điểm khởi động của ứng dụng.

### Nhiệm vụ:

* Load ENV.
* Khởi tạo Config.
* Kết nối Database.
* Khởi tạo AppState.
* Đăng ký Routes.
* Đăng ký Middleware.
* Khởi động Axum Server.

---

## .env

Chứa các biến môi trường của dự án.

Ví dụ:

* DATABASE_URL
* JWT_SECRET
* APP_PORT

---

## .env.example

Mẫu cấu hình môi trường dành cho thành viên mới của dự án.

---

## .gitignore

Danh sách các file và thư mục không được đẩy lên Git.

Ví dụ:

* target/
* .env
* .idea/
* .vscode/

---

## Cargo.toml

Khai báo thông tin dự án và các thư viện phụ thuộc (Dependencies).

---

## Cargo.lock

Lưu phiên bản chính xác của các thư viện đã cài đặt để đảm bảo môi trường build đồng nhất giữa các máy.
