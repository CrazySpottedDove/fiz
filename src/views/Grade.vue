<template>
    <Layout>
        <div class="text-center mt-4 fixed left-12">
            <button class="px-4 py-2 bg-cyan-700 text-zinc-200 font-bold rounded hover:bg-blue-700 w-20"
                @click="changeShowGrade">
                {{ showGrade ? 'Less' : 'More' }}
            </button>
        </div>
        <ul class="grid grid-cols-2 gap-4">
            <AnalysisCard v-for="analysis in gradeStore.analysis" :key="analysis.xn + analysis.xq"
                :name="`${analysis.xn} ${analysis.xq}`" :gpa="analysis.gpa" :credit="analysis.credit" />
        </ul>
        <br>
        <br>
        <ul class="grid grid-cols-2 gap-4" v-if="showGrade">
            <GradeCard v-for="grade in gradeStore.grades" :key="grade.kch" :name="grade.name" :grade="grade.grade"
                :gpa="grade.gpa" :credit="grade.credit" />
        </ul>
    </Layout>
</template>
<script setup>
import { onMounted, ref, watch } from "vue";
import Layout from "../components/Layout.vue";
import { useConfigStore, useGradeStore } from "../stores";
import AnalysisCard from "../components/AnalysisCard.vue";
import GradeCard from "../components/GradeCard.vue";
const gradeStore = useGradeStore();
const showGrade = ref(false);
const configStore = useConfigStore();
onMounted(() => {
    watch(() => configStore.config.show_grade, (new_show_grade) => {
        showGrade.value = new_show_grade;
    }
        , { immediate: true })
})
function changeShowGrade() {
    showGrade.value = !showGrade.value
}
</script>