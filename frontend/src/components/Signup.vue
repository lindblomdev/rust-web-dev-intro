<script setup lang="ts">
import { ref } from "vue";
import useAuth from "../composables/useAuth";

const { signup } = useAuth();
const username = ref("");
const password = ref("");

async function trySignup() {
    try {
        await signup(username.value, password.value);
    } catch (e) {
        alert(e);
    }
}
</script>

<template>
    <h2>Signup</h2>
    <label>
        Username
        <input type="text" v-model="username" />
    </label>
    <label>
        Password
        <input type="password" v-model="password" />
    </label>
    <button @click="trySignup()">Log in</button>
</template>
