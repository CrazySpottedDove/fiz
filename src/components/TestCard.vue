<template>
    <Card>
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
    </Card>
    <br>
</template>
<script setup>
import Card from './Card.vue';

const props = defineProps({
    name: String,
    time: String,
    room: String,
    sit: String,
});
</script>
