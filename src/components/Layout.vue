<script setup>
import Navigator from './Navigator.vue';
</script>
<template>
    <body
        class="dark:bg-zinc-900 bg-zinc-100 antialiased selection:bg-blue-300 selection:text-zinc-900 dark:selection:bg-blue-600 dark:selection:text-white 2xl:text-lg">
        <div :style="{
            background: 'linear-gradient(45deg, rgba(96, 122, 250, 0) 20.79%, rgba(96, 122, 250, 0.09) 40.92%, rgba(255,255,255, 0) 70.35%)'
        }" class="fixed top-0 left-0 w-full h-full pointer-events-none -z-1">
        </div>
        <Navigator />
        <main class="px-6 lg:px-0 mx-auto max-w-6xl flex flex-col min-h-[96vh] py-6 dark:text-zinc-300 text-zinc-700"
            id="main-content">
            <slot />
        </main>
    </body>
</template>


