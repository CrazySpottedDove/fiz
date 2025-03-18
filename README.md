# [fiz(fire in zju)](https://crazyspotteddove.github.io/projects/fiz/)

快速简洁的学在浙大第三方工具。全平台支持。

点击标题访问介绍网页。

## How to Build

```bash
npm install
npm run tauri add dialog updater opener
# 开发
npx tailwindcss -i ./src/styles/global.css -o ./src/styles/tailwind.css --watch
npm run tauri dev
# 构建
npm run tauri build
```
