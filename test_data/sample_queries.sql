-- Sample queries for manual testing with isql
-- Run these after connecting: isql -3 test_connection -v

-- Basic table listing (should work with current driver)
help;

-- Simple data queries (may not work yet - depends on driver implementation)
SELECT COUNT(*)
FROM users;
SELECT username, email
FROM users;
SELECT name, price
FROM products
WHERE price > 50;

-- INSERT statements for testing data modification
INSERT INTO users (username, email) VALUES ('johndoe', 'john.doe@example.com');
INSERT INTO users (username, email) VALUES ('janedoe', 'jane.doe@example.com');
INSERT INTO products (name, price) VALUES ('Wireless Mouse', 29.99);
INSERT INTO products (name, price) VALUES ('Mechanical Keyboard', 89.99);
INSERT INTO orders (user_id, total_amount) VALUES (1, 119.98);

-- UPDATE statements for testing data modification
UPDATE products SET price = 24.99 WHERE name = 'Wireless Mouse';
UPDATE users SET email = 'john.smith@example.com' WHERE username = 'johndoe';
UPDATE orders SET total_amount = 114.98 WHERE user_id = 1;

-- DELETE statements for testing data removal
DELETE FROM orders WHERE total_amount < 50;
DELETE FROM products WHERE price < 20;

-- Join query for advanced testing
SELECT u.username, COUNT(o.order_id) AS order_count
FROM users u
       LEFT JOIN orders o ON u.user_id = o.user_id
GROUP BY u.username;

-- View query
SELECT *
FROM user_order_summary;
