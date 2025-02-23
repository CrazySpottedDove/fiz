<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useMaterialStore, useCourseStore, useGradeStore, useConfigStore, useHomeworkStore, useTestStore, useStateStore } from './stores';
import Swal from 'sweetalert2';
import { ref, watch } from 'vue';
const courseStore = useCourseStore();
const gradeStore = useGradeStore();
const materialStore = useMaterialStore();
const configStore = useConfigStore();
const homeworkStore = useHomeworkStore();
const testStore = useTestStore();
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
listen("watches-inited",(event)=>{
    courseStore.setWatches(event.payload);
})
function refresh() {
    const semesterPromise = invoke("get_semesters");
    const gradesPromise = invoke("get_grades_and_analysis");
    semesterPromise.then(() => {
        const coursesPromise = invoke("get_courses");
        const testsPromise = invoke("get_tests");
        coursesPromise.then((courses) => {
            courseStore.setCourses(courses);
            const materialsPromise = invoke("get_materials");
            const homeworksPromise = invoke("get_homeworks");
            materialsPromise.then((materials) => {
                materialStore.setMaterials(materials);
                const watchesPromise = invoke("get_watches");
                watchesPromise.catch((error) => {
                    Swal.fire({
                        icon: 'error',
                        title: '下载监听课程课件失败',
                        timer: 1500,
                        text: error,
                    });
                });
            }).catch((error) => {
                Swal.fire({
                    icon: 'error',
                    title: '获取课件失败',
                    timer: 1500,
                    text: error,
                });
            });
            homeworksPromise.then((homeworks) => {
                homeworkStore.setHomeworks(homeworks);
            }).catch((error) => {
                Swal.fire({
                    icon: 'error',
                    title: '获取作业失败',
                    timer: 1500,
                    text: error,
                });
            });
        }).catch((error) => {
            Swal.fire({
                icon: 'error',
                title: '获取课程信息失败',
                timer: 1500,
                text: error,
            });
        });
        testsPromise.then((tests) => {
            testStore.setTests(tests);
        }).catch((error) => {
            Swal.fire({
                icon: 'error',
                title: '获取考试信息失败',
                timer: 1500,
                text: error,
            });
        });
    }).catch((error) => {
        Swal.fire({
            icon: 'error',
            title: '获取学期信息失败',
            timer: 1500,
            text: error,
        });
    });
    gradesPromise.then(([grades, analysis]) => {
        gradeStore.setGrades(grades);
        gradeStore.setAnalysis(analysis);
    }).catch((error) => {
        Swal.fire({
            icon: 'error',
            title: '获取成绩信息失败',
            timer: 1500,
            text: error,
        });
    });
}
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