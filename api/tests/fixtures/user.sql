WITH inserted_user AS (
    INSERT INTO app_user (name)
        VALUES ('Matt')
        RETURNING id
)
INSERT INTO auth_user (username, password_hash, app_user)
    VALUES (
        'matt42',
        '$argon2id$v=19$m=15000,t=2,p=1$MTH7xNfvwRljrZSYdfunsA$fLlixnzNI8yiggfZskODRSzRGVTX+XTVId6PFANd2Uw',
        (SELECT id FROM inserted_user)
    );
