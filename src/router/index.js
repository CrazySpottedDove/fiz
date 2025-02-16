import { createRouter, createWebHistory } from 'vue-router';
import LoginPage from '../views/LoginPage.vue';
import ToDo from '../views/ToDo.vue';
import Courseware from '../views/Courseware.vue';
import Grade from '../views/Grade.vue';
const routes = [
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
        path:'/courseware',
        name:"Courseware",
        component:Courseware
    },
    {
        path:'/grade',
        name:'Grade',
        component:Grade
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;