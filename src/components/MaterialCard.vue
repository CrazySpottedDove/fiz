<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                {{ materialName }}
            </h1>
        </div>
        <br>
        <UploadsCard :uploads="uploads" :title="materialName" />
    </li>
    <br>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import UploadsCard from './UploadsCard.vue';
const props = defineProps({
    materialName: String,
    uploads: Array,
    title:String,
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
