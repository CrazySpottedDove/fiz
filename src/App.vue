<!-- filepath: /home/dove/CrazySpottedDove/fiz/src/App.vue -->
<script setup>
import { onMounted, ref, provide } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { listen } from '@tauri-apps/api/event';
import { useMaterialStore, useCourseStore, useGradeStore, useConfigStore } from './stores';
const courseStore = useCourseStore();
const gradeStore = useGradeStore();
const materialStore = useMaterialStore();
const configStore = useConfigStore();
listen("courses-inited", (event) => {
    courseStore.courses = event.payload;
});
listen("grades-and-analysis-inited", (event) => {
    [gradeStore.grades, gradeStore.analysis] = event.payload;
});
listen("materials-inited", (event) => {
    materialStore.materials = event.payload;
});
listen("config-inited", (event) => {
    configStore.config = event.payload;
});
listen("login-success", async (event) => {
    const [coursesResult, gradesResult] = await Promise.allSettled([
        invoke("get_courses"),
        invoke("get_grades_and_analysis")
    ]);

    if (coursesResult.status === "fulfilled") {
        courseStore.courses = coursesResult.value;
        try {
            const materials = await invoke("get_materials");
            materialStore.materials = materials;
        } catch (error) {
            window.alert(`获取课件失败：${error}`);
        }
    } else {
        window.alert(`获取课程失败：${coursesResult.reason}`);
    }

    if (gradesResult.status === "fulfilled") {
        const [grades, analysis] = gradesResult.value;
        gradeStore.grades = grades;
        gradeStore.analysis = analysis;
    } else {
        window.alert(`获取成绩信息失败：${gradesResult.reason}`);
    }
});
</script>

<template>
    <router-view></router-view>
</template>