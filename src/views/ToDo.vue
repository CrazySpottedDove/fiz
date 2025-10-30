<script setup>
import { emit } from "@tauri-apps/api/event";
import HomeworkCard from "../components/HomeworkCard.vue";
import QuizCard from "../components/QuizCard.vue";
import Layout from "../components/Layout.vue";
import { useConfigStore, useHomeworkStore, useQuizStore } from "../stores";
import { computed } from "vue";

const homeworkStore = useHomeworkStore();
const quizStore = useQuizStore();
const configStore = useConfigStore();

function changeShowFinishedTask() {
    configStore.config.show_finished_task = !configStore.config.show_finished_task;
    emit('update_config', configStore.config);
}

// 合并并排序
const mergedTasks = computed(() => {
    // 给每个对象加上类型标记
    const homeworks = homeworkStore.homeworks
        .filter(hw => configStore.config.show_finished_task || !hw.submitted)
        .map(hw => ({ ...hw, _type: 'homework' }));
    const quizes = quizStore.quizes
        .filter(qz => configStore.config.show_finished_task || !qz.submitted)
        .map(qz => ({ ...qz, _type: 'quiz' }));
    // 合并并按ddl排序
    return [...homeworks, ...quizes].sort((a, b) => new Date(a.ddl) - new Date(b.ddl));
});
</script>

<template>
    <Layout>
        <div class="text-center mt-4 fixed left-12">
            <button class="px-4 py-2 bg-cyan-700 text-zinc-200 font-bold rounded hover:bg-blue-700 w-20"
                @click="changeShowFinishedTask">
                {{ configStore.config.show_finished_task ? 'Less' : 'More' }}
            </button>
        </div>
        <template v-for="task in mergedTasks" :key="task.id">
            <HomeworkCard v-if="task._type === 'homework'" :homework="task" />
            <QuizCard v-else :quiz="task" />
        </template>
    </Layout>
</template>