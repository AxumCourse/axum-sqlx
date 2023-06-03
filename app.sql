CREATE TABLE member (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(30) NOT NULL COMMENT '会员名称',
    dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '加入时间',
    balance INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '账户余额',
    types TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '会员类型',
    is_del BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否删除',
    UNIQUE(name)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;


-- 初始数据

INSERT INTO member (name, balance, types) VALUES
    ('axum.rs', 12345, 3),
    ('张三', 33333, 0),
    ('李四', 44444, 2),
    ('王五', 55555, 1),
    ('赵六', 66666, 3);