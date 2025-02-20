<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                {{ materialName }}
            </h1>
        </div>
        <br>
        <UploadsCard :uploads="uploads" :title="title" />
    </li>
    <br>
</template>

<script setup>
import UploadsCard from './UploadsCard.vue';
const props = defineProps({
    materialName: String,
    uploads: Array,
    title:String,
});
</script>
