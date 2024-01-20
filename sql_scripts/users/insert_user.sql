INSERT INTO USERS (username, email, phone, created_by, updated_by, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id