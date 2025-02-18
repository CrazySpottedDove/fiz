<template>
    <Layout>
        <h1 class="text-cyan-500 text-center text-xl font-bold">{{ name }}</h1>
        <br>
        <ul>
            <MaterialCard v-for="material in materials" :key="material.id" :materialName="material.title"
                :uploads="material.uploads" :title="name" />
        </ul>
    </Layout>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { useMaterialStore } from '../stores';
import Layout from '../components/Layout.vue';
import MaterialCard from '../components/MaterialCard.vue';

const route = useRoute();
const materialStore = useMaterialStore();
const materials = ref([]);
const name = ref('');
onMounted(() => {
    const courseId = parseInt(route.params.id, 10);
    name.value = route.query.name;
    // 从全局的课程 store 中查找课程
    materials.value = materialStore.materials[courseId];
});
</script>