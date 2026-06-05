INSERT INTO todo_priorities (id, name, level, color, is_default, created_at, updated_at)
VALUES
    (1, 'Low', 1, '#22C55E', TRUE, NOW(), NOW()),
    (2, 'Medium', 2, '#F59E0B', TRUE, NOW(), NOW()),
    (3, 'High', 3, '#EF4444', TRUE, NOW(), NOW()),
    (4, 'Urgent', 4, '#DC2626', TRUE, NOW(), NOW())
ON CONFLICT (id) DO UPDATE
SET
    name = EXCLUDED.name,
    level = EXCLUDED.level,
    color = EXCLUDED.color,
    is_default = EXCLUDED.is_default,
    updated_at = NOW();
