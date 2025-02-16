import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from 'pinia';
import { invoke } from "@tauri-apps/api/core";
import "./styles/tailwind.css";

async function initializeApp() {
    try {
        await invoke('check_dir');
        const account_ready = await invoke('check_account');
        if (account_ready) {
            router.push('/todo');
            invoke('init_semesters').catch(err => window.alert(`初始化学期失败：${err}`));
            invoke('init_courses').catch(err => window.alert(`初始化课程失败：${err}`));
            invoke('init_grades_and_analysis').catch(err => window.alert(`初始化成绩和分析失败：${err}`));
            invoke('login').catch(err => window.alert(`登录失败：${err}`));
        } else {
            console.log('账号未登录');
            router.push('/login');
        }
    } catch (error) {
        console.error(`初始化失败：${error}`);
        router.push('/login');
    }
}

// 使用路由守卫控制初始化逻辑
router.beforeEach(async (to, from, next) => {
    if (to.path === "/") {
        await initializeApp();
    }
    next();
});

createApp(App).use(router).use(createPinia()).mount("#app");