set -ex


# Bash script because I'm lazy to want to create the test files using rust
# TODO: Use the tempfile/ tempdir create instead



SANDBOX_DIR="/tmp/sandbox"

mkdir -p "$SANDBOX_DIR"
cd "$SANDBOX_DIR"




TEST_FILE="$SANDBOX_DIR/test_file.txt"
TEST_FOLDER="$SANDBOX_DIR/tmp/test_folder"



echo `date` >> "$TEST_FILE"
mkdir -p "$TEST_FOLDER"
echo `date` >> "$TEST_FOLDER/test_relative.txt"



# Test 0 : Readlink against a file

# Test 1 : Symlink to a file (absolute path)
ln -sfn `realpath $TEST_FILE` "symlink_file_absolute"


# Test 3.1 : Symlink to a file (relative path)
ln -sfn "test_relative.txt" "$TEST_FOLDER/test_relative.sym.relative_1"

# Test 3.2 : Symlink to a file (relative path)
ln -sfn "./test_relative.txt" "$TEST_FOLDER/test_relative.sym.relative_2"


# Test 4 : Symlink to a symlink (relative path)
ln -sfn "$TEST_FOLDER/test_relative.txt" symlink_jump_1_rel_a
ln -sfn "./symlink_jump_1_rel_a" symlink_jump_2_rel_a
ln -sfn "./symlink_jump_2_rel_a" symlink_jump_3_rel_a


# Test 5 : Symlink to a symlink (absolute path)
ln -sfn `realpath "$TEST_FOLDER/test_relative.txt"` symlink_jump_1_rel_b
ln -sfn "./symlink_jump_1_rel_b" symlink_jump_2_rel_b
ln -sfn symlink_jump_2_rel_b symlink_jump_3_rel_b


# Test 6: Test against circular symlinks
ln -sfn circular_b circular_a
ln -sfn circular_c circular_b
ln -sfn circular_a circular_c


# Test 7: Test against root: Dangerous to accidentally use override flag
cd /
touch "/test_root_file"
ln -sn /test_root_file /test_root_symlink_abs
ln -sn test_root_file /test_root_symlink_rel