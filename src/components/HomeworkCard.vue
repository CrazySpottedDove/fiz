<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold [word-break:break-word]"
                :class="homework.submitted ? 'dark:text-green-400' : 'dark:text-yellow-400'">
                {{ `${homework.course} : ${homework.title}` }}
            </h1>
            <p>{{ homework.ddl }}</p>
        </div>
        <div v-html="homework.description" class="description mt-2"></div>
        <br>
        <UploadsCard :uploads="homework.uploads" :title="homework.course" />
    </li>
    <br>
</template>

<script setup>
import UploadsCard from './UploadsCard.vue';
const props = defineProps({
    homework: Object,
});


</script>
<style scoped>
.description * {
    color:inherit !important;
    font-size:inherit !important;
}
</style>