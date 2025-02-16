<template>
    <Layout>
        <!-- <ul>
            <CourseCard v-for="course in courses" :key="course.id" class="dark:text-indigo-100" :name="course.name"
                :image="course.image">
            </CourseCard>
        </ul> -->
    </Layout>
</template>
<script setup>
import { inject } from "vue";
import Layout from "../components/Layout.vue";
import CourseCard from "../components/CourseCard.vue";
// const courses = inject('courses');
// console.log(courses.value)

</script>