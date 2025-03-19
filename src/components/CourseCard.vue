<template>
    <Card>
        <router-link class="flex justify-between items-center w-full"
            :to="{ name: 'CoursewareDetail', params: { id: id }, query: { name: name } }">
            <!-- 左侧：名称和描述 -->
            <div class="flex flex-col flex-1 pr-4">
                <h1 class="text-xl font-bold text-zinc-700 [word-break:break-word]"
                    :class="watched ? 'dark:text-green-400' : 'dark:text-cyan-600'">
                    {{ name }}
                </h1>
            </div>
            <p class="text-sm dark:text-zinc-400 text-zinc-600 mt-2">
                {{ time }}
            </p>
        </router-link>
    </Card>
</template>
<script setup>
import Card from './Card.vue';
const props = defineProps({
    name: String,
    time: String,
    id: Number,
    watched: Boolean
});

</script>
