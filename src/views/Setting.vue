<script setup>
import { invoke } from '@tauri-apps/api/core';
import Layout from '../components/Layout.vue';
import { useConfigStore } from '../stores';
import { ref } from 'vue';

const configStore = useConfigStore();
// 初始化一个局部 reactive 对象，用于双向绑定编辑配置
const config = ref({
    courseware_dir: configStore.config.courseware_dir,
    exp: configStore.config.exp,
    accept_mp4: configStore.config.accept_mp4
});

// 保存配置到 store
function saveConfig() {
    // 复制配置到全局 store 中
    configStore.config = { ...config.value };
    invoke('update_config', { courseware_dir: configStore.config.courseware_dir, exp: configStore.config.exp, accept_mp4: configStore.config.accept_mp4 });
    alert('配置已保存');
}
</script>

<template>
    <Layout>
        <div class="text-center mt-4 fixed left-12">
            <button class="px-4 py-2 bg-cyan-700 text-zinc-200 font-bold rounded hover:bg-blue-700 w-20"
                @click="saveConfig">
                保存配置
            </button>
        </div>
        <form class="p-8 space-y-6">
            <div class="flex">
                <label for="courseware_dir" class="block text-xl font-bold">
                    课件目录：
                </label>
                &nbsp;
                <input id="courseware_dir" type="text" v-model="config.courseware_dir"
                    class="mt-1 block w-1/2 rounded-md focus:border-indigo-500 focus:ring-indigo-500 dark:bg-slate-700 text-center" />
            </div>
            <div>
                <label class="inline-flex items-center">
                    <input type="checkbox" v-model="config.exp"
                        class="rounded border-gray-300 text-indigo-600  focus:ring-indigo-500 scale-150" />
                    <span class="ml-2 text-xl font-bold">开启实验模式</span>
                </label>
            </div>
            <div>
                <label class="inline-flex items-center">
                    <input type="checkbox" v-model="config.accept_mp4"
                        class="rounded border-gray-300 text-indigo-600  focus:ring-indigo-500 scale-150" />
                    <span class="ml-2 text-xl font-bold">批量下载时包括 mp4 文件</span>
                </label>
            </div>
        </form>
    </Layout>
</template>