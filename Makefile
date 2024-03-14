TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APP_PATHS := $(wildcard $(APP_DIR)/*.rs)
APP_NAMES := $(patsubst $(APP_DIR)/%.rs, %, $(APP_PATHS))
TMP_IMAGE_DIR := $(TARGET_DIR)/images
USER_FS_IMG := user-fs.img

elf:
	@$(foreach app, $(APP_NAMES), export CARGO_BIN_EXE_NAME=$(app) && cargo build --bin $(app) --release;)

collect: elf
	@rm -rf $(TMP_IMAGE_DIR)
	@mkdir -p $(TMP_IMAGE_DIR)
	@$(foreach app, $(APP_NAMES), mv $(TARGET_DIR)/$(app) $(TMP_IMAGE_DIR);)

image: collect
	@cd ../frontier_fs_fuse && cargo run --release -- --source-dir ../frontier_user/$(TMP_IMAGE_DIR) --target-path ../frontier_user/$(TARGET_DIR)/$(USER_FS_IMG) --check

build: image
