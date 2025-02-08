<script setup lang="ts">
import { onMounted, ref } from "vue";
import TodoItem from "./components/TodoItem.vue";
import { invoke } from "@tauri-apps/api/core";

const todos = ref<{ title: string; completed: boolean }[]>([]);
const newTodoTitle = ref("");

async function fetchTodos() {
  todos.value = await invoke('get_todos');
}

async function addTodo() {
  if (newTodoTitle.value.trim()) {
    await invoke('add_todo', { title: newTodoTitle.value });
    newTodoTitle.value = "";
    fetchTodos();
  }
}

async function updateTodoCompleted(index: number, completed: boolean) {
  // message.value = `${index}: ${completed}`;
  await invoke("toggle_todo", { index, completed });
  fetchTodos();
}

async function removeTodoItem(index: number) {
  await invoke("delete_todo", { index });
  fetchTodos();
}

onMounted(fetchTodos);

// const message = ref("Hello, Vue 3 + Vite + Tauri!");
</script>

<template>
  <main class="container">
    <h1>Todo List</h1>

    <form class="row" @submit.prevent="addTodo">
      <input v-model="newTodoTitle" placeholder="Add a new todo..." />
      <button type="submit">Add</button>
    </form>

    <!-- <div>{{ todos }}</div>
    <div>{{ message }}</div> -->

    <div class="todo-list">
      <TodoItem
        v-for="(todo, index) in todos"
        :key="index"
        :title="todo.title"
        :completed="todo.completed"
        @update:completed="updateTodoCompleted(index, $event)"
        @delete="removeTodoItem(index)"
      />
    </div>
  </main>
</template>

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

input {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  margin-right: 5px;
}

button {
  border-radius: 8px;
  padding: 0.6em 1.2em;
  cursor: pointer;
}
</style>