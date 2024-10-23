<script setup lang="ts">
import { ref } from "vue";
import useTodos from "../composables/useTodos";
import useAuth from "../composables/useAuth";

const { addTodo, deleteTodo, todos, toggleCompleted } = useTodos();
const { logout } = useAuth();
const newTodoTitle = ref("");

function onInput(e: KeyboardEvent) {
    if (e.key === "Enter") {
        addAndClear();
    }
}

async function addAndClear() {
    await addTodo(newTodoTitle.value);
    newTodoTitle.value = "";
}
</script>

<template>
    <h2>Todo</h2>
    <ul class="todo-list">
        <li v-for="todo in todos" :key="todo.id">
            <label>
                <input
                    type="checkbox"
                    :checked="todo.completed"
                    @change="toggleCompleted(todo)"
                />
                {{ todo.title }} {{ todo.completed ? "✅" : "" }}
            </label>
            <button @click="deleteTodo(todo)">Delete</button>
        </li>
    </ul>
    <input type="text" v-model="newTodoTitle" @keypress="onInput" />
    <button @click="addAndClear()">Add todo</button>
    <hr />
    <button @click="logout()">Log out</button>
</template>

<style>
.todo-list {
    list-style: none;
    padding: 0;
    text-align: left;
}

li {
    display: flex;
    justify-content: space-between;
}
</style>
