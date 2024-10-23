import { computed, readonly, ref } from "vue";
import { apiBaseUrl } from "../constants";

const token = ref(localStorage.getItem("token"));
const isTokenValid = computed(() => {
  if (!token.value) return false;

  return !!token.value;
});

export default function useAuth() {
  async function login(username: string, password: string) {
    const newToken = await fetch(apiBaseUrl + "/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, password }),
    }).then((res) => res.text());

    if (newToken) {
      localStorage.setItem("token", newToken);
      token.value = newToken;
    } else {
      throw new Error("Invalid credentials");
    }
  }

  async function signup(username: string, password: string) {
    const newToken = await fetch(apiBaseUrl + "/signup", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, password }),
    }).then((res) => res.text());

    if (newToken) {
      localStorage.setItem("token", newToken);
      token.value = newToken;
    } else {
      throw new Error("Invalid credentials");
    }
  }

  function logout() {
    localStorage.removeItem("token");
    token.value = null;
  }

  return { signup, logout, isTokenValid, login, token: readonly(token) };
}
