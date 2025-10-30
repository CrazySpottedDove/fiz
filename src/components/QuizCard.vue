<template>
    <Card>
        <div class="flex justify-between items-center w-full">
            <h1 class="text-xl font-bold [word-break:break-word]"
                :class="quiz.submitted ? 'dark:text-green-400' : 'dark:text-yellow-400'">
                {{ `${quiz.course} : ${quiz.title}` }}
            </h1>
            <p>{{ quiz.ddl }}</p>
        </div>
        <div class="flex justify-between items-center w-full">
            <div class="description mt-2">请及时在学在浙大进行测试。</div>
        </div>
        <br>
    </Card>
    <br>
</template>

<script setup>
import Card from './Card.vue';

const props = defineProps({
    quiz: Object,
});

</script>
<style scoped>
.description * {
    color: inherit !important;
    font-size: inherit !important;
    word-break: break-word;
}
</style>