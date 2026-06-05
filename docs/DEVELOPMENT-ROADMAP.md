# DEVELOPMENT-ROADMAP.md

# Lộ Trình Phát Triển Todo API

## Tổng Quan

Tài liệu này mô tả thứ tự phát triển dự án Todo API sử dụng Rust, Axum và Supabase PostgreSQL.

Mục tiêu là xây dựng backend API hoàn chỉnh, có thể test bằng Postman và kết nối với frontend sau này.

---

## Giai Đoạn 1: Khởi Tạo Project

### Mục tiêu

Tạo project Rust backend ban đầu.

### Công việc

1. Tạo project Rust:

```bash
cargo new todo-api
```

2. Di chuyển vào thư mục project:

```bash
cd todo-api
```

3. Cài các dependency cần thiết trong `Cargo.toml`.

4. Tạo cấu trúc thư mục:

```text
migrations/
seeds/
src/config/
src/db/
src/dto/
src/error/
src/handlers/
src/middleware/
src/models/
src/repositories/
src/routes/
src/services/
src/utils/
```

### Kết quả cần đạt

Project chạy được lệnh:

```bash
cargo run
```

---

## Giai Đoạn 2: Cấu Hình Môi Trường

### Mục tiêu

Thiết lập file `.env` và kết nối Supabase PostgreSQL.

### Công việc

1. Tạo file `.env`.
2. Tạo file `.env.example`.
3. Thêm biến môi trường.
4. Tạo project Supabase.
5. Copy Database URL từ Supabase.
6. Cấu hình `DATABASE_URL`.

### Kết quả cần đạt

Backend đọc được biến môi trường và kết nối được database.

---

## Giai Đoạn 3: Kết Nối Database

### Mục tiêu

Tạo connection pool đến Supabase PostgreSQL.

### Công việc

1. Viết `src/db/connection.rs`.
2. Viết config database.
3. Khởi tạo `PgPool`.
4. Inject `PgPool` vào `AppState`.

### Kết quả cần đạt

Khi chạy server, console báo kết nối database thành công.

---

## Giai Đoạn 4: Tạo Migration Database

### Mục tiêu

Tạo đầy đủ bảng cho hệ thống.

### Danh sách bảng

```text
roles
users
refresh_tokens
todo_categories
todo_priorities
todos
todo_reminders
notification_logs
user_notification_settings
```

### Công việc

1. Tạo migration cho bảng `roles`.
2. Tạo migration cho bảng `users`.
3. Tạo migration cho bảng `refresh_tokens`.
4. Tạo migration cho bảng `todo_categories`.
5. Tạo migration cho bảng `todo_priorities`.
6. Tạo migration cho bảng `todos`.
7. Tạo migration cho bảng `todo_reminders`.
8. Tạo migration cho bảng `notification_logs`.
9. Tạo migration cho bảng `user_notification_settings`.

### Kết quả cần đạt

Chạy được:

```bash
sqlx migrate run
```

Sau đó thấy đầy đủ bảng trong Supabase.

---

## Giai Đoạn 5: Seed Dữ Liệu Ban Đầu

### Mục tiêu

Thêm dữ liệu mặc định cho hệ thống.

### Bảng cần seed

```text
roles
todo_categories
todo_priorities
```

### Công việc

1. Seed role:

```text
ADMIN
USER
```

2. Seed category:

```text
Công việc
Cá nhân
Học tập
Khác
```

3. Seed priority:

```text
Low
Medium
High
Urgent
```

### Kết quả cần đạt

Database có dữ liệu mặc định trước khi test API.

---

## Giai Đoạn 6: Xây Dựng AppState Và Error Handler

### Mục tiêu

Tạo nền tảng dùng chung cho toàn bộ ứng dụng.

### Công việc

1. Tạo `src/app_state.rs`.
2. Tạo `src/error/app_error.rs`.
3. Tạo `src/error/mod.rs`.
4. Chuẩn hóa response lỗi.
5. Chuẩn hóa response thành công.

### Kết quả cần đạt

Handler có thể trả lỗi thống nhất dạng JSON.

---

## Giai Đoạn 7: Xây Dựng Model

### Mục tiêu

Tạo struct Rust đại diện cho các bảng database.

### Công việc

Tạo model cho:

```text
role.rs
user.rs
refresh_token.rs
todo_category.rs
todo_priority.rs
todo.rs
todo_reminder.rs
notification_log.rs
user_notification_setting.rs
```

### Kết quả cần đạt

Các model có thể mapping với dữ liệu PostgreSQL thông qua SQLx.

---

## Giai Đoạn 8: Xây Dựng DTO

### Mục tiêu

Tạo dữ liệu request và response cho API.

### Công việc

Tạo DTO cho:

```text
auth
user
todo
category
priority
reminder
notification
```

### Kết quả cần đạt

API có request body và response rõ ràng.

---

## Giai Đoạn 9: Xây Dựng Repository Layer

### Mục tiêu

Tách riêng phần truy vấn database.

### Công việc

Tạo repository cho:

```text
role_repository.rs
user_repository.rs
refresh_token_repository.rs
category_repository.rs
priority_repository.rs
todo_repository.rs
reminder_repository.rs
notification_repository.rs
```

### Kết quả cần đạt

Service có thể gọi Repository để thao tác database.

---

## Giai Đoạn 10: Xây Dựng Utility

### Mục tiêu

Viết các hàm tiện ích dùng chung.

### Công việc

1. `jwt.rs`

* Generate access token.
* Verify access token.
* Decode claims.

2. `password.rs`

* Hash password.
* Verify password.

3. `response.rs`

* Success response.
* Error response.

4. `date.rs`

* Format ngày giờ.
* Xử lý UTC.

### Kết quả cần đạt

Auth Service có thể sử dụng JWT và password helper.

---

## Giai Đoạn 11: Xây Dựng Auth Module

### Mục tiêu

Hoàn thiện đăng ký, đăng nhập, refresh token và logout.

### Công việc

1. Tạo `auth_service.rs`.
2. Tạo `auth_handler.rs`.
3. Tạo `auth_routes.rs`.
4. Tạo API đăng ký.
5. Tạo API đăng nhập.
6. Tạo API refresh token.
7. Tạo API logout.

### API cần có

```http
POST /api/auth/register
POST /api/auth/login
POST /api/auth/refresh
POST /api/auth/logout
```

### Kết quả cần đạt

Test được đăng ký và đăng nhập bằng Postman.

---

## Giai Đoạn 12: Xây Dựng Auth Middleware

### Mục tiêu

Bảo vệ các API cần đăng nhập.

### Công việc

1. Tạo `middleware/auth.rs`.
2. Đọc header Authorization.
3. Verify JWT.
4. Lấy `user_id` từ token.
5. Gắn user context vào request.
6. Trả lỗi nếu token sai hoặc hết hạn.

### Kết quả cần đạt

API Todo chỉ truy cập được khi có access token hợp lệ.

---

## Giai Đoạn 13: Xây Dựng User Module

### Mục tiêu

Quản lý thông tin người dùng.

### API cần có

```http
GET /api/users/me
PUT /api/users/me
PUT /api/users/change-password
```

### Công việc

1. Tạo `user_service.rs`.
2. Tạo `user_handler.rs`.
3. Tạo `user_routes.rs`.
4. Test bằng Postman.

### Kết quả cần đạt

Người dùng xem và cập nhật được thông tin cá nhân.

---

## Giai Đoạn 14: Xây Dựng Todo Module

### Mục tiêu

Xây dựng chức năng CRUD Todo.

### API cần có

```http
GET /api/todos
GET /api/todos/:id
POST /api/todos
PUT /api/todos/:id
DELETE /api/todos/:id
```

### Công việc

1. Tạo `todo_service.rs`.
2. Tạo `todo_handler.rs`.
3. Tạo `todo_routes.rs`.
4. Kiểm tra Todo thuộc về đúng user.
5. Hỗ trợ lọc theo status, category, priority.

### Kết quả cần đạt

Người dùng quản lý được Todo riêng của mình.

---

## Giai Đoạn 15: Xây Dựng Category Module

### Mục tiêu

Quản lý danh mục công việc.

### API cần có

```http
GET /api/categories
POST /api/categories
PUT /api/categories/:id
DELETE /api/categories/:id
```

### Kết quả cần đạt

Tạo, sửa, xoá, xem danh mục Todo.

---

## Giai Đoạn 16: Xây Dựng Priority Module

### Mục tiêu

Quản lý mức độ ưu tiên.

### API cần có

```http
GET /api/priorities
POST /api/priorities
PUT /api/priorities/:id
DELETE /api/priorities/:id
```

### Kết quả cần đạt

Tạo, sửa, xoá, xem mức độ ưu tiên Todo.

---

## Giai Đoạn 17: Xây Dựng Reminder Module

### Mục tiêu

Quản lý nhắc việc.

### API cần có

```http
GET /api/reminders
POST /api/reminders
PUT /api/reminders/:id
DELETE /api/reminders/:id
```

### Kết quả cần đạt

Người dùng tạo được nhắc việc cho Todo.

---

## Giai Đoạn 18: Xây Dựng Notification Module

### Mục tiêu

Lưu và quản lý lịch sử thông báo.

### API cần có

```http
GET /api/notifications
GET /api/notifications/settings
PUT /api/notifications/settings
```

### Công việc

1. Lấy danh sách notification log.
2. Lấy cấu hình notification.
3. Cập nhật cấu hình notification.
4. Tự động tạo notification setting khi user đăng ký.

### Kết quả cần đạt

Người dùng xem được lịch sử thông báo và cấu hình nhận thông báo.

---

## Giai Đoạn 19: Test API Bằng Postman

### Mục tiêu

Kiểm thử toàn bộ API.

### Thứ tự test

```text
1. Register
2. Login
3. Copy access_token
4. Get profile
5. Create category
6. Create priority
7. Create todo
8. Get todo list
9. Update todo
10. Create reminder
11. Get notifications
12. Logout
```

### Kết quả cần đạt

Tất cả API hoạt động đúng và trả JSON thống nhất.

---

## Giai Đoạn 20: Kiểm Tra Bảo Mật Cơ Bản

### Mục tiêu

Đảm bảo API an toàn ở mức cơ bản.

### Checklist

```text
Không lưu password plain text
JWT có thời hạn
Refresh token lưu database
API Todo yêu cầu login
User chỉ xem được Todo của chính mình
.env không được commit lên Git
DATABASE_URL không public
Input được validate
```

---

## Giai Đoạn 21: Chuẩn Bị Kết Nối Frontend

### Mục tiêu

Backend sẵn sàng cho Vue.js hoặc React.

### Công việc

1. Bật CORS.
2. Chuẩn hóa response.
3. Viết tài liệu API.
4. Export Postman Collection.
5. Deploy backend nếu cần.

### Kết quả cần đạt

Frontend có thể gọi API ổn định.

---

## Thứ Tự Code Đề Xuất

```text
1. Config + ENV
2. Database Connection
3. Migration
4. Seed
5. AppState
6. Error Handler
7. Models
8. DTO
9. Repositories
10. Utils
11. Auth
12. Middleware
13. User
14. Todo
15. Category
16. Priority
17. Reminder
18. Notification
19. Postman Testing
20. Frontend Integration
```

---

## Kết Luận

Nên hoàn thành từng giai đoạn theo đúng thứ tự. Không nên code Todo trước khi hoàn tất Auth và Middleware, vì hầu hết dữ liệu Todo đều phụ thuộc vào người dùng đã đăng nhập.

Lộ trình này giúp dự án dễ kiểm soát, dễ debug và phù hợp để đưa vào portfolio backend Rust.
