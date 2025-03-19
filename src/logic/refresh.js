import { invoke } from '@tauri-apps/api/core';
import { useMaterialStore, useCourseStore, useGradeStore, useHomeworkStore, useTestStore} from '../stores';
import Swal from 'sweetalert2';
function showError(title, error) {
    Swal.fire({
        icon: 'error',
        title: title,
        timer: 1500,
        text: error,
    });
}
export function refresh() {
    const courseStore = useCourseStore();
    const gradeStore = useGradeStore();
    const materialStore = useMaterialStore();
    const homeworkStore = useHomeworkStore();
    const testStore = useTestStore();
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