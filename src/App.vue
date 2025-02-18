<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useMaterialStore, useCourseStore, useGradeStore, useConfigStore, useHomeworkStore } from './stores';
const courseStore = useCourseStore();
const gradeStore = useGradeStore();
const materialStore = useMaterialStore();
const configStore = useConfigStore();
const homeworkStore = useHomeworkStore();
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
listen("login-success", async (event) => {
    const [coursesResult, gradesResult] = await Promise.allSettled([
        invoke("get_courses"),
        invoke("get_grades_and_analysis")
    ]);

    if (coursesResult.status === "fulfilled") {
        courseStore.setCourses(coursesResult.value);
        try {
            const [materialsResult, homeworksResult] = await Promise.allSettled([
                invoke("get_materials"),
                invoke("get_homeworks")
            ])
            if (materialsResult.status === "fulfilled") {
                materialStore.setMaterials(materialsResult.value);
            } else {
                window.alert(`获取课件失败：${materialsResult.reason}`);
            }
            if (homeworksResult.status === "fulfilled") {
                homeworkStore.setHomeworks(homeworksResult.value);
            } else {
                window.alert(`获取作业失败：${homeworksResult.reason}`);
            }
        } catch (error) {
            console.error(error);
        }
    } else {
        window.alert(`获取课程失败：${coursesResult.reason}`);
    }

    if (gradesResult.status === "fulfilled") {
        const [grades, analysis] = gradesResult.value;
        gradeStore.setGrades(grades);
        gradeStore.setAnalysis(analysis);
    } else {
        window.alert(`获取成绩信息失败：${gradesResult.reason}`);
    }
});
</script>

<template>
    <router-view></router-view>
</template>