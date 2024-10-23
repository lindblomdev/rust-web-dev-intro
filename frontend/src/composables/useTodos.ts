import { computed, onMounted, ref } from "vue";
import useAuth from "./useAuth";
import { apiBaseUrl } from "../constants";

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

const endpoint = (id?: number) =>
  `${apiBaseUrl}/todos${typeof id === "number" ? `/${id}` : ""}`;
const request = (method: string, body?: any) => ({
  method,
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify(body),
});

function makeAuthenticatedCall(
  token: String | null,
  request: RequestInfo,
  init?: RequestInit,
) {
  let headers = init?.headers || {};

  if (token) {
    headers = { ...headers, Authorization: `Bearer ${token}` };
  }

  return fetch(request, {
    ...init,
    headers,
  });
}

export default function useTodos() {
  const { token } = useAuth();
  const todos = ref<Todo[]>([]);

  async function getTodos() {
    const response = await makeAuthenticatedCall(token.value, endpoint());
    const json = await response.json();
    todos.value = json;
  }

  async function toggleCompleted(todo: Todo) {
    todo.completed = !todo.completed;
    const response = await makeAuthenticatedCall(
      token.value,
      endpoint(todo.id),
      request("PATCH", todo),
    );
    const json = await response.json();
    todos.value = json;
  }

  async function addTodo(newTodoTitle: string) {
    if (newTodoTitle === "") {
      return;
    }

    const response = await makeAuthenticatedCall(
      token.value,
      endpoint(),
      request("POST", {
        title: newTodoTitle,
      }),
    );

    const json = await response.json();
    todos.value = json;
  }

  async function deleteTodo(todo: Todo) {
    const response = await makeAuthenticatedCall(
      token.value,
      endpoint(todo.id),
      {
        method: "DELETE",
      },
    );
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
  };
}
