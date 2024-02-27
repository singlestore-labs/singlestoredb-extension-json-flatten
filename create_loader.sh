MODULE=json_flatten

usage()
{
    cat <<EOF
$0 TARGET ARTIFACT

TARGET    [ debug | release ]
ARTIFACT  [ standalone | extension_embed | extension ]

EOF
}

[ $# -ne 2 ] && usage

TARGET="$1"
case "$TARGET" in
    debug)   ;;
    release) ;;
    *)       usage ;;
esac

ARTIFACT="$2"
case "$ARTIFACT" in
    standalone)      ;;
    extension_embed) ;;
    extension)       ;;
    *)               usage ;;
esac

WASM_PATH=target/wasm32-wasi/$TARGET/$MODULE.wasm
WIT_PATH=$MODULE.wit
EXT_PATH=target/$MODULE.tar

WASM_B64=$(base64 -w 0 "${WASM_PATH}")
WIT_B64=$(base64 -w 0 "${WIT_PATH}")

OUTFILE_STANDALONE=load_standalone.sql
OUTFILE_EXTENSION_EMBED=$MODULE.sql
OUTFILE_EXTENSION=load_extension.sql

emit_extension_stmts()
{
    EXT_B64=$(base64 -w 0 "${EXT_PATH}")
    cat <<EOF >> $OUTFILE_EXTENSION
DROP EXTENSION IF EXISTS $MODULE;
CREATE EXTENSION $MODULE FROM BASE64 '$EXT_B64';
EOF
}

emit_function_stmts()
{
    case "$ARTIFACT" in
        standalone)
            MAYBE_REPLACE="OR REPLACE"
            CONTENT_SRC="BASE64 '$WASM_B64'"
            OUTFILE="$OUTFILE_STANDALONE"
            ;;
        extension_embed)
            MAYBE_REPLACE=""
            CONTENT_SRC="LOCAL INFILE '`basename $WASM_PATH`'"
            OUTFILE="$OUTFILE_EXTENSION_EMBED"
            ;;
        *)
            usage
            ;;
    esac

    cat <<EOF > "$OUTFILE"
CREATE $MAYBE_REPLACE FUNCTION json_flatten_json(
    json JSON COLLATE utf8mb4_general_ci NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
AS WASM FROM $CONTENT_SRC;

CREATE $MAYBE_REPLACE FUNCTION json_flatten_bigint(
    json JSON COLLATE utf8mb4_general_ci NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value BIGINT NOT NULL)
AS WASM FROM $CONTENT_SRC;

CREATE $MAYBE_REPLACE FUNCTION json_flatten_double(
    json JSON COLLATE utf8mb4_general_ci NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value DOUBLE NOT NULL)
AS WASM FROM $CONTENT_SRC;

CREATE $MAYBE_REPLACE FUNCTION json_flatten_string(
    json JSON COLLATE utf8mb4_general_ci NOT NULL,
    expr ARRAY(TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL))
RETURNS TABLE(
    name TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    value LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
AS WASM FROM $CONTENT_SRC;
EOF
}

if [ "$ARTIFACT" = "extension" ] ; then
    emit_extension_stmts
else
    emit_function_stmts
fi

echo "Loader '$ARTIFACT' created successfully ($TARGET)."

