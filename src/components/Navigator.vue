<script setup>
import navigation from "../data/navigation";
import Link from "./Link.vue";
import icon from "../assets/fiz.ico"
</script>
<template>
    <nav class="sticky top-0 z-10 lg:backdrop-blur-lg bg-zinc-900 bg-opacity-90 lg:bg-opacity-50 py-4 px-6 lg:px-0">
        <ul class="flex justify-center items-center space-x-4 text-sm">
            <li v-for="item in navigation" :key="item.href">
                <Link :text="item.text" :href="item.href">
                </Link>
            </li>
        </ul>
    </nav>
</template>