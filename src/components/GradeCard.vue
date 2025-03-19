<template>
    <Card>
        <div class="flex justify-between items-center w-full space-x-4">
            <!-- 左侧：名称和描述 -->
            <div class="flex flex-col flex-1 pr-4">
                <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                    {{ name }}
                </h1>
            </div>
            <div class="flex flex-col flex-2 pr-4">
                <h1 :class="gpaClass"
                    class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                    {{ grade }}
                </h1>
            </div>
            <div class="flex flex-col flex-3 pr-4">
                <h1 :class="gpaClass" class="text-xl font-bold [word-break:break-word]">
                    {{ `${gpa.toFixed(1)} / ${credit}` }}
                </h1>
            </div>
        </div>
    </Card>
</template>
<script setup>
import { computed } from 'vue';
import Card from './Card.vue';
const props = defineProps({
    name: String,
    grade: String,
    gpa: Number,
    credit: String,
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
