<template>
    <Card>
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold dark:text-cyan-600 text-zinc-700 [word-break:break-word]">
                {{ materialName }}
            </h1>
        </div>
        <br>
        <UploadsCard :uploads="uploads" :title="title" />
    </Card>
    <br>
</template>

<script setup>
import UploadsCard from './UploadsCard.vue';
import Card from './Card.vue';
const props = defineProps({
    materialName: String,
    uploads: Array,
    title:String,
});
</script>
