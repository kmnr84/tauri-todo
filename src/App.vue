<script setup lang="ts">
import { ref } from "vue";
import TodoItem from "./components/TodoItem.vue";

const todos = ref<{ title: string; completed: boolean }[]>([]);
const newTodoTitle = ref("");

function addTodo() {
  if (newTodoTitle.value.trim()) {
    todos.value.push({ title: newTodoTitle.value, completed: false });
    newTodoTitle.value = "";
  }
}
</script>

<template>
  <main class="container">
    <h1>Todo List</h1>

    <form class="row" @submit.prevent="addTodo">
      <input v-model="newTodoTitle" placeholder="Add a new todo..." />
      <button type="submit">Add</button>
    </form>

    <div class="todo-list">
      <TodoItem
        v-for="(todo, index) in todos"
        :key="index"
        :title="todo.title"
        v-model:completed="todo.completed"
        @delete="todos.splice(index, 1)"
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