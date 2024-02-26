# JSON Flatten

## Overview

This extension provides a set of TVFs that perform flattening operations against JSON objects.

## Contents

### `json_flatten_string`

Usage: `json_flatten_string(input JSON, exprs: ARRAY(TEXT))`

Parameters:
- input:  The source json object.
- exprs:  An array of zero or more JSON path expressions.  If it is empty, then the root is assumed (`"$"`).  Multiple paths may be provided; if so, the results are concatenated.

Returns:  A table with two columns, `name` and `value`.  Each `value` will be converted to a string.  If the value is not actully a JSON string, it will be returned as empty.

Description:  This function will return a table with the names and values of each object immediately under the specified JSON path(s).  Please note that this function will return *child* objects relative to the path expression(s), therefore if an expression does not refer to a JSON object or an array, an empty set will be the result.

Example:
```
select * from json_flatten_string('{"a":"b", "c":"d", "e": "f"}', []);

+------+-------+
| name | value |
+------+-------+
| a    | b     |
| c    | d     |
| e    | f     |
+------+-------+
```

### `json_flatten_bigint`

Usage: `json_flatten_bigint(input JSON, exprs: ARRAY(TEXT))`

Parameters:
- input:  The source json object.
- exprs:  An array of zero or more JSON path expressions.  If it is empty, then the root is assumed (`"$"`).  Multiple paths may be provided; if so, the results are concatenated.

Returns:  A table with two columns, `name` and `value`.  Each `value` will be converted to a `BIGINT`.  If the value is not actully a JSON integer, it will be returned as '0'.

Description:  This function will return a table with the names and values of each object immediately under the specified JSON path(s).  Please note that this function will return *child* objects relative to the path expression(s), therefore if an expression does not refer to a JSON object or an array, an empty set will be the result.

Example:
```
select * from json_flatten_bigint('{"a":0, "c":1, "e": 2}', []);

+------+-------+
| name | value |
+------+-------+
| a    |     0 |
| c    |     1 |
| e    |     2 |
+------+-------+
```

### `json_flatten_double`

Usage: `json_flatten_double(input JSON, exprs: ARRAY(TEXT))`

Parameters:
- input:  The source json object.
- exprs:  An array of zero or more JSON path expressions.  If it is empty, then the root is assumed (`"$"`).  Multiple paths may be provided; if so, the results are concatenated.

Returns:  A table with two columns, `name` and `value`.  Each `value` will be converted to a `DOUBLE`.  If the value is not actully a JSON floating-point number, it will be returned as '0'.

Description:  This function will return a table with the names and values of each object immediately under the specified JSON path(s).  Please note that this function will return *child* objects relative to the path expression(s), therefore if an expression does not refer to a JSON object or an array, an empty set will be the result.

Example:
```
select * from json_flatten_double('{"a":1.1, "c":2, "e": 3.0}', []);

+------+-------+
| name | value |
+------+-------+
| a    |   1.1 |
| c    |     2 |
| e    |     3 |
+------+-------+
```

### `json_flatten_json`

Usage: `json_flatten_json(input JSON, exprs: ARRAY(TEXT))`

Parameters:
- input:  The source json object.
- exprs:  An array of zero or more JSON path expressions.  If it is empty, then the root is assumed (`"$"`).  Multiple paths may be provided; if so, the results are concatenated.

Returns:  A table with two columns, `name` and `value`.  Each `value` will be returned as a JSON expression.

Description:  This function will return a table with the names and values of each object immediately under the specified JSON path(s).  Please note that this function will return *child* objects relative to the path expression(s), therefore if an expression does not refer to a JSON object or an array, an empty set will be the result.

Example:
```
select * from json_flatten_json('{"a":1, "c":2.2, "e": "f", "g":{"h":"i", "j":8}}', []);

+------+-----------------+
| name | value           |
+------+-----------------+
| a    | 1               |
| c    | 2.2             |
| e    | "f"             |
| g    | {"h":"i","j":8} |
+------+-----------------+
```

## Deployment to SingleStore

To install these functions using the MySQL CLI, you can use the following command.  Replace '$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.
```bash
mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p < load_standalone.sql
```

## Additional Examples

Here is an example of flattening a JSON array.  The names are simply returned as the index of each item.
```
select * from json_flatten_string('["a", "b", "c"]', []);

+------+-------+
| name | value |
+------+-------+
| 0    | a     |
| 1    | b     |
| 2    | c     |
+------+-------+
```

Here is an example of flattening JSON to strings where not every value is actually a string.  The non-string values are returned empty.
```
select * from json_flatten_string('{"a":9, "b":"c", "d":34.2}', []);

+------+-------+
| name | value |
+------+-------+
| a    |       |
| b    | c     |
| d    |       |
+------+-------+
```

Here is an example of flattening an embedded JSON object using a path.
```
select * from json_flatten_string('{"a":9, "b":{"c":"d", "e":"f"}}', ['$.b']);

+------+-------+
| name | value |
+------+-------+
| c    | d     |
| e    | f     |
+------+-------+
```

Here is an example of passing multiple JSON path expressions.  The results are concatenated.
```
select * from json_flatten_string('{"a":9, "b":{"c":"d", "e":"f"}, "g":{"h":"i", "j":"k"}}', ['$.b', '$.g']);

+------+-------+
| name | value |
+------+-------+
| c    | d     |
| e    | f     |
| h    | i     |
| j    | k     |
+------+-------+
```

Here is an example of passing a JSON path to an element that does not have children.  Empty set is returned.
```
select * from json_flatten_string('{"a":9, "b":"c"}', ['$.a']);
Empty set (0.001 sec)
```

## Building

In order to build the Wasm

1. Install Rust and Cargo
2. Install the Rust `wasm32-wasi` target
3. Run `cargo build --target wasm32-wasi`

To build the extension from the Wasm locally, run the following.

```bash
cp ./target/wasm32-wasi/release/json-flatten.wasm ./json-flatten.wasm
tar cvf json-flatten.tar json-flatten.sql json-flatten.wasm json-flatten.wit 
```

## Testing

There are automated script tests in the `db-tests` directory.
They are run against the `singlestoredb-dev-image`.

In order to run the tests

1. Install Python 3
2. Install `singlestoredb[pytest]` (and optionally `pytest-xdist`) using pip
3. Set the `SINGLESTORE_LICENSE` environment variable
4. Run the `pytest` command

If you installed `pytest-xdist`, you can also distribute the tests to multiple workers
by running `pytest -n auto` or giving a specific number instead of `auto`
