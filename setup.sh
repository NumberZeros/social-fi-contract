#!/bin/bash

# Setup script for Social Fi Contract development

set -e

echo "🚀 Setting up Social Fi Contract..."

# Check prerequisites
echo "✓ Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Install from https://www.rust-lang.org/tools/install"
    exit 1
fi

if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Install from https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Install from https://www.anchor-lang.com/docs/installation"
    exit 1
fi

if ! command -v pnpm &> /dev/null; then
    echo "❌ pnpm not found. Install from https://pnpm.io/installation"
    exit 1
fi

echo "✅ All prerequisites installed"

# Install dependencies
echo "📦 Installing dependencies..."
pnpm install

# Create .env file
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    cp .env.example .env
    echo "⚠️  Please update .env with your configuration"
fi

# Build the contract
echo "🔨 Building contract..."
anchor build

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Update .env with your configuration"
echo "2. Start local validator: solana-test-validator"
echo "3. Run tests: pnpm test"
echo "4. Deploy: anchor deploy"
echo ""
