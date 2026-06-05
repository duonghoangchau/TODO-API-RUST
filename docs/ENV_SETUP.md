# ENV_SETUP.md

# Hướng Dẫn Cấu Hình Môi Trường Todo API

## Tổng Quan

Tài liệu này hướng dẫn cấu hình môi trường cho dự án Todo API sử dụng:

```text
Rust
Axum
SQLx
PostgreSQL
Supabase
JWT
```

Backend sẽ kết nối trực tiếp tới PostgreSQL của Supabase thông qua `DATABASE_URL`.

---

## Yêu Cầu Cài Đặt

Trước khi chạy project, cần cài:

```text
Rust
Cargo
SQLx CLI
Postman
Git
```

Kiểm tra Rust:

```bash
rustc --version
```

Kiểm tra Cargo:

```bash
cargo --version
```

---

## Cài SQLx CLI

Dự án sử dụng SQLx để chạy migration.

Cài SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

Kiểm tra:

```bash
sqlx --version
```

---

## Tạo Project Supabase

### Bước 1: Tạo Supabase Project

1. Vào Supabase.
2. Tạo project mới.
3. Chọn region gần Việt Nam, ví dụ Singapore.
4. Lưu lại database password.

### Bước 2: Lấy Database URL

Vào:

```text
Project Settings
→ Database
→ Connection string
```

Chọn dạng URI.

Ví dụ:

```env
DATABASE_URL=postgresql://postgres.xxxxx:YOUR_PASSWORD@aws-0-ap-southeast-1.pooler.supabase.com:6543/postgres
```

Lưu ý thay `YOUR_PASSWORD` bằng mật khẩu database thật.

---

## Tạo File .env

Tạo file `.env` ở thư mục gốc project:

```text
todo-api/.env
```

Nội dung mẫu:

```env
APP_NAME=Todo API
APP_ENV=local
APP_HOST=127.0.0.1
APP_PORT=8000

DATABASE_URL=postgresql://postgres.xxxxx:YOUR_PASSWORD@aws-0-ap-southeast-1.pooler.supabase.com:6543/postgres

JWT_SECRET=your-super-secret-key
JWT_ACCESS_TOKEN_EXPIRE_MINUTES=15
JWT_REFRESH_TOKEN_EXPIRE_DAYS=30

RUST_LOG=debug
```

---

## Tạo File .env.example

Tạo file `.env.example` để commit lên Git.

```env
APP_NAME=Todo API
APP_ENV=local
APP_HOST=127.0.0.1
APP_PORT=8000

DATABASE_URL=postgresql://postgres.xxxxx:YOUR_PASSWORD@your-supabase-host:6543/postgres

JWT_SECRET=change-me
JWT_ACCESS_TOKEN_EXPIRE_MINUTES=15
JWT_REFRESH_TOKEN_EXPIRE_DAYS=30

RUST_LOG=debug
```

Lưu ý:

```text
.env được dùng thật trên máy local.
.env.example chỉ là file mẫu.
Không đưa mật khẩu thật vào .env.example.
```

---

## Cấu Hình .gitignore

Đảm bảo `.gitignore` có:

```gitignore
/target
.env
.env.local
*.log
.DS_Store
.idea/
.vscode/
```

Quan trọng nhất:

```gitignore
.env
```

Không được commit file `.env` lên Git.

---

## Kiểm Tra Cargo.toml

Các dependency đề xuất:

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.6", features = ["cors", "trace"] }

serde = { version = "1", features = ["derive"] }
serde_json = "1"

sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "macros"] }

dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"

jsonwebtoken = "9"
argon2 = "0.5"
rand = "0.8"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }
thiserror = "2"
```

---

## Chạy Migration

Sau khi đã có `DATABASE_URL`, chạy:

```bash
sqlx migrate run
```

Nếu thành công, Supabase sẽ có các bảng:

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

## Chạy Seed

Sau khi migration xong, chạy seed dữ liệu mặc định.

Các bảng cần seed:

```text
roles
todo_categories
todo_priorities
```

Thứ tự seed:

```text
1. roles
2. todo_categories
3. todo_priorities
```

Ví dụ nếu seed bằng file SQL, có thể chạy thủ công trong Supabase SQL Editor hoặc thông qua script riêng của project.

---

## Chạy Project Local

Chạy server:

```bash
cargo run
```

Nếu thành công, server chạy tại:

```text
http://127.0.0.1:8000
```

Hoặc:

```text
http://localhost:8000
```

---

## Test Health Check

Nếu có API health check:

```http
GET http://localhost:8000/api/health
```

Response mong muốn:

```json
{
  "success": true,
  "message": "Todo API is running"
}
```

---

## Test Kết Nối Database

Có thể tạo API test database tạm thời:

```http
GET http://localhost:8000/api/health/db
```

Response mong muốn:

```json
{
  "success": true,
  "message": "Database connected"
}
```

Sau khi test xong có thể giữ lại hoặc xoá API này.

---

## Lỗi Thường Gặp

## 1. Không đọc được .env

Kiểm tra đã gọi:

```rust
dotenvy::dotenv().ok();
```

trong `main.rs`.

---

## 2. DATABASE_URL sai

Kiểm tra:

```text
Host
Port
Username
Password
Database Name
SSL mode
```

Với Supabase, nên dùng connection string chính xác trong phần Database Settings.

---

## 3. Lỗi password chứa ký tự đặc biệt

Nếu password có ký tự như:

```text
@
#
%
&
/
```

cần encode password trong URL.

Ví dụ:

```text
@ -> %40
# -> %23
% -> %25
& -> %26
/ -> %2F
```

---

## 4. SQLx CLI chưa cài

Cài lại:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

---

## 5. Không chạy được migration

Kiểm tra thư mục:

```text
migrations/
```

và biến môi trường:

```env
DATABASE_URL=...
```

---

## 6. Port đã được sử dụng

Nếu port `8000` bị chiếm, đổi trong `.env`:

```env
APP_PORT=8001
```

---

## Checklist Trước Khi Code

```text
Rust đã cài
Cargo chạy được
SQLx CLI đã cài
Supabase project đã tạo
DATABASE_URL đã cấu hình
.env đã có
.env đã được thêm vào .gitignore
cargo run chạy được
sqlx migrate run chạy được
Postman đã sẵn sàng để test API
```

---

## Kết Luận

Sau khi hoàn thành cấu hình môi trường, backend Rust có thể kết nối trực tiếp với Supabase PostgreSQL, chạy migration, seed dữ liệu và test API bằng Postman.
