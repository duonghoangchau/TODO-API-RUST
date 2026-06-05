INSERT INTO todo_categories (id, name, slug, color, is_default, created_at, updated_at)
VALUES
    (1, 'Cong viec', 'work', '#3B82F6', TRUE, NOW(), NOW()),
    (2, 'Ca nhan', 'personal', '#22C55E', TRUE, NOW(), NOW()),
    (3, 'Hoc tap', 'study', '#A855F7', TRUE, NOW(), NOW()),
    (4, 'Khac', 'other', '#6B7280', TRUE, NOW(), NOW())
ON CONFLICT (id) DO UPDATE
SET
    name = EXCLUDED.name,
    slug = EXCLUDED.slug,
    color = EXCLUDED.color,
    is_default = EXCLUDED.is_default,
    updated_at = NOW();
