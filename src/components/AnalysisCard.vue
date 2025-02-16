<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <div class="flex justify-between items-center w-full">
            <!-- 左侧：名称和描述 -->
            <div class="flex flex-col flex-1 pr-4">
                <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                    {{ name }}
                </h1>
            </div>
            <div class="flex flex-col flex-2 pr-4">
                <h1 :class="gpaClass" class="text-xl font-bold [word-break:break-word]">
                    {{ `${gpa.toFixed(2)} / ${credit}` }}
                </h1>
            </div>
        </div>
    </li>
</template>
<script setup>
import { computed } from 'vue';
const props = defineProps({
    name: String,
    gpa: Number,
    credit: Number,
});
const gpaClass = computed(() => {
    if (props.gpa >= 4.5) {
        return 'dark:text-green-500';
    } else if (props.gpa >= 3.5) {
        return 'dark:text-cyan-500';
    } else if (props.gpa >= 2.4) {
        return 'dark:text-yellow-500';
    } else {
        return 'dark:text-red-500';
    }
});
</script>
