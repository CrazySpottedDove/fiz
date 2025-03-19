<script setup>
import { listen } from '@tauri-apps/api/event';
import { useMaterialStore, useCourseStore, useGradeStore, useConfigStore, useHomeworkStore, useStateStore } from './stores';
import { ref, watch } from 'vue';
import { refresh } from './logic/refresh.js';
const courseStore = useCourseStore();
const gradeStore = useGradeStore();
const materialStore = useMaterialStore();
const configStore = useConfigStore();
const homeworkStore = useHomeworkStore();
const stateStore = useStateStore();
const timerId = ref(null);
listen("courses-inited", (event) => {
    courseStore.setCourses(event.payload);
});
listen("grades-and-analysis-inited", (event) => {
    gradeStore.setGrades(event.payload[0]);
    gradeStore.setAnalysis(event.payload[1]);
});
listen("materials-inited", (event) => {
    materialStore.setMaterials(event.payload);
});
listen("config-inited", (event) => {
    configStore.setConfig(event.payload);
});
listen("homeworks-inited", (event) => {
    homeworkStore.setHomeworks(event.payload);
});
listen("watches-inited", (event) => {
    courseStore.setWatches(event.payload);
})

listen("login-success", (event) => {
    refresh();
    stateStore.setLogin(true);
});
watch(
    () => stateStore.login,
    (newVal) => {
        // 如果从未登录变为已登录，开始计时
        if (newVal) {
            timerId.value = setInterval(() => {
                refresh();
            }, 5 * 60000);
        }
        // 如果从已登录变为未登录，清除定时器
        else {
            if (timerId.value) {
                clearTimeout(timerId.value);
                timerId.value = null;
            }
        }
    }
);
</script>

<template>
    <router-view></router-view>
</template>