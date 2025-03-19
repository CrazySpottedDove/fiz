<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800 relative z-10">
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold [word-break:break-word]"
                :class="homework.submitted ? 'dark:text-green-400' : 'dark:text-yellow-400'">
                {{ `${homework.course} : ${homework.title}` }}
            </h1>
            <p>{{ homework.ddl }}</p>
        </div>
        <div class="flex justify-between items-center w-full">
            <div v-html="homework.description" class="description mt-2"></div>
            <div class="active:text-blue-600 dark:text-cyan-600 cursor-pointer hover:text-cyan-400 min-w-9"
                @click="submit(homework.id)">提交</div>
        </div>
        <br>
        <UploadsCard :uploads="homework.uploads" :title="homework.course" />
    </li>
    <div v-if="submitting" class="fixed inset-0 z-50 dark:bg-zinc-900 bg-opacity-80 flex flex-col">
        <div class="flex-1 flex justify-center items-center">
            <p class="text-white font-bold text-xl">上传中...</p>
        </div>
    </div>
    <br>
</template>

<script setup>
import { ref } from 'vue';
import UploadsCard from './UploadsCard.vue';
import { invoke } from '@tauri-apps/api/core';
import Swal from 'sweetalert2';

const props = defineProps({
    homework: Object,
});

const submitting = ref(false);
function submit(id) {
    const queryFilePromise = invoke('query_file')
    queryFilePromise.then((file_path) => {
        if (file_path === null) {
            return;
        }
        const submitFilePromise = invoke('submit_file', { file_path: file_path });
        const comment = window.prompt("请输入备注(可不输入)") || "";
        submitFilePromise.then((file_id) => {
            submitting.value = true;
            const submitPromise = invoke('submit_homework', { homework_id: id, file_id: file_id, comment: comment });
            submitPromise.then(() => {
                submitting.value = false;
                Swal.fire({
                    icon: 'success',
                    title: '提交成功',
                    timer: 1500,
                })
                props.homework.submitted = true;

            }).catch((e) => {
                submitting.value = false;
                Swal.fire({
                    icon: 'error',
                    title: '提交失败',
                    timer: 2000,
                    text: e,
                })
            })
        }).catch((e) => {
            submitting.value = false;
            Swal.fire({
                icon: 'error',
                title: '上传文件失败',
                timer: 2000,
                text: e,
            })
        })

    }).catch((e) => {
        Swal.fire({
            icon: 'error',
            title: '获取文件路径失败',
            timer: 2000,
            text: e,
        })
    })
}

</script>
<style scoped>
.description * {
    color: inherit !important;
    font-size: inherit !important;
    word-break: break-word;
}
</style>