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
    const [coursesResult, gradesResult] = await Promise.allSettled([
        invoke("get_courses"),
        invoke("get_grades_and_analysis")
    ]);

    if (coursesResult.status === "fulfilled") {
        courseStore.courses = coursesResult.value;
    } else {
        window.alert(`获取课程失败：${coursesResult.reason}`);
        // 不覆盖 courseStore.courses，保留原有数据
    }

    if (gradesResult.status === "fulfilled") {
        const [grades, analysis] = gradesResult.value;
        gradeStore.grades = grades;
        gradeStore.analysis = analysis;
    } else {
        window.alert(`获取成绩信息失败：${gradesResult.reason}`);
        // 不覆盖 gradeStore，保留原有数据
    }
});
</script>

<template>
    <router-view></router-view>
</template>