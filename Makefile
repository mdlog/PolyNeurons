.PHONY: build test clean deploy run-engine run-validator

# Build all Rust components
build:
	@echo "🔨 Building PolyNeurons..."
	cargo build --release
	@echo "✅ Build complete!"

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test --workspace
	npm run test
	@echo "✅ All tests passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf artifacts cache

# Deploy smart contracts
deploy:
	@echo "🚀 Deploying contracts..."
	npm run compile
	npm run deploy

# Run cognitive engine
run-engine:
	@echo "🧠 Starting Cognitive Engine..."
	@if [ ! -f .env ]; then cp .env.example .env; fi
	@export $$(cat .env | grep -v '^#' | xargs) && cd cognitive-engine && cargo run --release

# Run validator plugin
run-validator:
	@echo "🔌 Starting Validator Plugin..."
	@if [ ! -f .env ]; then cp .env.example .env; fi
	@export $$(cat .env | grep -v '^#' | xargs) && cd validator-plugin && cargo run --release

# Install dependencies
install:
	@echo "📦 Installing dependencies..."
	npm install
	cargo fetch
	@echo "✅ Dependencies installed!"

# Format code
fmt:
	cargo fmt --all
	@echo "✨ Code formatted!"

# Lint code
lint:
	cargo clippy --all-targets --all-features
	@echo "✅ Linting complete!"

# Run full setup
setup: install build
	@echo "🎉 Setup complete! Ready to go!"

# Run examples
register-node:
	@./run-example.sh register_node

submit-task:
	@./run-example.sh submit_task

submit-task-cheap:
	@./run-example.sh submit_task_cheap

view-task:
	@./run-example.sh view_task
