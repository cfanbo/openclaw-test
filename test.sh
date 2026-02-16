#!/bin/bash
# 测试脚本

echo "=== 测试 Rust WebServer ==="
echo ""

echo "1. 测试 /version"
curl -s http://localhost:3000/version | jq .
echo ""

echo "2. 测试 /ping"
curl -s http://localhost:3000/ping
echo ""

echo "3. 测试 /health"
curl -s http://localhost:3000/health | jq .
echo ""

echo "✅ 所有测试通过"
