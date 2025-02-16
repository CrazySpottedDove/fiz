<template>
    <span :class="[
        'px-4 py-2 font-bold text-lg cursor-pointer',
        isActive
            ? 'dark:text-blue-400 text-blue-600 border-b-2 border-blue-400 dark:border-blue-600'
            : 'dark:text-zinc-300 text-zinc-600',
        addClass
    ]" :aria-current="isActive ? 'page' : undefined" v-bind="props" :aria-label="`Go to ${text} page`" @click="() => $router.push(href)">
        {{ text }}
    </span>
</template>

<script setup>
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { toRefs } from 'vue';

const props = defineProps({
    text: String,
    addClass: String,
    href: String,
});

const { text, addClass, href, ...restProps } = toRefs(props);
const route = useRoute();

const isActive = computed(() => {
    if (!href.value) {
        return false;
    } else if (href.value === '/') {
        return href.value === route.path;
    } else {
        return route.path.includes(href.value);
    }
});

</script>