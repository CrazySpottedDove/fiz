/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");
module.exports = {
    content: ["./src/**/*.{js,ts,jsx,tsx,vue}"],
    darkMode: "class",

    theme: {
        extend: {
            fontFamily: {
                sans: ["InterVariable", "Inter", ...defaultTheme.fontFamily.sans],
                mono: ["Fira Code", ...defaultTheme.fontFamily.mono], // 添加 FiraCode
            },
            typography: (theme) => ({
                DEFAULT: {
                    css: {
                        fontFamily: theme("fontFamily.mono"), // 设置所有文本使用 Fira Code
                        "code::before": {
                            content: '""',
                        },
                        "code::after": {
                            content: '""',
                        },
                        // 为 inline code 增加左右间距
                        ":not(pre) > code": {
                            backgroundColor: theme("colors.neutral.200"),
                            border: "1px solid",
                            borderColor: theme("colors.zinc.300"),
                            padding: "0.25rem 0.4rem",
                            borderRadius: "0.25rem",
                            fontWeight: "400",
                            marginLeft: "0.5rem",
                            marginRight: "0.5rem",
                        },
                        // 移除 blockquote 自动添加的引号
                        "blockquote p:first-of-type::before": {
                            content: "none",
                        },
                        "blockquote p:last-of-type::after": {
                            content: "none",
                        },
                        img: {
                            margin: "2rem auto",       // 上下外边距
                            maxWidth: "100%",           // 最大宽度为容器宽度
                            borderRadius: "0.5rem",     // 圆角
                        },
                    },
                },
                invert: {
                    css: {
                        fontFamily: theme("fontFamily.mono"), // 设置所有文本使用 Fira Code
                        ":not(pre) > code": {
                            backgroundColor: theme("colors.neutral.800"),
                            borderColor: theme("colors.zinc.700"),
                            marginLeft: "0.5rem",
                            marginRight: "0.5rem",
                        },
                        "blockquote p:first-of-type::before": {
                            content: "none",
                        },
                        "blockquote p:last-of-type::after": {
                            content: "none",
                        },
                        img: {
                            margin: "2rem auto",       // 上下外边距
                            maxWidth: "100%",           // 最大宽度为容器宽度
                            borderRadius: "0.5rem",     // 圆角
                        },
                    },
                },
            }),
        },
    },
    plugins: [require("@tailwindcss/typography")],
};