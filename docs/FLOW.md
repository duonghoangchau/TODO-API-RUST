# Luồng Xử Lý Dự Án Todo API (Rust + Axum + Supabase)

## Tổng Quan Luồng Hệ Thống

Dự án Todo API hoạt động theo mô hình Backend API riêng biệt.

Frontend hoặc Postman sẽ gửi request đến Backend Rust. Backend Rust xử lý nghiệp vụ, xác thực người dùng, truy vấn dữ liệu và lưu trữ dữ liệu vào PostgreSQL trên Supabase.

```text
Client / Postman / Frontend
        ↓
Rust Backend API
        ↓
Supabase PostgreSQL
```

---

## Luồng Khởi Động Ứng Dụng

Khi chạy project bằng lệnh:

```bash
cargo run
```

Hệ thống sẽ thực hiện theo luồng sau:

```text
main.rs
   ↓
Load biến môi trường từ .env
   ↓
Đọc cấu hình ứng dụng
   ↓
Kết nối Supabase PostgreSQL
   ↓
Khởi tạo AppState
   ↓
Đăng ký Routes
   ↓
Đăng ký Middleware
   ↓
Khởi động Axum Server
```

### Mô tả chi tiết

1. `main.rs` là điểm bắt đầu của ứng dụng.
2. Hệ thống đọc thông tin trong file `.env`.
3. Lấy `DATABASE_URL` để kết nối tới Supabase PostgreSQL.
4. Tạo database connection pool.
5. Tạo `AppState` để chia sẻ database pool cho toàn bộ ứng dụng.
6. Đăng ký các route như Auth, User, Todo, Category, Priority, Reminder, Notification.
7. Đăng ký middleware như Auth Middleware và Logger Middleware.
8. Server bắt đầu chạy tại địa chỉ cấu hình, ví dụ:

```text
http://localhost:8000
```

---

## Luồng Tổng Quát Khi Gọi API

Mỗi request gửi vào hệ thống sẽ đi qua các tầng xử lý sau:

```text
Client / Postman
      ↓
Route
      ↓
Middleware
      ↓
Handler
      ↓
Service
      ↓
Repository
      ↓
Supabase PostgreSQL
      ↓
Repository
      ↓
Service
      ↓
Handler
      ↓
JSON Response
      ↓
Client / Postman
```

### Vai trò từng tầng

### Route

Xác định request đang gọi đến endpoint nào.

Ví dụ:

```text
POST /api/todos
GET /api/todos
PUT /api/todos/:id
DELETE /api/todos/:id
```

### Middleware

Xử lý trước khi request vào Handler.

Ví dụ:

* Kiểm tra JWT Token.
* Ghi log request.
* Kiểm tra quyền truy cập.

### Handler

Nhận request từ client.

Nhiệm vụ:

* Lấy dữ liệu từ body, params, query.
* Gọi Service tương ứng.
* Trả response về client.

### Service

Xử lý nghiệp vụ chính của hệ thống.

Ví dụ:

* Kiểm tra dữ liệu hợp lệ.
* Kiểm tra quyền sở hữu Todo.
* Kiểm tra email đã tồn tại chưa.
* Hash password.
* Tạo access token.

### Repository

Làm việc trực tiếp với database.

Nhiệm vụ:

* Insert dữ liệu.
* Select dữ liệu.
* Update dữ liệu.
* Delete dữ liệu.

### Supabase PostgreSQL

Lưu trữ dữ liệu thật của hệ thống.

---

## Luồng Đăng Ký Tài Khoản

API:

```http
POST /api/auth/register
```

Body ví dụ:

```json
{
  "name": "Chau",
  "email": "chau@gmail.com",
  "password": "123456"
}
```

Luồng xử lý:

```text
Client / Postman
      ↓
auth_routes.rs
      ↓
auth_handler.rs
      ↓
auth_service.rs
      ↓
user_repository.rs
      ↓
Supabase PostgreSQL
```

### Các bước xử lý

1. Client gửi thông tin đăng ký.
2. Route chuyển request đến `auth_handler.rs`.
3. Handler nhận dữ liệu từ body.
4. Service kiểm tra email đã tồn tại chưa.
5. Service hash password.
6. Repository lưu user mới vào bảng `users`.
7. Service tạo access token và refresh token.
8. Repository lưu refresh token vào bảng `refresh_tokens`.
9. Handler trả response về client.

Response ví dụ:

```json
{
  "success": true,
  "message": "Register successfully",
  "data": {
    "access_token": "...",
    "refresh_token": "...",
    "user": {
      "id": 1,
      "name": "Chau",
      "email": "chau@gmail.com"
    }
  }
}
```

---

## Luồng Đăng Nhập

API:

```http
POST /api/auth/login
```

Body ví dụ:

```json
{
  "email": "chau@gmail.com",
  "password": "123456"
}
```

Luồng xử lý:

```text
Client / Postman
      ↓
auth_routes.rs
      ↓
auth_handler.rs
      ↓
auth_service.rs
      ↓
user_repository.rs
      ↓
refresh_token_repository.rs
      ↓
Supabase PostgreSQL
```

### Các bước xử lý

1. Client gửi email và password.
2. Handler nhận dữ liệu login.
3. Service tìm user theo email.
4. Nếu user không tồn tại, trả lỗi.
5. Nếu user tồn tại, kiểm tra password.
6. Nếu password sai, trả lỗi.
7. Nếu đúng, tạo access token và refresh token.
8. Lưu refresh token vào database.
9. Trả token về client.

---

## Luồng Xác Thực JWT

Các API cần đăng nhập sẽ yêu cầu Header:

```http
Authorization: Bearer <access_token>
```

Luồng xử lý:

```text
Client / Postman
      ↓
Request có Authorization Header
      ↓
auth middleware
      ↓
Verify JWT
      ↓
Lấy user_id từ token
      ↓
Cho request đi tiếp vào Handler
```

### Nếu token hợp lệ

Request tiếp tục đi vào Handler.

### Nếu token không hợp lệ

Hệ thống trả về lỗi:

```json
{
  "success": false,
  "message": "Unauthorized"
}
```

---

## Luồng Tạo Todo

API:

```http
POST /api/todos
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "title": "Học Rust",
  "description": "Làm Todo API bằng Axum",
  "category_id": 1,
  "priority_id": 2,
  "due_date": "2026-06-10"
}
```

Luồng xử lý:

```text
Client / Postman
      ↓
todo_routes.rs
      ↓
auth middleware
      ↓
todo_handler.rs
      ↓
todo_service.rs
      ↓
todo_repository.rs
      ↓
Supabase PostgreSQL
```

### Các bước xử lý

1. Client gửi request tạo Todo kèm access token.
2. Auth Middleware kiểm tra token.
3. Nếu token hợp lệ, lấy `user_id`.
4. Handler nhận dữ liệu Todo từ body.
5. Service kiểm tra dữ liệu hợp lệ.
6. Service kiểm tra Category và Priority có tồn tại không.
7. Repository lưu Todo vào bảng `todos`.
8. Handler trả Todo vừa tạo về client.

---

## Luồng Lấy Danh Sách Todo

API:

```http
GET /api/todos
```

Header:

```http
Authorization: Bearer <access_token>
```

Luồng xử lý:

```text
Client / Postman
      ↓
todo_routes.rs
      ↓
auth middleware
      ↓
todo_handler.rs
      ↓
todo_service.rs
      ↓
todo_repository.rs
      ↓
Supabase PostgreSQL
```

### Các bước xử lý

1. Client gửi request lấy danh sách Todo.
2. Middleware xác thực access token.
3. Service lấy `user_id` từ request context.
4. Repository query danh sách Todo theo `user_id`.
5. Database trả dữ liệu Todo.
6. Handler trả response về client.

---

## Luồng Cập Nhật Todo

API:

```http
PUT /api/todos/:id
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "title": "Học Rust nâng cao",
  "is_completed": true
}
```

### Các bước xử lý

1. Client gửi request cập nhật Todo.
2. Middleware kiểm tra token.
3. Handler lấy `todo_id` từ params.
4. Service kiểm tra Todo có tồn tại không.
5. Service kiểm tra Todo có thuộc về user hiện tại không.
6. Repository cập nhật dữ liệu trong bảng `todos`.
7. Handler trả Todo đã cập nhật.

---

## Luồng Xoá Todo

API:

```http
DELETE /api/todos/:id
```

Header:

```http
Authorization: Bearer <access_token>
```

### Các bước xử lý

1. Client gửi request xoá Todo.
2. Middleware kiểm tra token.
3. Handler lấy `todo_id`.
4. Service kiểm tra Todo tồn tại.
5. Service kiểm tra Todo thuộc về user hiện tại.
6. Repository xoá Todo khỏi database.
7. Handler trả response xoá thành công.

---

## Luồng Tạo Category

API:

```http
POST /api/categories
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "name": "Công việc",
  "color": "#FF5733"
}
```

### Các bước xử lý

1. Client gửi request tạo Category.
2. Middleware xác thực người dùng.
3. Handler nhận dữ liệu Category.
4. Service kiểm tra tên Category hợp lệ.
5. Repository lưu Category vào bảng `todo_categories`.
6. Handler trả Category vừa tạo.

---

## Luồng Tạo Priority

API:

```http
POST /api/priorities
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "name": "High",
  "level": 3,
  "color": "#FF0000"
}
```

### Các bước xử lý

1. Client gửi request tạo Priority.
2. Middleware xác thực người dùng.
3. Handler nhận dữ liệu Priority.
4. Service kiểm tra dữ liệu hợp lệ.
5. Repository lưu Priority vào bảng `todo_priorities`.
6. Handler trả Priority vừa tạo.

---

## Luồng Tạo Reminder

API:

```http
POST /api/reminders
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "todo_id": 1,
  "remind_at": "2026-06-10T08:00:00Z"
}
```

### Các bước xử lý

1. Client gửi request tạo Reminder.
2. Middleware xác thực người dùng.
3. Handler nhận dữ liệu Reminder.
4. Service kiểm tra Todo có tồn tại không.
5. Service kiểm tra Todo có thuộc về user hiện tại không.
6. Repository lưu Reminder vào bảng `todo_reminders`.
7. Handler trả Reminder vừa tạo.

---

## Luồng Ghi Notification Log

Notification Log dùng để lưu lại lịch sử gửi thông báo.

Luồng xử lý:

```text
Reminder đến thời gian cần nhắc
      ↓
notification_service.rs
      ↓
notification_repository.rs
      ↓
notification_logs
```

### Các bước xử lý

1. Hệ thống kiểm tra các Reminder đến hạn.
2. Notification Service tạo nội dung thông báo.
3. Gửi thông báo cho người dùng.
4. Repository lưu lịch sử gửi vào bảng `notification_logs`.

---

## Luồng Refresh Token

API:

```http
POST /api/auth/refresh
```

Body ví dụ:

```json
{
  "refresh_token": "..."
}
```

### Các bước xử lý

1. Client gửi refresh token.
2. Handler nhận refresh token.
3. Service kiểm tra refresh token có tồn tại trong database không.
4. Service kiểm tra refresh token còn hạn không.
5. Nếu hợp lệ, tạo access token mới.
6. Handler trả access token mới về client.

---

## Luồng Logout

API:

```http
POST /api/auth/logout
```

Header:

```http
Authorization: Bearer <access_token>
```

Body ví dụ:

```json
{
  "refresh_token": "..."
}
```

### Các bước xử lý

1. Client gửi request logout.
2. Middleware xác thực access token.
3. Service xoá hoặc vô hiệu hoá refresh token.
4. Handler trả response logout thành công.

---

## Luồng Test API Bằng Postman

### Bước 1: Chạy server

```bash
cargo run
```

### Bước 2: Đăng ký tài khoản

```http
POST http://localhost:8000/api/auth/register
```

### Bước 3: Đăng nhập

```http
POST http://localhost:8000/api/auth/login
```

### Bước 4: Copy access token

Sau khi đăng nhập thành công, copy `access_token`.

### Bước 5: Gọi API cần xác thực

Thêm header:

```http
Authorization: Bearer <access_token>
```

### Bước 6: Test các API Todo

```http
GET    /api/todos
POST   /api/todos
PUT    /api/todos/:id
DELETE /api/todos/:id
```

---

## Luồng Dữ Liệu Với Supabase

Supabase trong dự án này đóng vai trò là PostgreSQL Cloud Database.

```text
Rust Backend
      ↓
SQLx / PostgreSQL Driver
      ↓
Supabase PostgreSQL
```

### Backend sẽ thực hiện:

* Tạo bảng thông qua migration.
* Thêm dữ liệu thông qua repository.
* Cập nhật dữ liệu thông qua repository.
* Truy vấn dữ liệu thông qua repository.
* Xoá dữ liệu thông qua repository.

---

## Luồng Migration Database

Khi chạy lệnh:

```bash
sqlx migrate run
```

Hệ thống sẽ đọc các file trong thư mục `migrations/` và thực thi lên Supabase PostgreSQL.

Luồng:

```text
migrations/
      ↓
sqlx migrate run
      ↓
DATABASE_URL
      ↓
Supabase PostgreSQL
      ↓
Tạo bảng dữ liệu
```

Sau khi chạy migration, các bảng sẽ được tạo trên Supabase:

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

---

## Luồng Seed Dữ Liệu

Seed dùng để thêm dữ liệu mặc định vào hệ thống.

Ví dụ:

```text
roles
todo_priorities
todo_categories
user_notification_settings
```

Luồng:

```text
seeds/
   ↓
Chạy seed script
   ↓
Insert dữ liệu mặc định
   ↓
Supabase PostgreSQL
```

---

## Tổng Kết

Luồng xử lý chính của hệ thống là:

```text
Client / Postman
      ↓
Rust Axum API
      ↓
Handler
      ↓
Service
      ↓
Repository
      ↓
Supabase PostgreSQL
      ↓
JSON Response
      ↓
Client / Postman
```

Cách tổ chức này giúp dự án rõ ràng, dễ bảo trì, dễ mở rộng và phù hợp với một backend API thực tế.
