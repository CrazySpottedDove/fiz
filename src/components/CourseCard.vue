<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <router-link class="flex justify-between items-center w-full"
            :to="{ name: 'CoursewareDetail', params: { id: id },query:{name:name} }">
            <!-- 左侧：名称和描述 -->
            <div class="flex flex-col flex-1 pr-4">
                <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                    {{ name }}
                </h1>
            </div>
            <p class="text-sm dark:text-zinc-400 text-zinc-600 mt-2">
                {{ time }}
            </p>
        </router-link>
    </li>

</template>
<script setup>
const props = defineProps({
    name: String,
    time: String,
    id: Number
});

</script>
