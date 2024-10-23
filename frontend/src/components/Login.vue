<script setup lang="ts">
import { ref } from "vue";
import useAuth from "../composables/useAuth";

const { login } = useAuth();
const username = ref("");
const password = ref("");

async function tryLogin() {
    try {
        await login(username.value, password.value);
    } catch (e) {
        alert(e);
    }
}
</script>

<template>
    <h2>Login</h2>
    <label>
        Username
        <input type="text" v-model="username" />
    </label>
    <label>
        Password
        <input type="password" v-model="password" />
    </label>
    <button @click="tryLogin()">Log in</button>
</template>
