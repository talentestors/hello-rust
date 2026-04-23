#!/bin/bash
set -euo pipefail

# 默认仅预览，加 --force 才会真正删除
DRY_RUN=true
[[ "${1:-}" == "--force" ]] && DRY_RUN=false

# 1. 定义要删除的文件扩展名
FILE_EXTENSIONS=( "*.exe" "*.dll" "*.so" "*.dylib" "*.o" "*.a" )

# 2. 构建 find 命令的参数数组（用于匹配文件）
# 注意：find 的 -name 条件需要用 -o 连接，并包裹在 \( \) 中
FIND_FILES_ARGS=( . -type f \( )
for i in "${!FILE_EXTENSIONS[@]}"; do
  FIND_FILES_ARGS+=( -name "${FILE_EXTENSIONS[$i]}" )
  if [[ $i -lt $((${#FILE_EXTENSIONS[@]} - 1)) ]]; then
    FIND_FILES_ARGS+=( -o )
  fi
done
FIND_FILES_ARGS+=( \) )

if $DRY_RUN; then
  echo "当前为【预览模式】，确认无误后请运行: $0 --force"
  
  echo "directories to be deleted:"
  find . -type d -name "target" -print 2>/dev/null || true
  
  echo "files to be deleted:"
  find "${FIND_FILES_ARGS[@]}" -print 2>/dev/null || true
  
else
  # 删除 target 目录
  # -depth 确保先删内容再删目录，避免报错
  find . -type d -name "target" -depth -exec rm -rf {} + 2>/dev/null || true
  
  # 删除编译产物文件
  find "${FIND_FILES_ARGS[@]}" -exec rm -f {} + 2>/dev/null || true
fi