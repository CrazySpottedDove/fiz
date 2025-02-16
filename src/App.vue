<!-- filepath: /home/dove/CrazySpottedDove/fiz/src/App.vue -->
<script setup>
import { onMounted, ref, provide } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { listen } from '@tauri-apps/api/event';
import { useCourseStore, useGradeStore } from './stores';
const courseStore = useCourseStore();
const gradeStore = useGradeStore();
listen("courses-inited", (event) => {
    courseStore.courses = event.payload;
});
listen("grades-and-analysis-inited", (event) => {
    gradeStore.grades = event.payload.grades;
    gradeStore.analysis = event.payload.analysis;
});
listen("login-success", async (event) => {
    try {
        courseStore.courses = await invoke("get_courses");
        [gradeStore.grades, gradeStore.analysis] = await invoke("get_grades_and_analysis");
    }
    catch (e) { window.alert(`获取课程失败：${e}`); }
});
</script>

<template>
    <!-- 你可以显示自检进度动画，或简单空白等待 -->
    <!-- <div>启动中...</div> -->
    <router-view></router-view>
</template>