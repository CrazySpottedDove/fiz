import { defineStore } from "pinia";
export const useCourseStore = defineStore('course', {
    state: () => {
        return {
            courses: [],
        }
    }
})

export const useGradeStore = defineStore('grade', {
    state: () => {
        return {
            grades: [],
            analysis: [],
        }
    }
})

export const useMaterialStore = defineStore('material', {
    state: () => {
        return {
            materials: {}
        }
    }
})

export const useConfigStore = defineStore('config',{
    state:()=>{
        return {
            config:{}
        }
    }
})