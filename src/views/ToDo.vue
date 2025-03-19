<template>
    <Layout>
        <div class="text-center mt-4 fixed left-12">
            <button class="px-4 py-2 bg-cyan-700 text-zinc-200 font-bold rounded hover:bg-blue-700 w-20"
                @click="changeShowFinishedTask">
                {{ configStore.config.show_finished_task ? 'Less' : 'More' }}
            </button>
        </div>
        <HomeworkCard v-for="homework in filteredHomeworks" :homework="homework" :key="homework.id" class="relative z-10"/>
    </Layout>
</template>
<script setup>
import { emit } from "@tauri-apps/api/event";
import HomeworkCard from "../components/HomeworkCard.vue";
import Layout from "../components/Layout.vue";
import { useConfigStore, useHomeworkStore } from "../stores";
import { computed } from "vue";

const homeworkStore = useHomeworkStore();
const configStore = useConfigStore();
function changeShowFinishedTask() {
    configStore.config.show_finished_task = !configStore.config.show_finished_task;
    emit('update_config', configStore.config);
}
const filteredHomeworks = computed(() => {
    return homeworkStore.homeworks.filter(homework => configStore.config.show_finished_task || !homework.submitted);
});
</script>