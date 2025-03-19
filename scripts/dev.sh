#!/bin/bash
if [ -z "$1" ]; then
  echo "请输入提交信息!"
  exit 1
fi
git add .
git commit -m "$1"
git push origin dev