import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from 'pinia';
import { invoke } from "@tauri-apps/api/core";
import "./styles/tailwind.css";
import { check} from '@tauri-apps/plugin-updater'
import Swal from "sweetalert2";
import { relaunch } from "@tauri-apps/plugin-process";
async function handleUpdate() {
    const update = await check();
    if (update) {
        const result = await Swal.fire({
            title: `发现新版本: ${update.version}`,
            icon: 'info',
            showCancelButton: true,
            confirmButtonText: '更新',
            cancelButtonText: '取消'
        });

        if (result.isConfirmed) {
            Swal.fire({
                title: "正在下载更新...",
                html: "请稍候...",
                allowOutsideClick: false,
                didOpen: () => {
                    Swal.showLoading();
                }
            });
            await update.downloadAndInstall();
            await relaunch();
        }
    }
}

async function initializeApp() {
    try {
        handleUpdate();
        await invoke('check_dir');
        invoke('init_config').catch(err => window.alert(`初始化配置失败：${err}`));
        const account_ready = await invoke('check_account');
        if (account_ready) {
            router.push('/todo');
            invoke('init_homeworks').catch(err => window.alert(`初始化作业失败：${err}`));
            invoke('init_courses').catch(err => window.alert(`初始化课程失败：${err}`));
            invoke('init_grades_and_analysis').catch(err => window.alert(`初始化成绩和分析失败：${err}`));
            invoke('login').catch(err => window.alert(`登录失败：${err}`));
        } else {
            router.push('/login');
        }
    } catch (error) {
        window.alert(`初始化失败：${error}`);
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