
CREATE TYPE organization_type AS ENUM ('PUBLIC', 'PRIVATE');

CREATE TABLE organization (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    type organization_type NOT NULL,
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
BEFORE UPDATE ON organization 
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

