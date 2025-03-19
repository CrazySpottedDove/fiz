#!/bin/bash
if [ -z "$1" ]; then
  echo "请输入提交信息!"
  exit 1
fi
git add .
git commit -m "$1"
git push origin dev
git checkout release
git merge dev
git push origin release
git checkout dev
jq '
  .version |= (
    split(".") as $v
    | ($v[0] | tonumber) as $major
    | ($v[1] | tonumber) as $minor
    | ($v[2] | tonumber) as $patch
    | if $patch == 9 then
        if $minor == 9 then
          "\(( $major + 1 )).0.0"
        else
          "\($major).\(( $minor + 1 )).0"
        end
      else
        "\($major).\($minor).\(( $patch + 1 ))"
      end
  )
' ./src-tauri/tauri.conf.json > tmp.$$.json && mv tmp.$$.json ./src-tauri/tauri.conf.json