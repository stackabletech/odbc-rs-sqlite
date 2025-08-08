-- Sample queries for manual testing with isql
-- Run these after connecting: isql -3 test_connection -v

-- Basic table listing (should work with current driver)
help;

-- Simple data queries (may not work yet - depends on driver implementation)
SELECT COUNT(*) FROM users;
SELECT username, email FROM users;
SELECT name, price FROM products WHERE price > 50;

-- Join query for advanced testing
SELECT u.username, COUNT(o.order_id) as order_count 
FROM users u 
LEFT JOIN orders o ON u.user_id = o.user_id 
GROUP BY u.username;

-- View query
SELECT * FROM user_order_summary;
