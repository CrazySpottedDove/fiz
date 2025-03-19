<template>
    <span :class="[
        'px-4 py-2 font-bold text-lg cursor-pointer',
        isActive
            ? 'dark:text-blue-400 text-blue-600 border-b-2 border-blue-400 dark:border-blue-600'
            : 'dark:text-zinc-300 text-zinc-600',
        addClass
    ]" :aria-current="isActive ? 'page' : undefined" v-bind="props" :aria-label="`Go to ${text} page`"
        @click="handleChangeRoute">
        {{ text }}
    </span>
</template>
<script setup>
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { toRefs } from 'vue';
import router from '../router';
import Swal from 'sweetalert2';
import { useStateStore } from '../stores';
import { refresh } from '../logic/refresh';
const stateStore = useStateStore();
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
async function handleChangeRoute(){
    if (text.value === "登出") {
        Swal.fire({
            title: '确认登出?',
            icon: 'warning',
            showCancelButton: true,
            confirmButtonText: '确认',
            cancelButtonText: '取消'
        }).then((result) => {
            if (result.isConfirmed) {
                // 执行登出操作
                stateStore.setLogin(false);
                router.push(href.value);
            }
        });
    }else if (text.value === "刷新"){
        refresh();
        Swal.fire({
            title: `Launch: 刷新`,
            icon: 'success',
            showConfirmButton: false,
            position: 'top',
            timer: 1000,
            toast: true,
        });
    }
     else {
        router.push(href.value);
    }
}
</script>