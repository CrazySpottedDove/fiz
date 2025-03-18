#!/bin/bash
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
        # patch == 9，需要进位
        if $minor == 9 then
          # minor 也 == 9，需要再向前进位
          "\(( $major + 1 )).0.0"
        else
          "\($major).\(( $minor + 1 )).0"
        end
      else
        # 否则只给 patch + 1
        "\($major).\($minor).\(( $patch + 1 ))"
      end
  )
' ./src-tauri/tauri.conf.json > tmp.$$.json && mv tmp.$$.json ./src-tauri/tauri.conf.json