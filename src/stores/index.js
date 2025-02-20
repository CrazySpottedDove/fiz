import { defineStore } from "pinia";

export const useCourseStore = defineStore('course', {
    state: () => {
        return {
            courses: [],
        }
    },
    actions: {
        setCourses(courses) {
            this.courses = courses;
        }
    }
})

export const useGradeStore = defineStore('grade', {
    state: () => {
        return {
            grades: [],
            analysis: [],
        }
    },
    actions: {
        setGrades(grades) {
            this.grades = grades;
        },
        setAnalysis(analysis) {
            this.analysis = analysis;
        }
    }
})

export const useMaterialStore = defineStore('material', {
    state: () => {
        return {
            materials: {}
        }
    },
    actions: {
        setMaterials(materials) {
            this.materials = materials;
        }
    }
})

export const useConfigStore = defineStore('config', {
    state: () => {
        return {
            config: {}
        }
    },
    actions: {
        setConfig(config) {
            this.config = config;
        }
    }
})

export const useHomeworkStore = defineStore('homework', {
    state: () => {
        return {
            homeworks: []
        }
    },
    actions: {
        setHomeworks(homeworks) {
            this.homeworks = homeworks;
        }
    }
})

export const useTestStore = defineStore('test', {
    state: () => {
        return {
            tests: []
        }
    },
    actions: {
        setTests(tests) {
            this.tests = tests;
        }
    }
})

export const useStateStore = defineStore('state', {
    state: () => {
        return {
            login: false
        }
    },
    actions: {
        setLogin(login) {
            this.login = login;
        }
    }
})
