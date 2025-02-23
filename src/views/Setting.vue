<script setup>
import { invoke } from '@tauri-apps/api/core';
import Layout from '../components/Layout.vue';
import { useConfigStore, useCourseStore } from '../stores';
import { ref, onMounted, watch } from 'vue';
import Swal from 'sweetalert2';

const configStore = useConfigStore();
const config = ref({});
const courseStore = useCourseStore();
// 初始化一个局部 reactive 对象，用于双向绑定编辑配置
onMounted(() => {
    watch(
        () => configStore.config,
        (newConfig) => {
            if (newConfig && Object.keys(newConfig).length > 0) {
                config.value = {
                    courseware_dir: newConfig.courseware_dir,
                    exp: newConfig.exp,
                    accept_mp4: newConfig.accept_mp4,
                    material_rev: newConfig.material_rev,
                };
            }
        },
        { immediate: true }
    );
});

// 保存配置到 store
function saveConfig() {
    // 复制配置到全局 store 中
    configStore.setConfig({ ...config.value });
    invoke('update_config', { courseware_dir: configStore.config.courseware_dir, exp: configStore.config.exp, accept_mp4: configStore.config.accept_mp4, material_rev: configStore.config.material_rev });
    Swal.fire({
        icon: 'success',
        title: '配置已保存',
        timer: 1500,
    })
}
const editWatches = ref(false)

function intoWatches() {
    selectedIds.value = courseStore.watches.map(w => w.id);
    editWatches.value = true;
}

function closeWatches() {
    editWatches.value = false;
}
const selectedIds = ref([]);
function selectWatches() {
    editWatches.value = false;
    const newWatches = courseStore.courses.filter(course => selectedIds.value.includes(course.id)).map(course => ({ id: course.id, name: course.name }));
    courseStore.setWatches(newWatches);
    invoke('set_watches', { watches: newWatches });
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
            <div>
                <label class="inline-flex items-center">
                    <input type="checkbox" v-model="config.material_rev"
                        class="rounded border-gray-300 text-indigo-600  focus:ring-indigo-500 scale-150" />
                    <span class="ml-2 text-xl font-bold">课件倒序排列</span>
                </label>
            </div>
            <ul>
                <p class="text-xl font-bold cursor-pointer hover:text-blue-600 active:text-green-400" @click="intoWatches">监听课程</p>
                <li v-for="watch in courseStore.watches" :key="watch.id" class="text-lg font-bold ml-6 mt-2">
                    {{ watch.name }}
                </li>
            </ul>
        </form>
        <div v-if="editWatches" class="fixed inset-0 z-50 dark:bg-zinc-900 bg-opacity-80 flex flex-col items-center">
            <!-- 顶部文字 -->
            <div class="my-6">
                <p class="text-center text-xl font-bold text-cyan-500">
                    fiz 会自动下载被监听课程的全部课件
                </p>
            </div>

            <!-- 中间可滚动列表，列举所有课程，每个课程携带复选框 -->
            <div class="flex-1 overflow-y-auto w-3/4 py-4 px-6">
                <label v-for="course in courseStore.courses" :key="course.id" class="flex items-center mt-2 mb-2">
                    <input type="checkbox" :value="course.id" v-model="selectedIds" class="mr-2 scale-125" />
                    <span class="text-lg font-bold dark:text-zinc-100">
                        {{ course.name }}
                    </span>
                </label>
            </div>

            <!-- 下方操作按钮 -->
            <div class="my-6">
                <button @click="selectWatches"
                    class="bg-cyan-600 hover:bg-cyan-700 text-white font-bold py-2 px-4 rounded">
                    就监听这些！
                </button>
            </div>

            <!-- 右上角关闭按钮 -->
            <div class="absolute top-4 right-4">
                <button @click="closeWatches" class="text-cyan-500 hover:text-blue-700 text-xl font-bold">
                    关闭
                </button>
            </div>
        </div>
    </Layout>

</template>