CREATE  FUNCTION json_flatten_json(
    json JSON NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
AS WASM FROM LOCAL INFILE 'json_flatten.wasm';

CREATE  FUNCTION json_flatten_bigint(
    json JSON NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value BIGINT NOT NULL)
AS WASM FROM LOCAL INFILE 'json_flatten.wasm';

CREATE  FUNCTION json_flatten_double(
    json JSON NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value DOUBLE NOT NULL)
AS WASM FROM LOCAL INFILE 'json_flatten.wasm';

CREATE  FUNCTION json_flatten_string(
    json JSON NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
AS WASM FROM LOCAL INFILE 'json_flatten.wasm';
