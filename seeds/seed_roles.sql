INSERT INTO roles (id, name, description, created_at, updated_at)
VALUES
    (1, 'ADMIN', 'Quan tri vien he thong', NOW(), NOW()),
    (2, 'USER', 'Nguoi dung thong thuong', NOW(), NOW())
ON CONFLICT (id) DO UPDATE
SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    updated_at = NOW();
