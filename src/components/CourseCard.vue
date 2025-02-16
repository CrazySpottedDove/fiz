<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <a class="flex justify-between items-center w-full" href="" target="_blank">
            <!-- 左侧：名称和描述 -->
            <div class="flex flex-col flex-1 pr-4">
                <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                    {{ name }}
                </h1>
            </div>
            <!-- 右侧：头像图片 -->
            <div v-if="image"
                class="rounded aspect-square overflow-hidden inline-flex drop-shadow-none lg:drop-shadow-[0px_0px_20px_rgba(83,70,200,0.90)] flex-shrink-0">
                <img :src="image" alt="Image" class="aspect-square w-20 h-20 object-cover rounded-full" />
            </div>
            <div v-else
                class="rounded aspect-square overflow-hidden inline-flex drop-shadow-none lg:drop-shadow-[0px_0px_20px_rgba(83,70,200,0.90)] flex-shrink-0 w-20 h-20">
            </div>
        </a>
    </li>

</template>
<script setup>
import { defineProps } from 'vue';
const props = defineProps({
    name: String,
    image: String,
});

</script>
