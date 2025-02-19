<template>
    <li
        class="dark:text-zinc-400 text-zinc-500 gap-1 p-6 border dark:border-zinc-700/60 rounded-lg flex flex-col bg-opacity-40 dark:bg-opacity-40 bg-zinc-50 dark:bg-zinc-800">
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold [word-break:break-word] text-cyan-600">
                {{ name }}
            </h1>
            <p>{{ time }}</p>
        </div>
        <div class="flex justify-between items-center w-full">
            <div class="mt-2">{{ room }}</div>
            <div>{{ sit }}</div>
        </div>
    </li>
    <br>
</template>
<script setup>
const props = defineProps({
    name: String,
    time: String,
    room: String,
    sit: String,
});
</script>
