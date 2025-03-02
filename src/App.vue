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
listen("watches-inited", (event) => {
    courseStore.setWatches(event.payload);
})
function showError(title, error) {
    Swal.fire({
        icon: 'error',
        title: title,
        timer: 1500,
        text: error,
    });
}
function refresh() {
    const etaPromise = invoke("login_eta");
    etaPromise.then(() => {
        const gradesPromise = invoke("get_grades_and_analysis");
        gradesPromise.then(([grades, analysis]) => {
            gradeStore.setGrades(grades);
            gradeStore.setAnalysis(analysis);
        }).catch((error) => {
            showError('获取成绩信息失败', error);
        });
    }).catch((error) => {
        showError('登录 ETA 失败', error);
    });
    const semesterPromise = invoke("get_semesters");
    semesterPromise.then(() => {
        const zdbkPromise = invoke("login_zdbk");
        zdbkPromise.then(() => {
            const testsPromise = invoke("get_tests");
            testsPromise.then((tests) => {
                testStore.setTests(tests);
            }).catch((error) => {
                showError('获取考试信息失败', error);
            });
        }).catch((error) => {
            showError('登录教务网失败', error);
        })

        const coursesPromise = invoke("get_courses");
        coursesPromise.then((courses) => {
            courseStore.setCourses(courses);
            const materialsPromise = invoke("get_materials");
            const homeworksPromise = invoke("get_homeworks");
            materialsPromise.then((materials) => {
                materialStore.setMaterials(materials);
                const watchesPromise = invoke("get_watches");
                watchesPromise.catch((error) => {
                    showError('下载课程课件失败', error);
                });
            }).catch((error) => {
                showError('获取课件失败', error);
            });
            homeworksPromise.then((homeworks) => {
                homeworkStore.setHomeworks(homeworks);
            }).catch((error) => {
                showError('获取作业信息失败', error);
            });
        }).catch((error) => {
            showError('获取课程信息失败', error);
        });

    }).catch((error) => {
        showError('获取学期信息失败', error);
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