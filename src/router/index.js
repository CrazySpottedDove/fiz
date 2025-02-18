import { createRouter, createWebHistory } from 'vue-router';
import LoginPage from '../views/LoginPage.vue';
import ToDo from '../views/ToDo.vue';
import Courseware from '../views/Courseware.vue';
import Grade from '../views/Grade.vue';
import CoursewareDetail from '../views/CoursewareDetail.vue';
import Setting from '../views/Setting.vue';
import Init from '../views/Init.vue';
const routes = [
    {
        path:'/',
        name:'Init',
        component: Init
    },
    {
        path: '/login',
        name: 'Login',
        component: LoginPage
    },
    {
        path: '/todo',
        name: 'ToDo',
        component: ToDo
    },
    {
        path: '/courseware',
        name: "Courseware",
        component: Courseware
    },
    {
        path: '/grade',
        name: 'Grade',
        component: Grade
    },
    {
        path: '/courseware/:id',
        name: 'CoursewareDetail',
        component: CoursewareDetail,
        props: true
    },
    {
        path: '/setting',
        name: 'Setting',
        component: Setting
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;