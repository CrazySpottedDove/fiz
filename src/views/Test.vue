<script setup>
import Layout from '../components/Layout.vue';
import TestCard from '../components/TestCard.vue';
import { useTestStore } from '../stores';
const testStore = useTestStore();
</script>

<template>
    <Layout>
        <ul>
            <TestCard v-for="test in testStore.tests" :name="test.name" :time="test.time" :sit="test.sit" :room="test.room"/>
        </ul>
    </Layout>
</template>