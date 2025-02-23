<template>
    <LoginLayout>
        <div class="flex flex-col items-center justify-center h-screen" v-if="loging">
            <h1 class="text-4xl font-bold mb-8 dark:text-blue-500">学在浙大在召唤……</h1>
        </div>
        <div class="flex flex-col items-center justify-center h-screen">
            <h1 class="text-4xl font-bold mb-8 dark:text-blue-500">登录</h1>
            <form @submit.prevent="relogin" class="flex flex-col space-y-6 w-full max-w-md">
                <input v-model="stuid" placeholder="学号"
                    class="px-4 py-2 border border-gray-300 rounded-md dark:bg-zinc-300 dark:text-zinc-700 dark:placeholder-zinc-500 w-full" />
                <div class="relative w-full">
                    <input :type="showPassword ? 'text' : 'password'" v-model="password" placeholder="密码"
                        class="px-4 py-2 border border-gray-300 rounded-md dark:bg-zinc-300 dark:text-zinc-700 dark:placeholder-zinc-500 w-full" />
                    <button type="button" @click="togglePasswordVisibility"
                        class="absolute inset-y-0 right-0 px-3 py-2 text-gray-600 dark:text-gray-500">
                        {{ showPassword ? '隐藏' : '显示' }}
                    </button>
                </div>
                <button type="submit"
                    class="px-4 py-2 dark:bg-blue-900 dark:text-zinc-200 text-lg rounded-md w-full">登录</button>
            </form>
        </div>
    </LoginLayout>
</template>

<script setup>
import LoginLayout from '../components/LoginLayout.vue';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
const stuid = ref('')
const password = ref('')
const showPassword = ref(false);
const router = useRouter()
const loging = ref(false)

const relogin = async () => {
    try {
        loging.value = true;
        await invoke('relogin', { stuid: stuid.value, password: password.value })
        router.push('/todo');
    } catch (error) {
        window.alert(`登录失败：${error}`)
    } finally {
        loging.value = false;
    }
}
const togglePasswordVisibility = () => {
    showPassword.value = !showPassword.value;
};
</script>

<style scoped>
/* 增加 max-width 和 width */
form {
    max-width: 600px;
    /* 设置表单的最大宽度 */
    width: 100%;
    /* 设置表单的宽度为 100% */
}
</style>