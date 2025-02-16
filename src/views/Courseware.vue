<template>
    <Layout>
        <div class="text-center mt-4 fixed left-12">
            <button class="px-4 py-2 bg-cyan-700 text-zinc-200 font-bold rounded hover:bg-blue-700 w-20"
                @click="toggleInactive">
                {{ showInactive ? 'Less' : 'More' }}
            </button>
        </div>
        <ul class="grid grid-cols-2 gap-4">
            <!-- 总是显示激活课程 -->
            <CourseCard v-if="noActive" class="dark:text-indigo-100" :name="'无活跃课程'" />
            <CourseCard v-for="course in activeCourses" :key="course.id" class="dark:text-indigo-100"
                :name="course.name" :image="course.cover ? `assets://${course.id}.jpg` : null" />
            <!-- 根据开关显示非激活课程 -->
            <CourseCard v-if="showInactive" v-for="course in inactiveCourses" :key="course.id"
                class="dark:text-indigo-100" :name="course.name"
                :image="course.cover ? `assets://${course.id}.jpg` : null" />
        </ul>

    </Layout>
</template>

<script setup>
import { ref, computed } from "vue";
import Layout from "../components/Layout.vue";
import CourseCard from "../components/CourseCard.vue";
import { useCourseStore } from "../stores";

const courseStore = useCourseStore();
const courses = courseStore.courses;
// 定义是否显示非激活课程的开关
const showInactive = ref(false);

// 计算属性过滤激活与非激活的课程
const activeCourses = computed(() => {
    return courses.filter(course => course.is_active);
});

const inactiveCourses = computed(() => {
    return courses.filter(course => !course.is_active);
});

const noActive = computed(() => activeCourses.value.length === 0);
// 切换显示状态
const toggleInactive = () => {
    showInactive.value = !showInactive.value;
};
</script>