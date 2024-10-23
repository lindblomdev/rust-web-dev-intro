<script setup lang="ts">
import Todos from "./components/Todos.vue";
import Signup from "./components/Signup.vue";
import Login from "./components/Login.vue";
import useAuth from "./composables/useAuth";
const { isTokenValid } = useAuth();
</script>

<template>
    <h1>Rust by web</h1>
    <Todos v-if="isTokenValid"></Todos>
    <template v-else>
        <Login></Login>
        <Signup></Signup>
    </template>
</template>
