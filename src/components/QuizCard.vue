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
            <div class="description mt-2">请及时完成测试。</div>
            <div class="active:text-blue-600 dark:text-cyan-600 cursor-pointer hover:text-cyan-400 min-w-9"
                @click="submit(homework.id)"><a :href="quiz.url" target="_blank"
                    >点击前往学在浙大进行测试</a></div>
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