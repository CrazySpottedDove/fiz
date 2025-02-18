<template>
    <ul>
        <li v-for="upload in uploads" :key="upload.id"
            class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
            <div class="flex justify-between items-center w-full">
                <h1 class="text-lg font-bold dark:text-blue-300 text-zinc-700 [word-break:break-word]"
                    :class="previewAble(upload.name) ? 'cursor-pointer' : 'cursor-not-allowed'"
                    @click="previewAble(upload.name) && preview(upload.reference_id)">
                    {{ upload.name }}
                </h1>
                <p class="text-xl font-bold active:text-green-400 cursor-pointer hover:text-emerald-600"
                    @click="fetch(upload.reference_id, upload.name)">↓
                </p>
            </div>
        </li>
    </ul>
    <div v-if="loading || showPreview" class="fixed inset-0 z-50 dark:bg-zinc-900 bg-opacity-80 flex flex-col">
        <!-- 顶部关闭条 -->
        <div class="flex justify-end p-4">
            <button @click="closePreview" class="text-cyan-500 hover:text-blue-700 text-xl font-bold">关闭</button>
        </div>
        <!-- 预览内容区域 -->
        <div class="flex-1 flex justify-center items-center">
            <template v-if="loading">
                <div>
                    <p class="text-white font-bold text-xl">加载中...</p>
                    <p class="text-white font-bold text-xl">fiz 预览功能性能较差，建议下载文件到本地/(ㄒoㄒ)/~~</p>
                </div>
            </template>
            <template v-else-if="previewType === 'image'">
                <img :src="previewData" class="w-full h-full object-contain" />
            </template>
            <template v-else-if="previewType === 'pdf'">
                <iframe :src="previewData" class="w-full h-full"></iframe>
            </template>
        </div>
    </div>
</template>
<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
const props = defineProps({
    uploads: Array,
    title: String,
});
const showPreview = ref(false); // 控制模态框显示
const previewData = ref(''); // 预览数据（Base64 或 URL）
const previewType = ref(''); // 文件类型（image/pdf/video）
const loading = ref(false);
const fetch = async (reference_id, name) => {
    try {
        invoke('fetch_upload', { reference_id: reference_id, name: name, title: props.title });
    } catch (e) {
        window.alert(`下载${name}失败：${e}`);
    }
};
function base64ToBlob(base64Data, contentType) {
    const byteCharacters = atob(base64Data);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    return new Blob([byteArray], { type: contentType });
}

async function preview(reference_id) {
    try {
        loading.value = true;
        const [base64Data, contentType] = await invoke('get_preview', { reference_id: reference_id });

        let blob = base64ToBlob(base64Data, contentType);
        let blobUrl = URL.createObjectURL(blob);

        // 根据 content_type 设置预览数据和类型
        if (contentType.startsWith('image/')) {
            previewData.value = blobUrl;
            previewType.value = 'image';
        } else if (contentType === 'application/pdf') {
            previewData.value = blobUrl;
            console.log(previewData.value);
            previewType.value = 'pdf';
        } else {
            console.error('不支持的文件类型:', contentType);
            return;
        }

        // 显示模态框
        showPreview.value = true;
    } catch (e) {
        window.alert('预览失败：' + e);
    } finally {
        loading.value = false;
    }
}

function closePreview() {
    showPreview.value = false;
    previewData.value = '';
    previewType.value = '';
}

const previewAble = (name) => {
    return name.endsWith('.jpg') || name.endsWith('.jpeg') || name.endsWith('.png') || name.endsWith('.doc') || name.endsWith('.pdf') || name.endsWith('docx') || name.endsWith('ppt') || name.endsWith('pptx') || name.endsWith('xls') || name.endsWith('xlsx')
};
</script>