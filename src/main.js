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
            // 安装更新
            let downloaded = 0;
            let contentLength = 0;
            // alternatively we could also call update.download() and update.install() separately
            await update.downloadAndInstall((event) => {
                switch (event.event) {
                    case 'Started':
                        contentLength = event.data.contentLength;
                        console.log(`started downloading ${event.data.contentLength} bytes`);
                        break;
                    case 'Progress':
                        downloaded += event.data.chunkLength;
                        console.log(`downloaded ${downloaded} from ${contentLength}`);
                        break;
                    case 'Finished':
                        console.log('download finished');
                        break;
                }
            });
            await relaunch();
        }
    }else{
        console.log("没有发现新版本");
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