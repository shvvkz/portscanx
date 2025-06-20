REPO = shvvkz/portscanx
BINARY_NAME = portscanx
INSTALL_DIR = /usr/local/bin
ARCH = x86_64
VERSION := $(shell grep '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
SHELL := /bin/bash
.ONESHELL:
.SHELLFLAGS = -eu -o pipefail -c

.PHONY: release install uninstall clean help

release:
	@echo "🔖 Creating release for version v$(VERSION)..."
	git add Cargo.toml
	git commit -m "🔖 Bump version to v$(VERSION)" || echo "✅ Version already committed."
	git tag v$(VERSION)
	git push origin main
	git push origin v$(VERSION)
	@echo "🚀 Release v$(VERSION) pushed!"

install:
	@echo "📦 Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	sudo -v
	@echo "[INFO] sudo validated"
	LATEST_VERSION=$$(curl -s https://api.github.com/repos/$(REPO)/releases/latest | grep tag_name | cut -d '"' -f 4)
	@echo "[INFO] Latest version: $$LATEST_VERSION"
	if [ -z "$$LATEST_VERSION" ]; then
		echo "❌ Failed to fetch latest version."; exit 1
	fi
	echo "⬇️ Downloading version $$LATEST_VERSION..."
	curl -L -o /tmp/$(BINARY_NAME) https://github.com/$(REPO)/releases/download/$$LATEST_VERSION/$(BINARY_NAME)
	sudo mv /tmp/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	sudo chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	echo "✅ $(BINARY_NAME) installed to $(INSTALL_DIR)"

uninstall:
	@echo "🗑️ Uninstalling $(BINARY_NAME)..."
	if [ -f $(INSTALL_DIR)/$(BINARY_NAME) ]; then \
		sudo rm $(INSTALL_DIR)/$(BINARY_NAME); \
		echo "✅ Uninstalled from $(INSTALL_DIR)"; \
	else \
		echo "❌ $(BINARY_NAME) not found in $(INSTALL_DIR)"; \
	fi

clean:
	@echo "🧹 Cleaning project..."
	cargo clean

help:
	@echo "📖 Available commands:"
	@echo "  make install     - Install the latest release"
	@echo "  make release     - Commit, tag, and push a new release based on Cargo.toml"
	@echo "  make uninstall   - Remove the installed binary"
	@echo "  make clean       - Clean the build artifacts"