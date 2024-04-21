

CREATE TABLE price (
    id SERIAL PRIMARY KEY,
    item_id uuid REFERENCES item(id) NOT NULL,
    price INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE private_key (
    id SERIAL PRIMARY KEY,
    organization_id uuid REFERENCES organization(id) NOT NULL,
    key VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE session (
    id SERIAL PRIMARY KEY,
    private_key_id INT REFERENCES private_key(id) NOT NULL,
    access_token VARCHAR(255) NOT NULL,
    expiration_timestamp TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);



CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';


CREATE TRIGGER update_updated_at_column
BEFORE UPDATE ON price
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();


CREATE TRIGGER update_updated_at_column
BEFORE UPDATE ON private_key 
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();


CREATE TRIGGER update_updated_at_column
BEFORE UPDATE ON session
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

