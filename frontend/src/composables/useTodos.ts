import { computed, onMounted, ref } from "vue";

interface Todo {
    id: number;
    title: string;
    completed: boolean;
}

const endpoint = (id?: number) => `http://localhost:3030/todos${typeof id === 'number' ? `/${id}` : ''}`;
const request = (method: string, body?: any) => ({
    method,
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
});

export default function useTodos() {
    const todos = ref<Todo[]>([]);

    async function getTodos() {
        const response = await fetch(endpoint());
        const json = await response.json();
        todos.value = json;
    }

    async function toggleCompleted(todo: Todo) {
        todo.completed = !todo.completed;
        const response = await fetch(endpoint(todo.id), request('PATCH', todo));
        const json = await response.json();
        todos.value = json;
    }

    async function addTodo(newTodoTitle: string) {
        if (newTodoTitle === '') {
            return;
        }

        const response = await fetch(endpoint(), request('POST', {
            title: newTodoTitle,
        }));

        const json = await response.json();
        todos.value = json;
    }

    async function deleteTodo(todo: Todo) {
        const response = await fetch(endpoint(todo.id), {
            method: 'DELETE'
        });
        const json = await response.json();
        todos.value = json;
    }

    onMounted(() => {
        getTodos();
    });

    return {
        todos: computed(() => todos.value),
        addTodo,
        deleteTodo,
        toggleCompleted,
    }
}
