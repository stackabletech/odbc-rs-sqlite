-- Test database schema for ODBC driver testing
-- Creates standardized tables with various data types for comprehensive testing

-- Users table - Basic identity and string handling
CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL,
    full_name VARCHAR(100),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT 1,
    age INTEGER,
    balance DECIMAL(10,2) DEFAULT 0.00
);

-- Products table - Catalog data with various numeric types
CREATE TABLE IF NOT EXISTS products (
    product_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    category VARCHAR(50),
    stock_quantity INTEGER DEFAULT 0,
    weight_kg REAL,
    is_available BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Orders table - Relational data for join testing
CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    order_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    shipping_address TEXT,
    notes TEXT,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Order items table - Many-to-many relationship testing
CREATE TABLE IF NOT EXISTS order_items (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price DECIMAL(10,2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id)
);

-- Test data for basic functionality
INSERT OR IGNORE INTO users (user_id, username, email, full_name, age, balance) VALUES
(1, 'alice', 'alice@example.com', 'Alice Johnson', 28, 1250.50),
(2, 'bob', 'bob@example.com', 'Bob Smith', 35, 875.25),
(3, 'carol', 'carol@example.com', 'Carol Davis', 42, 2100.75);

INSERT OR IGNORE INTO products (product_id, name, description, price, category, stock_quantity, weight_kg) VALUES
(1, 'Widget A', 'High-quality widget for general use', 29.99, 'widgets', 100, 0.5),
(2, 'Gadget B', 'Advanced gadget with multiple features', 149.99, 'gadgets', 25, 1.2),
(3, 'Tool C', 'Professional-grade tool', 89.50, 'tools', 50, 2.1);

INSERT OR IGNORE INTO orders (order_id, user_id, total_amount, status, shipping_address) VALUES
(1, 1, 179.98, 'completed', '123 Main St, Anytown, USA'),
(2, 2, 89.50, 'pending', '456 Oak Ave, Somewhere, USA'),
(3, 1, 29.99, 'shipped', '123 Main St, Anytown, USA');

INSERT OR IGNORE INTO order_items (order_id, product_id, quantity, unit_price) VALUES
(1, 1, 1, 29.99),
(1, 2, 1, 149.99),
(2, 3, 1, 89.50),
(3, 1, 1, 29.99);

-- View for testing complex queries
CREATE VIEW IF NOT EXISTS user_order_summary AS
SELECT 
    u.user_id,
    u.username,
    u.full_name,
    COUNT(o.order_id) as total_orders,
    COALESCE(SUM(o.total_amount), 0) as total_spent
FROM users u
LEFT JOIN orders o ON u.user_id = o.user_id
GROUP BY u.user_id, u.username, u.full_name;

-- Create indexes for performance testing
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_date ON orders(order_date);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);