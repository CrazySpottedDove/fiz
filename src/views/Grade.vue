<template>
    <Layout>
        <ul class="grid grid-cols-2 gap-4">
            <AnalysisCard v-for="analysis in gradeStore.analysis" :key="analysis.xn+analysis.xq"
                :name="`${analysis.xn} ${analysis.xq}`" :gpa="analysis.gpa"
                :credit="analysis.credit" />
        </ul>
        <br>
        <br>
        <ul class="grid grid-cols-2 gap-4">
            <GradeCard v-for="grade in gradeStore.grades" :key="grade.kch"
                :name="grade.name" :grade="grade.grade" :gpa="grade.gpa" :credit="grade.credit" />
        </ul>
    </Layout>
</template>
<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Layout from "../components/Layout.vue";
import { useGradeStore } from "../stores";
import AnalysisCard from "../components/AnalysisCard.vue";
import GradeCard from "../components/GradeCard.vue";
const gradeStore = useGradeStore();

</script>