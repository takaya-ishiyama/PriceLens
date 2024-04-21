
CREATE TYPE organization_type AS ENUM ('PUBLIC', 'PRIVATE');

CREATE TABLE organization (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    type organization_type NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- CREATE TABLE item (
--     id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
--     FOREIGN KEY (organization_id) REFERENCES community(id) NOT NULL,
--     name VARCHAR(50) NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     deleted_at TIMESTAMP
-- );

-- CREATE TABLE price (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     FOREIGN KEY (item_id) REFERENCES item(id) NOT NULL,
--     price INT NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     deleted_at TIMESTAMP
-- );

-- CREATE TABLE private_key (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     FOREIGN KEY (organization_id) REFERENCES organization(id) NOT NULL,
--     key VARCHAR(50) NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     deleted_at TIMESTAMP
-- );

-- CREATE TABLE session (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     FOREIGN KEY (private_key_id) REFERENCES private_key(id) NOT NULL,
--     access_token VARCHAR(255) NOT NULL,
--     expiration_timestamp TIMESTAMP NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     deleted_at TIMESTAMP
-- );



CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_updated_at_column
BEFORE UPDATE ON organization 
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

-- CREATE TRIGGER update_updated_at_column
-- BEFORE UPDATE ON item  
-- FOR EACH ROW
-- EXECUTE PROCEDURE update_updated_at_column();

-- CREATE TRIGGER update_updated_at_column
-- BEFORE UPDATE ON price
-- FOR EACH ROW
-- EXECUTE PROCEDURE update_updated_at_column();


-- CREATE TRIGGER update_updated_at_column
-- BEFORE UPDATE ON private_key 
-- FOR EACH ROW
-- EXECUTE PROCEDURE update_updated_at_column();


-- CREATE TRIGGER update_updated_at_column
-- BEFORE UPDATE ON session
-- FOR EACH ROW
-- EXECUTE PROCEDURE update_updated_at_column();

