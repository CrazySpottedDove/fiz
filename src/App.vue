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
listen("login-success", (event) => {
    const coursesPromise = invoke("get_courses");
    const gradesPromise = invoke("get_grades_and_analysis");
    coursesPromise.then((courses) => {
        courseStore.setCourses(courses);
        const materialsPromise = invoke("get_materials");
        const homeworksPromise = invoke("get_homeworks");
        materialsPromise.then((materials) => {
            materialStore.setMaterials(materials);
        }).catch((error) => {
            window.alert(`获取课件失败：${error}`);
        });
        homeworksPromise.then((homeworks) => {
            homeworkStore.setHomeworks(homeworks);
        }).catch((error) => {
            window.alert(`获取作业失败：${error}`);
        });
    }).catch((error) => {
        window.alert(`获取课程失败：${error}`);
    });
    gradesPromise.then(([grades, analysis]) => {
        gradeStore.setGrades(grades);
        gradeStore.setAnalysis(analysis);
    }).catch((error) => {
        window.alert(`获取成绩信息失败：${error}`);
    });
});

</script>

<template>
    <router-view></router-view>
</template>