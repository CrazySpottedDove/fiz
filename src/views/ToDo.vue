<template>
    <Layout>
        <HomeworkCard v-for="homework in homeworkStore.homeworks" :homework="homework" :key="homework.id" />
    </Layout>
</template>
<script setup>
import HomeworkCard from "../components/HomeworkCard.vue";
import Layout from "../components/Layout.vue";
import { useHomeworkStore } from "../stores";
const homeworkStore = useHomeworkStore();

</script>