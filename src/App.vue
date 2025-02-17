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
    [gradeStore.grades, gradeStore.analysis] = event.payload;
});
listen("login-success", async (event) => {
    try {
        const courses = await invoke("get_courses");
        courseStore.courses = courses;
    } catch (e) {
        window.alert(`获取课程失败：${e}`);
        // 不覆盖 courseStore.courses，保留原有数据
    }

    try {
        const [grades, analysis] = await invoke("get_grades_and_analysis");
        gradeStore.grades = grades;
        gradeStore.analysis = analysis;
    } catch (e) {
        window.alert(`获取成绩信息失败：${e}`);
        // 不覆盖 gradeStore，保留原有数据
    }
});
</script>

<template>
    <!-- 你可以显示自检进度动画，或简单空白等待 -->
    <!-- <div>启动中...</div> -->
    <router-view></router-view>
</template>