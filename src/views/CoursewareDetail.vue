<template>
    <Layout>
        <h1 class="text-cyan-500 text-center text-xl font-bold">{{ name }}</h1>
        <br>
        <ul>
            <MaterialCard v-for="material in sortedMaterials" :key="material.id" :materialName="material.title"
                :uploads="material.uploads" :title="name" />
        </ul>
    </Layout>
</template>

<script setup>
import { computed, onMounted, ref, watchEffect } from 'vue';
import { useRoute } from 'vue-router';
import { useConfigStore, useMaterialStore } from '../stores';
import Layout from '../components/Layout.vue';
import MaterialCard from '../components/MaterialCard.vue';

const route = useRoute();
const materialStore = useMaterialStore();
const materials = ref([]);
const name = ref('');
const configStore = useConfigStore();
onMounted(() => {
    const courseId = parseInt(route.params.id, 10);
    name.value = route.query.name;
    // 从全局的课程 store 中查找课程
    watchEffect(() => {
        materials.value = materialStore.materials[courseId] || [];
    });
});
const sortedMaterials = computed(() => {
    configStore.config.material_rev ? materials.value.slice().reverse() : materials.value
    console.log(configStore.config.material_rev)
})
</script>