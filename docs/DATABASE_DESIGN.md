roles
  │
  └──────< users
              │
              ├──────< refresh_tokens
              │
              ├──────< todos
              │             │
              │             ├──────> todo_categories
              │             │
              │             ├──────> todo_priorities
              │             │
              │             └──────< todo_reminders
              │
              ├──────< notification_logs
              │
              └──────< user_notification_settings

# Thiết Kế Database Todo API
## Tổng Quan
Dự án Todo API sử dụng PostgreSQL trên Supabase để lưu trữ dữ liệu.

Danh sách bảng chính:

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

# Thứ Tự Tạo Bảng
Nên tạo bảng theo thứ tự sau để tránh lỗi khóa ngoại:

```text
1. roles
2. users
3. refresh_tokens
4. todo_categories
5. todo_priorities
6. todos
7. todo_reminders
8. notification_logs
9. user_notification_settings
```

---

# Thứ Tự Chạy Seed
Các bảng cần seed dữ liệu ban đầu:

```text
1. roles
2. todo_categories
3. todo_priorities
4. user_notification_settings
```

Trong đó:

* `roles` nên chạy đầu tiên.
* `todo_categories` và `todo_priorities` chạy trước khi tạo Todo.
* `user_notification_settings` có thể tạo mặc định sau khi user được tạo.

---

# 1. Bảng roles
## Mục đích

Lưu vai trò của người dùng trong hệ thống.

Ví dụ:

* ADMIN
* USER

## Tên bảng
```text
roles
```

## Các cột

| Cột         | Kiểu dữ liệu | Mô tả              |
| ----------- | ------------ | ------------------ |
| id          | BIGSERIAL    | Khóa chính         |
| name        | VARCHAR(50)  | Tên vai trò        |
| description | TEXT         | Mô tả vai trò      |
| created_at  | TIMESTAMP    | Thời gian tạo      |
| updated_at  | TIMESTAMP    | Thời gian cập nhật |

## Seed cần chạy
```sql
INSERT INTO roles (id, name, description, created_at, updated_at)
VALUES
(1, 'ADMIN', 'Quản trị viên hệ thống', NOW(), NOW()),
(2, 'USER', 'Người dùng thông thường', NOW(), NOW());
```

---

# 2. Bảng users
## Mục đích

Lưu thông tin tài khoản người dùng.

## Tên bảng
```text
users
```

## Các cột

| Cột           | Kiểu dữ liệu | Mô tả                     |
| ------------- | ------------ | ------------------------- |
| id            | BIGSERIAL    | Khóa chính                |
| role_id       | BIGINT       | Khóa ngoại đến bảng roles |
| full_name     | VARCHAR(255) | Họ tên người dùng         |
| email         | VARCHAR(255) | Email đăng nhập           |
| password_hash | TEXT         | Mật khẩu đã được mã hóa   |
| avatar_url    | TEXT         | Link ảnh đại diện         |
| is_active     | BOOLEAN      | Trạng thái tài khoản      |
| created_at    | TIMESTAMP    | Thời gian tạo             |
| updated_at    | TIMESTAMP    | Thời gian cập nhật        |

## Quan hệ

```text
users.role_id -> roles.id
```

## Seed cần chạy
Không bắt buộc.

Có thể tạo user thông qua API đăng ký:

```http
POST /api/auth/register
```

Nếu muốn tạo admin mặc định, có thể seed sau khi đã seed `roles`.

---

# 3. Bảng refresh_tokens
## Mục đích
Lưu refresh token dùng để cấp lại access token khi JWT hết hạn.

## Tên bảng
```text
refresh_tokens
```

## Các cột

| Cột        | Kiểu dữ liệu   | Mô tả                     |
| ---------- | -------------- | ------------------------- |
| id         | BIGSERIAL      | Khóa chính                |
| user_id    | BIGINT         | Khóa ngoại đến bảng users |
| token      | TEXT           | Refresh token             |
| expires_at | TIMESTAMP      | Thời gian hết hạn         |
| revoked_at | TIMESTAMP NULL | Thời gian bị thu hồi      |
| created_at | TIMESTAMP      | Thời gian tạo             |

## Quan hệ
```text
refresh_tokens.user_id -> users.id
```

## Seed cần chạy
Không cần seed.

Bảng này sẽ tự có dữ liệu khi người dùng đăng nhập.

---

# 4. Bảng todo_categories
## Mục đích
Lưu danh mục công việc.

Ví dụ:

* Công việc
* Cá nhân
* Học tập

## Tên bảng
```text
todo_categories
```

## Các cột

| Cột        | Kiểu dữ liệu | Mô tả                           |
| ---------- | ------------ | ------------------------------- |
| id         | BIGSERIAL    | Khóa chính                      |
| name       | VARCHAR(100) | Tên danh mục                    |
| slug       | VARCHAR(100) | Tên rút gọn dùng cho hệ thống   |
| color      | VARCHAR(20)  | Mã màu hiển thị                 |
| is_default | BOOLEAN      | Có phải danh mục mặc định không |
| created_at | TIMESTAMP    | Thời gian tạo                   |
| updated_at | TIMESTAMP    | Thời gian cập nhật              |

## Seed cần chạy
```sql
INSERT INTO todo_categories (id, name, slug, color, is_default, created_at, updated_at)
VALUES
(1, 'Công việc', 'work', '#3B82F6', true, NOW(), NOW()),
(2, 'Cá nhân', 'personal', '#22C55E', true, NOW(), NOW()),
(3, 'Học tập', 'study', '#A855F7', true, NOW(), NOW()),
(4, 'Khác', 'other', '#6B7280', true, NOW(), NOW());
```

---

# 5. Bảng todo_priorities
## Mục đích
Lưu mức độ ưu tiên của công việc.

Ví dụ:

* Low
* Medium
* High

## Tên bảng
```text
todo_priorities
```

## Các cột
| Cột        | Kiểu dữ liệu | Mô tả                         |
| ---------- | ------------ | ----------------------------- |
| id         | BIGSERIAL    | Khóa chính                    |
| name       | VARCHAR(100) | Tên mức độ ưu tiên            |
| level      | INT          | Cấp độ ưu tiên                |
| color      | VARCHAR(20)  | Màu hiển thị                  |
| is_default | BOOLEAN      | Có phải mức độ mặc định không |
| created_at | TIMESTAMP    | Thời gian tạo                 |
| updated_at | TIMESTAMP    | Thời gian cập nhật            |

## Seed cần chạy
```sql
INSERT INTO todo_priorities (id, name, level, color, is_default, created_at, updated_at)
VALUES
(1, 'Low', 1, '#22C55E', true, NOW(), NOW()),
(2, 'Medium', 2, '#F59E0B', true, NOW(), NOW()),
(3, 'High', 3, '#EF4444', true, NOW(), NOW()),
(4, 'Urgent', 4, '#DC2626', true, NOW(), NOW());
```

---

# 6. Bảng todos
## Mục đích

Lưu thông tin công việc của người dùng.

## Tên bảng
```text
todos
```

## Các cột

| Cột          | Kiểu dữ liệu   | Mô tả                  |
| ------------ | -------------- | ---------------------- |
| id           | BIGSERIAL      | Khóa chính             |
| user_id      | BIGINT         | Người tạo Todo         |
| category_id  | BIGINT         | Danh mục công việc     |
| priority_id  | BIGINT         | Mức độ ưu tiên         |
| title        | VARCHAR(255)   | Tiêu đề công việc      |
| description  | TEXT           | Mô tả chi tiết         |
| status       | VARCHAR(50)    | Trạng thái công việc   |
| is_completed | BOOLEAN        | Đã hoàn thành hay chưa |
| due_date     | TIMESTAMP NULL | Hạn hoàn thành         |
| completed_at | TIMESTAMP NULL | Thời gian hoàn thành   |
| created_at   | TIMESTAMP      | Thời gian tạo          |
| updated_at   | TIMESTAMP      | Thời gian cập nhật     |

## Quan hệ
```text
todos.user_id -> users.id
todos.category_id -> todo_categories.id
todos.priority_id -> todo_priorities.id
```

## Giá trị status đề xuất
```text
pending
in_progress
completed
cancelled
```

## Seed cần chạy
Không bắt buộc.

Todo sẽ được tạo thông qua API:

```http
POST /api/todos
```

---

# 7. Bảng todo_reminders
## Mục đích

Lưu lịch nhắc việc cho Todo.

## Tên bảng
```text
todo_reminders
```

## Các cột

| Cột        | Kiểu dữ liệu   | Mô tả                 |
| ---------- | -------------- | --------------------- |
| id         | BIGSERIAL      | Khóa chính            |
| todo_id    | BIGINT         | Todo cần nhắc         |
| remind_at  | TIMESTAMP      | Thời gian nhắc        |
| message    | TEXT           | Nội dung nhắc         |
| is_sent    | BOOLEAN        | Đã gửi thông báo chưa |
| sent_at    | TIMESTAMP NULL | Thời gian đã gửi      |
| created_at | TIMESTAMP      | Thời gian tạo         |
| updated_at | TIMESTAMP      | Thời gian cập nhật    |

## Quan hệ
```text
todo_reminders.todo_id -> todos.id
```

## Seed cần chạy
Không cần seed.

Reminder sẽ được tạo thông qua API:

```http
POST /api/reminders
```

---

# 8. Bảng notification_logs
## Mục đích
Lưu lịch sử gửi thông báo cho người dùng.

## Tên bảng
```text
notification_logs
```

## Các cột

| Cột         | Kiểu dữ liệu   | Mô tả                |
| ----------- | -------------- | -------------------- |
| id          | BIGSERIAL      | Khóa chính           |
| user_id     | BIGINT         | Người nhận thông báo |
| todo_id     | BIGINT NULL    | Todo liên quan       |
| reminder_id | BIGINT NULL    | Reminder liên quan   |
| type        | VARCHAR(50)    | Loại thông báo       |
| title       | VARCHAR(255)   | Tiêu đề thông báo    |
| message     | TEXT           | Nội dung thông báo   |
| status      | VARCHAR(50)    | Trạng thái gửi       |
| sent_at     | TIMESTAMP NULL | Thời gian gửi        |
| created_at  | TIMESTAMP      | Thời gian tạo        |

## Quan hệ
```text
notification_logs.user_id -> users.id
notification_logs.todo_id -> todos.id
notification_logs.reminder_id -> todo_reminders.id
```

## Giá trị type đề xuất
```text
email
system
reminder
```

## Giá trị status đề xuất
```text
pending
sent
failed
```

## Seed cần chạy
Không cần seed.

Bảng này sẽ tự phát sinh dữ liệu khi hệ thống gửi thông báo.

---

# 9. Bảng user_notification_settings

## Mục đích
Lưu cấu hình nhận thông báo của từng người dùng.

## Tên bảng
```text
user_notification_settings
```

## Các cột

| Cột                   | Kiểu dữ liệu | Mô tả                      |
| --------------------- | ------------ | -------------------------- |
| id                    | BIGSERIAL    | Khóa chính                 |
| user_id               | BIGINT       | Người dùng                 |
| email_enabled         | BOOLEAN      | Bật/tắt thông báo email    |
| reminder_enabled      | BOOLEAN      | Bật/tắt nhắc việc          |
| daily_summary_enabled | BOOLEAN      | Bật/tắt tổng kết hằng ngày |
| created_at            | TIMESTAMP    | Thời gian tạo              |
| updated_at            | TIMESTAMP    | Thời gian cập nhật         |

## Quan hệ
```text
user_notification_settings.user_id -> users.id
```

## Seed cần chạy
Không seed trước được nếu chưa có user.

Bảng này nên được tạo tự động sau khi user đăng ký.

Ví dụ sau khi user đăng ký thành công:

```sql
INSERT INTO user_notification_settings (
    user_id,
    email_enabled,
    reminder_enabled,
    daily_summary_enabled,
    created_at,
    updated_at
)
VALUES (
    1,
    true,
    true,
    false,
    NOW(),
    NOW()
);
```

---

# Tổng Hợp Bảng Cần Seed Trước
## 1. roles

Bắt buộc seed trước.

```text
ADMIN
USER
```

## 2. todo_categories
Nên seed trước.

```text
Công việc
Cá nhân
Học tập
Khác
```

## 3. todo_priorities
Nên seed trước.

```text
Low
Medium
High
Urgent
```

## 4. user_notification_settings
Không seed global trước.

Tạo tự động khi user đăng ký tài khoản.

---

# Thứ Tự Chạy Seed Đề Xuất
```text
1. seed_roles.sql
2. seed_todo_categories.sql
3. seed_todo_priorities.sql
```

Sau đó khi user đăng ký:

```text
auth_service.rs
   ↓
Tạo user
   ↓
Tạo user_notification_settings mặc định
```

---

# Ghi Chú Quan Trọng
## Bảng không nên seed trước

```text
users
refresh_tokens
todos
todo_reminders
notification_logs
user_notification_settings
```

Lý do:

* `users` nên được tạo thông qua API đăng ký.
* `refresh_tokens` phát sinh khi đăng nhập.
* `todos` phát sinh khi người dùng tạo công việc.
* `todo_reminders` phát sinh khi người dùng tạo nhắc việc.
* `notification_logs` phát sinh khi hệ thống gửi thông báo.
* `user_notification_settings` phụ thuộc vào từng user.

---

# Kết Luận
Các bảng seed ban đầu cần thiết nhất cho hệ thống Todo API là:

```text
roles
todo_categories
todo_priorities
```

Các bảng còn lại sẽ phát sinh dữ liệu trong quá trình người dùng sử dụng hệ thống.
